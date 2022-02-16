use serde::{
    de::{Error as DeError, MapAccess, Visitor},
    ser::SerializeMap,
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::{collections::HashMap, fmt};

#[derive(Clone, Debug, PartialEq)]
pub enum AttributeValue {
    Null,
    String(String),
    Number(f64),
    Boolean(bool),
    Binary(Vec<u8>),
    StringSet(Vec<String>),
    NumberSet(Vec<f64>),
    BinarySet(Vec<Vec<u8>>),
    AttributeList(Vec<AttributeValue>),
    AttributeMap(HashMap<String, AttributeValue>),
}

impl<'de> Deserialize<'de> for AttributeValue {
    fn deserialize<D>(deserializer: D) -> Result<AttributeValue, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AttributeValueVisitor;

        impl<'de> Visitor<'de> for AttributeValueVisitor {
            type Value = AttributeValue;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, "a map")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let key = match map.next_key::<String>()? {
                    None => return Ok(AttributeValue::Null),
                    Some(key) => key,
                };

                let value = match key.as_str() {
                    "NULL" => {
                        // consume the next value, even if we don't use it
                        // Serde will fail because of trailing characters if we don't
                        map.next_value::<bool>()?;
                        AttributeValue::Null
                    }
                    "S" => AttributeValue::String(map.next_value::<String>()?),
                    "N" => {
                        let value = map.next_value::<String>()?;
                        let value = value.parse::<f64>().map_err(|e| {
                            A::Error::custom(format!("parse error {} for {}", e, &value))
                        })?;
                        AttributeValue::Number(value)
                    }
                    "B" => {
                        let value = map.next_value::<String>()?;
                        let value = base64::decode(&value).map_err(|e| {
                            A::Error::custom(format!("parse error {} for {}", e, &value))
                        })?;
                        AttributeValue::Binary(value)
                    }
                    "BOOL" => AttributeValue::Boolean(map.next_value::<bool>()?),
                    "SS" => AttributeValue::StringSet(map.next_value::<Vec<String>>()?),
                    "NS" => {
                        let value = map.next_value::<Vec<String>>()?;
                        let set = value
                            .into_iter()
                            .flat_map(|s| {
                                s.parse::<f64>().map_err(|e| {
                                    A::Error::custom(format!("parse error {} for {}", e, &s))
                                })
                            })
                            .collect::<Vec<_>>();
                        AttributeValue::NumberSet(set)
                    }
                    "BS" => {
                        let value = map.next_value::<Vec<String>>()?;
                        let set = value
                            .into_iter()
                            .flat_map(|s| {
                                base64::decode(&s).map_err(|e| {
                                    A::Error::custom(format!("parse error {} for {}", e, &s))
                                })
                            })
                            .collect::<Vec<_>>();
                        AttributeValue::BinarySet(set)
                    }
                    "L" => AttributeValue::AttributeList(map.next_value::<Vec<AttributeValue>>()?),
                    "M" => AttributeValue::AttributeMap(
                        map.next_value::<HashMap<String, AttributeValue>>()?,
                    ),
                    other => {
                        return Err(A::Error::custom(format!(
                            "unexpected dynamodb type {}",
                            other
                        )))
                    }
                };
                Ok(value)
            }
        }

        deserializer.deserialize_map(AttributeValueVisitor)
    }
}

impl Serialize for AttributeValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        match self {
            AttributeValue::Null => map.serialize_entry("NULL", &true)?,
            AttributeValue::String(s) => map.serialize_entry("S", s)?,
            AttributeValue::Number(n) => map.serialize_entry("N", &n.to_string())?,
            AttributeValue::Boolean(b) => map.serialize_entry("BOOL", b)?,
            AttributeValue::Binary(b) => {
                let value = base64::encode(b);
                map.serialize_entry("B", &value)?
            }
            AttributeValue::StringSet(s) => map.serialize_entry("SS", s)?,
            AttributeValue::NumberSet(s) => {
                let value = s.iter().map(|n| n.to_string()).collect::<Vec<_>>();
                map.serialize_entry("NS", &value)?
            }
            AttributeValue::BinarySet(s) => {
                let value = s.iter().map(base64::encode).collect::<Vec<_>>();
                map.serialize_entry("BS", &value)?
            }
            AttributeValue::AttributeList(l) => map.serialize_entry("L", l)?,
            AttributeValue::AttributeMap(m) => map.serialize_entry("M", m)?,
        }
        map.end()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_null_attribute() {
        let value = serde_json::json!({
            "NULL": true
        });

        let attr: AttributeValue = serde_json::from_value(value.clone()).unwrap();
        match attr {
            AttributeValue::Null => {}
            other => panic!("unexpected value {:?}", other),
        }

        let reparsed = serde_json::to_value(attr).unwrap();
        assert_eq!(value, reparsed);
    }

    #[test]
    fn test_string_attribute() {
        let value = serde_json::json!({
            "S": "value"
        });

        let attr: AttributeValue = serde_json::from_value(value.clone()).unwrap();
        match attr {
            AttributeValue::String(ref s) => assert_eq!("value", s.as_str()),
            other => panic!("unexpected value {:?}", other),
        }

        let reparsed = serde_json::to_value(attr).unwrap();
        assert_eq!(value, reparsed);
    }

    #[test]
    fn test_number_attribute() {
        let value = serde_json::json!({
            "N": "123.45"
        });

        let attr: AttributeValue = serde_json::from_value(value.clone()).unwrap();
        match attr {
            AttributeValue::Number(n) => assert_eq!(123.45, n),
            other => panic!("unexpected value {:?}", other),
        }

        let reparsed = serde_json::to_value(attr).unwrap();
        assert_eq!(value, reparsed);
    }

    #[test]
    fn test_binary_attribute() {
        let value = serde_json::json!({
            "B": "dGhpcyB0ZXh0IGlzIGJhc2U2NC1lbmNvZGVk"
        });

        let attr: AttributeValue = serde_json::from_value(value.clone()).unwrap();
        match attr {
            AttributeValue::Binary(ref b) => {
                let expected = base64::decode("dGhpcyB0ZXh0IGlzIGJhc2U2NC1lbmNvZGVk").unwrap();
                assert_eq!(&expected, b)
            }
            other => panic!("unexpected value {:?}", other),
        }

        let reparsed = serde_json::to_value(attr).unwrap();
        assert_eq!(value, reparsed);
    }

    #[test]
    fn test_boolean_attribute() {
        let value = serde_json::json!({
            "BOOL": true
        });

        let attr: AttributeValue = serde_json::from_value(value.clone()).unwrap();
        match attr {
            AttributeValue::Boolean(b) => assert_eq!(true, b),
            other => panic!("unexpected value {:?}", other),
        }

        let reparsed = serde_json::to_value(attr).unwrap();
        assert_eq!(value, reparsed);
    }

    #[test]
    fn test_string_set_attribute() {
        let value = serde_json::json!({
            "SS": ["Giraffe", "Hippo" ,"Zebra"]
        });

        let attr: AttributeValue = serde_json::from_value(value.clone()).unwrap();
        match attr {
            AttributeValue::StringSet(ref s) => {
                let expected = vec!["Giraffe", "Hippo", "Zebra"];
                assert_eq!(expected, s.iter().collect::<Vec<_>>());
            }
            other => panic!("unexpected value {:?}", other),
        }

        let reparsed = serde_json::to_value(attr).unwrap();
        assert_eq!(value, reparsed);
    }

    #[test]
    fn test_number_set_attribute() {
        let value = serde_json::json!({
            "NS": ["42.2", "-19", "7.5", "3.14"]
        });

        let attr: AttributeValue = serde_json::from_value(value.clone()).unwrap();
        match attr {
            AttributeValue::NumberSet(ref s) => {
                let expected = vec![42.2, -19.00, 7.5, 3.14];
                assert_eq!(&expected, s);
            }
            other => panic!("unexpected value {:?}", other),
        }

        let reparsed = serde_json::to_value(attr).unwrap();
        assert_eq!(value, reparsed);
    }

    #[test]
    fn test_binary_set_attribute() {
        let value = serde_json::json!({
            "BS": ["U3Vubnk=", "UmFpbnk=", "U25vd3k="]
        });

        let attr: AttributeValue = serde_json::from_value(value.clone()).unwrap();
        match attr {
            AttributeValue::BinarySet(ref s) => {
                let expected = vec!["U3Vubnk=", "UmFpbnk=", "U25vd3k="]
                    .into_iter()
                    .flat_map(|s| base64::decode(&s))
                    .collect::<Vec<_>>();
                assert_eq!(&expected, s);
            }
            other => panic!("unexpected value {:?}", other),
        }

        let reparsed = serde_json::to_value(attr).unwrap();
        assert_eq!(value, reparsed);
    }

    #[test]
    fn test_attribute_list_attribute() {
        let value = serde_json::json!({
            "L": [ {"S": "Cookies"} , {"S": "Coffee"}, {"N": "3.14159"}]
        });

        let attr: AttributeValue = serde_json::from_value(value.clone()).unwrap();
        match attr {
            AttributeValue::AttributeList(ref s) => {
                let expected = vec![
                    AttributeValue::String("Cookies".into()),
                    AttributeValue::String("Coffee".into()),
                    AttributeValue::Number(3.14159),
                ];
                assert_eq!(&expected, s);
            }
            other => panic!("unexpected value {:?}", other),
        }

        let reparsed = serde_json::to_value(attr).unwrap();
        assert_eq!(value, reparsed);
    }

    #[test]
    fn test_attribute_map_attribute() {
        let value = serde_json::json!({
            "M": {"Name": {"S": "Joe"}, "Age": {"N": "35"}}
        });

        let attr: AttributeValue = serde_json::from_value(value).unwrap();
        match attr {
            AttributeValue::AttributeMap(s) => {
                let mut expected = HashMap::new();
                expected.insert("Name".into(), AttributeValue::String("Joe".into()));
                expected.insert("Age".into(), AttributeValue::Number(35.00));
                assert_eq!(expected, s);
            }
            other => panic!("unexpected value {:?}", other),
        }
    }
}
