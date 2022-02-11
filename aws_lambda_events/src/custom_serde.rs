use base64::{decode, encode};
use chrono::{DateTime, Duration, TimeZone, Utc};
use http::{HeaderMap, Method};
use serde;
use serde::de::{Deserialize, Deserializer, Error as DeError, Unexpected, Visitor};
use serde::ser::{Error as SerError, SerializeMap, Serializer};
use std::collections::HashMap;
use std::fmt;

fn normalize_timestamp<'de, D>(deserializer: D) -> Result<(u64, u64), D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNumber {
        String(String),
        Float(f64),
        Int(u64),
    }

    let input: f64 = match StringOrNumber::deserialize(deserializer)? {
        StringOrNumber::String(s) => s.parse::<f64>().map_err(DeError::custom)?,
        StringOrNumber::Float(f) => f,
        StringOrNumber::Int(i) => i as f64,
    };

    // We need to do this due to floating point issues.
    let input_as_string = format!("{}", input);
    let parts: Result<Vec<u64>, _> = input_as_string
        .split('.')
        .map(|x| x.parse::<u64>().map_err(DeError::custom))
        .collect();
    let parts = parts?;
    if parts.len() > 1 {
        Ok((parts[0], parts[1]))
    } else {
        Ok((parts[0], 0))
    }
}

pub(crate) fn serialize_milliseconds<S>(
    date: &DateTime<Utc>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let ts_with_millis = date.timestamp() * 1000
        + date.timestamp_subsec_millis() as i64 * 10
        + date.timestamp_subsec_nanos() as i64;

    serializer.serialize_str(&ts_with_millis.to_string())
}

pub(crate) fn deserialize_milliseconds<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let (whole, frac) = normalize_timestamp(deserializer)?;
    assert_eq!(frac, 0);
    let seconds: f64 = (whole / 1000) as f64;
    let milliseconds: u32 = (seconds.fract() * 1000f64) as u32;
    let nanos = milliseconds * 1_000_000;
    Ok(Utc.timestamp(seconds as i64, nanos as u32))
}

pub(crate) fn serialize_seconds<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let seconds = date.timestamp();
    let milliseconds = date.timestamp_subsec_millis();
    let combined = format!("{}.{}", seconds, milliseconds);
    if milliseconds > 0 {
        serializer.serialize_str(&combined)
    } else {
        serializer.serialize_str(&seconds.to_string())
    }
}

#[allow(dead_code)]
pub(crate) fn deserialize_seconds<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let (whole, frac) = normalize_timestamp(deserializer)?;
    let seconds = whole;
    let nanos = frac * 1_000_000;
    Ok(Utc.timestamp(seconds as i64, nanos as u32))
}

pub(crate) fn deserialize_base64<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    decode(&s).map_err(DeError::custom)
}

pub(crate) fn serialize_base64<S>(value: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&encode(value))
}

/// Deserializes `Option<String>`, mapping JSON `null` or the empty string `""` to `None`.
#[cfg(not(feature = "string-null-empty"))]
pub(crate) fn deserialize_lambda_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    match Option::deserialize(deserializer)? {
        Some(s) =>
        {
            #[allow(clippy::comparison_to_empty)]
            if s == "" {
                Ok(None)
            } else {
                Ok(Some(s))
            }
        }
        None => Ok(None),
    }
}

/// Deserializes `HashMap<_>`, mapping JSON `null` to an empty map.
pub(crate) fn deserialize_lambda_map<'de, D, K, V>(
    deserializer: D,
) -> Result<HashMap<K, V>, D::Error>
where
    D: Deserializer<'de>,
    K: serde::Deserialize<'de>,
    K: std::hash::Hash,
    K: std::cmp::Eq,
    V: serde::Deserialize<'de>,
{
    // https://github.com/serde-rs/serde/issues/1098
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

pub(crate) fn serialize_duration_seconds<S>(
    duration: &Duration,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let seconds = duration.num_seconds();

    serializer.serialize_i64(seconds)
}

pub(crate) fn deserialize_duration_seconds<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let seconds = f64::deserialize(deserializer)?;
    Ok(Duration::seconds(seconds as i64))
}

pub(crate) fn serialize_duration_minutes<S>(
    duration: &Duration,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let minutes = duration.num_minutes();

    serializer.serialize_i64(minutes)
}

pub(crate) fn deserialize_duration_minutes<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let minutes = f64::deserialize(deserializer)?;
    Ok(Duration::minutes(minutes as i64))
}

/// Serialize a http::HeaderMap into a serde str => Vec<str> map
pub(crate) fn serialize_multi_value_headers<S>(
    headers: &HeaderMap,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map = serializer.serialize_map(Some(headers.keys_len()))?;
    for key in headers.keys() {
        let mut map_values = Vec::new();
        for value in headers.get_all(key) {
            map_values.push(value.to_str().map_err(S::Error::custom)?)
        }
        map.serialize_entry(key.as_str(), &map_values)?;
    }
    map.end()
}

/// Serialize a http::HeaderMap into a serde str => str map
pub(crate) fn serialize_headers<S>(headers: &HeaderMap, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map = serializer.serialize_map(Some(headers.keys_len()))?;
    for key in headers.keys() {
        let map_value = headers[key].to_str().map_err(S::Error::custom)?;
        map.serialize_entry(key.as_str(), map_value)?;
    }
    map.end()
}

pub mod http_method {
    use super::*;

    pub fn serialize<S: Serializer>(method: &Method, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_str(method.as_str())
    }

    struct MethodVisitor;
    impl<'de> Visitor<'de> for MethodVisitor {
        type Value = Method;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "valid method name")
        }

        fn visit_str<E: DeError>(self, val: &str) -> Result<Self::Value, E> {
            if val.is_empty() {
                Ok(Method::GET)
            } else {
                val.parse()
                    .map_err(|_| DeError::invalid_value(Unexpected::Str(val), &self))
            }
        }
    }

    pub fn deserialize<'de, D>(de: D) -> Result<Method, D::Error>
    where
        D: Deserializer<'de>,
    {
        de.deserialize_str(MethodVisitor)
    }

    pub fn deserialize_optional<'de, D>(deserializer: D) -> Result<Option<Method>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<&str> = Option::deserialize(deserializer)?;
        if let Some(val) = s {
            let visitor = MethodVisitor {};
            return visitor.visit_str(val).map(Some);
        }

        Ok(None)
    }

    pub fn serialize_optional<S: Serializer>(
        method: &Option<Method>,
        ser: S,
    ) -> Result<S::Ok, S::Error> {
        if let Some(method) = method {
            return serialize(method, ser);
        }

        ser.serialize_none()
    }
}

// Jan 2, 2006 3:04:05 PM
const CODEBUILD_TIME_FORMAT: &str = "%b %e, %Y %l:%M:%S %p";

pub mod codebuild_time {
    use super::*;
    struct TimeVisitor;
    impl<'de> Visitor<'de> for TimeVisitor {
        type Value = DateTime<Utc>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "valid codebuild time: {}", CODEBUILD_TIME_FORMAT)
        }

        fn visit_str<E: DeError>(self, val: &str) -> Result<Self::Value, E> {
            Utc.datetime_from_str(val, CODEBUILD_TIME_FORMAT)
                .map_err(|e| DeError::custom(format!("Parse error {} for {}", e, val)))
        }
    }

    pub mod str_time {
        use super::*;

        pub(crate) fn deserialize<'de, D>(d: D) -> Result<DateTime<Utc>, D::Error>
        where
            D: Deserializer<'de>,
        {
            d.deserialize_str(TimeVisitor)
        }

        pub fn serialize<S: Serializer>(date: &DateTime<Utc>, ser: S) -> Result<S::Ok, S::Error> {
            let s = format!("{}", date.format(CODEBUILD_TIME_FORMAT));
            ser.serialize_str(&s)
        }
    }

    pub mod optional_time {
        use super::*;

        pub(crate) fn deserialize<'de, D>(
            deserializer: D,
        ) -> Result<Option<DateTime<Utc>>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s: Option<String> = Option::deserialize(deserializer)?;
            if let Some(val) = s {
                let visitor = TimeVisitor {};
                return visitor.visit_str(&val).map(Some);
            }

            Ok(None)
        }

        pub fn serialize<S: Serializer>(
            date: &Option<DateTime<Utc>>,
            ser: S,
        ) -> Result<S::Ok, S::Error> {
            if let Some(date) = date {
                return str_time::serialize(date, ser);
            }

            ser.serialize_none()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::TimeZone;
    use serde_json;

    #[test]
    fn test_deserialize_base64() {
        #[derive(Deserialize)]
        struct Test {
            #[serde(deserialize_with = "deserialize_base64")]
            v: Vec<u8>,
        }
        let data = json!({
            "v": "SGVsbG8gV29ybGQ=",
        });
        let decoded: Test = serde_json::from_value(data).unwrap();
        assert_eq!(
            String::from_utf8(decoded.v).unwrap(),
            "Hello World".to_string()
        );
    }

    #[test]
    fn test_serialize_base64() {
        #[derive(Serialize)]
        struct Test {
            #[serde(serialize_with = "serialize_base64")]
            v: Vec<u8>,
        }
        let instance = Test {
            v: "Hello World".as_bytes().to_vec(),
        };
        let encoded = serde_json::to_string(&instance).unwrap();
        assert_eq!(encoded, r#"{"v":"SGVsbG8gV29ybGQ="}"#.to_string());
    }

    #[test]
    fn test_deserialize_milliseconds() {
        #[derive(Deserialize)]
        struct Test {
            #[serde(deserialize_with = "deserialize_milliseconds")]
            v: DateTime<Utc>,
        }
        let expected = Utc.ymd(2017, 10, 05).and_hms_nano(15, 33, 44, 0);

        // Test parsing strings.
        let data = json!({
            "v": "1507217624302",
        });
        let decoded: Test = serde_json::from_value(data).unwrap();
        assert_eq!(expected, decoded.v,);
        // Test parsing ints.
        let decoded: Test = serde_json::from_slice(r#"{"v":1507217624302}"#.as_bytes()).unwrap();
        assert_eq!(expected, decoded.v,);
        // Test parsing floats.
        let data = json!({
            "v": 1507217624302.0,
        });
        let decoded: Test = serde_json::from_value(data).unwrap();
        assert_eq!(expected, decoded.v,);
    }

    #[test]
    fn test_serialize_milliseconds() {
        #[derive(Serialize)]
        struct Test {
            #[serde(serialize_with = "serialize_milliseconds")]
            v: DateTime<Utc>,
        }
        let instance = Test {
            v: Utc.ymd(1983, 7, 22).and_hms_nano(1, 0, 0, 99),
        };
        let encoded = serde_json::to_string(&instance).unwrap();
        assert_eq!(encoded, String::from(r#"{"v":"427683600099"}"#));
    }

    #[test]
    fn test_serialize_seconds() {
        #[derive(Serialize)]
        struct Test {
            #[serde(serialize_with = "serialize_seconds")]
            v: DateTime<Utc>,
        }

        // Make sure nanoseconds are chopped off.
        let instance = Test {
            v: Utc.ymd(1983, 7, 22).and_hms_nano(1, 0, 0, 99),
        };
        let encoded = serde_json::to_string(&instance).unwrap();
        assert_eq!(encoded, String::from(r#"{"v":"427683600"}"#));

        // Make sure milliseconds are included.
        let instance = Test {
            v: Utc.ymd(1983, 7, 22).and_hms_nano(1, 0, 0, 2_000_000),
        };
        let encoded = serde_json::to_string(&instance).unwrap();
        assert_eq!(encoded, String::from(r#"{"v":"427683600.2"}"#));

        // Make sure milliseconds are included.
        let instance = Test {
            v: Utc.ymd(1983, 7, 22).and_hms_nano(1, 0, 0, 1_234_000_000),
        };
        let encoded = serde_json::to_string(&instance).unwrap();
        assert_eq!(encoded, String::from(r#"{"v":"427683600.1234"}"#));
    }

    #[cfg(feature = "string-null-empty")]
    #[test]
    fn test_deserialize_string() {
        #[derive(Deserialize)]
        struct Test {
            #[serde(deserialize_with = "deserialize_lambda_string")]
            v: String,
        }
        let input = json!({
          "v": "",
        });
        let decoded: Test = serde_json::from_value(input).unwrap();
        assert_eq!("".to_string(), decoded.v);

        let input = json!({
          "v": null,
        });
        let decoded: Test = serde_json::from_value(input).unwrap();
        assert_eq!("".to_string(), decoded.v);
    }

    #[cfg(feature = "string-null-none")]
    #[test]
    fn test_deserialize_string() {
        #[derive(Deserialize)]
        struct Test {
            #[serde(deserialize_with = "deserialize_lambda_string")]
            v: Option<String>,
        }
        let input = json!({
          "v": "",
        });
        let decoded: Test = serde_json::from_value(input).unwrap();
        assert_eq!(None, decoded.v);
        let input = json!({
          "v": null,
        });
        let decoded: Test = serde_json::from_value(input).unwrap();
        assert_eq!(None, decoded.v);
        let input = json!({
          "v": "foo",
        });
        let decoded: Test = serde_json::from_value(input).unwrap();
        assert_eq!(Some("foo".to_string()), decoded.v);
    }

    #[test]
    fn test_deserialize_map() {
        #[derive(Deserialize)]
        struct Test {
            #[serde(deserialize_with = "deserialize_lambda_map")]
            v: HashMap<String, String>,
        }
        let input = json!({
          "v": {},
        });
        let decoded: Test = serde_json::from_value(input).unwrap();
        assert_eq!(HashMap::new(), decoded.v);

        let input = json!({
          "v": null,
        });
        let decoded: Test = serde_json::from_value(input).unwrap();
        assert_eq!(HashMap::new(), decoded.v);
    }

    #[test]
    fn test_deserialize_duration_seconds() {
        #[derive(Deserialize)]
        struct Test {
            #[serde(deserialize_with = "deserialize_duration_seconds")]
            v: Duration,
        }

        let expected = Duration::seconds(36);

        let data = json!({
            "v": 36,
        });
        let decoded: Test = serde_json::from_value(data).unwrap();
        assert_eq!(expected, decoded.v,);

        let data = json!({
            "v": 36.1,
        });
        let decoded: Test = serde_json::from_value(data).unwrap();
        assert_eq!(expected, decoded.v,);
    }

    #[test]
    fn test_serialize_duration_seconds() {
        #[derive(Serialize)]
        struct Test {
            #[serde(serialize_with = "serialize_duration_seconds")]
            v: Duration,
        }
        let instance = Test {
            v: Duration::seconds(36),
        };
        let encoded = serde_json::to_string(&instance).unwrap();
        assert_eq!(encoded, String::from(r#"{"v":36}"#));
    }

    #[test]
    fn test_deserialize_duration_minutes() {
        #[derive(Deserialize)]
        struct Test {
            #[serde(deserialize_with = "deserialize_duration_minutes")]
            v: Duration,
        }

        let expected = Duration::minutes(36);

        let data = json!({
            "v": 36,
        });
        let decoded: Test = serde_json::from_value(data).unwrap();
        assert_eq!(expected, decoded.v,);

        let data = json!({
            "v": 36.1,
        });
        let decoded: Test = serde_json::from_value(data).unwrap();
        assert_eq!(expected, decoded.v,);
    }

    #[test]
    fn test_serialize_duration_minutes() {
        #[derive(Serialize)]
        struct Test {
            #[serde(serialize_with = "serialize_duration_minutes")]
            v: Duration,
        }
        let instance = Test {
            v: Duration::minutes(36),
        };
        let encoded = serde_json::to_string(&instance).unwrap();
        assert_eq!(encoded, String::from(r#"{"v":36}"#));
    }

    #[test]
    fn test_deserialize_missing_http_headers() {
        #[derive(Deserialize)]
        struct Test {
            #[serde(deserialize_with = "http_serde::header_map::deserialize", default)]
            pub headers: HeaderMap,
        }
        let data = json!({
            "not_headers": {}
        });

        let expected = HeaderMap::new();

        let decoded: Test = serde_json::from_value(data).unwrap();
        assert_eq!(expected, decoded.headers);
    }

    type TestTime = DateTime<Utc>;

    #[test]
    fn test_deserialize_codebuild_time() {
        #[derive(Deserialize)]
        struct Test {
            #[serde(with = "codebuild_time::str_time")]
            pub date: TestTime,
        }
        let data = json!({
            "date": "Sep 1, 2017 4:12:29 PM"
        });

        let expected = Utc
            .datetime_from_str("Sep 1, 2017 4:12:29 PM", CODEBUILD_TIME_FORMAT)
            .unwrap();
        let decoded: Test = serde_json::from_value(data).unwrap();
        assert_eq!(expected, decoded.date);
    }

    #[test]
    fn test_deserialize_codebuild_optional_time() {
        #[derive(Deserialize)]
        struct Test {
            #[serde(with = "codebuild_time::optional_time")]
            pub date: Option<TestTime>,
        }
        let data = json!({
            "date": "Sep 1, 2017 4:12:29 PM"
        });

        let expected = Utc
            .datetime_from_str("Sep 1, 2017 4:12:29 PM", CODEBUILD_TIME_FORMAT)
            .unwrap();
        let decoded: Test = serde_json::from_value(data).unwrap();
        assert_eq!(Some(expected), decoded.date);
    }
}
