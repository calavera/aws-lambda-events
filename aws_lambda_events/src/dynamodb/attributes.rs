use serde::{
    de::{Error as DeError, MapAccess, Visitor},
    Deserialize, Deserializer,
};
use std::{collections::HashMap, fmt};

#[derive(Debug, PartialEq)]
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
                    "NULL" => AttributeValue::Null,
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
                return Ok(value);
            }
        }

        deserializer.deserialize_map(AttributeValueVisitor)
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

        let attr: AttributeValue = serde_json::from_value(value).unwrap();
        match attr {
            AttributeValue::Null => {}
            other => panic!("unexpected value {:?}", other),
        }
    }

    #[test]
    fn test_string_attribute() {
        let value = serde_json::json!({
            "S": "value"
        });

        let attr: AttributeValue = serde_json::from_value(value).unwrap();
        match attr {
            AttributeValue::String(s) => assert_eq!("value", &s),
            other => panic!("unexpected value {:?}", other),
        }
    }

    #[test]
    fn test_number_attribute() {
        let value = serde_json::json!({
            "N": "123.45"
        });

        let attr: AttributeValue = serde_json::from_value(value).unwrap();
        match attr {
            AttributeValue::Number(n) => assert_eq!(123.45, n),
            other => panic!("unexpected value {:?}", other),
        }
    }

    #[test]
    fn test_binary_attribute() {
        let value = serde_json::json!({
            "B": "dGhpcyB0ZXh0IGlzIGJhc2U2NC1lbmNvZGVk"
        });

        let attr: AttributeValue = serde_json::from_value(value).unwrap();
        match attr {
            AttributeValue::Binary(b) => {
                let expected = base64::decode("dGhpcyB0ZXh0IGlzIGJhc2U2NC1lbmNvZGVk").unwrap();
                assert_eq!(expected, b)
            }
            other => panic!("unexpected value {:?}", other),
        }
    }

    #[test]
    fn test_boolean_attribute() {
        let value = serde_json::json!({
            "BOOL": true
        });

        let attr: AttributeValue = serde_json::from_value(value).unwrap();
        match attr {
            AttributeValue::Boolean(b) => assert_eq!(true, b),
            other => panic!("unexpected value {:?}", other),
        }
    }

    #[test]
    fn test_string_set_attribute() {
        let value = serde_json::json!({
            "SS": ["Giraffe", "Hippo" ,"Zebra"]
        });

        let attr: AttributeValue = serde_json::from_value(value).unwrap();
        match attr {
            AttributeValue::StringSet(s) => {
                let expected = vec!["Giraffe", "Hippo", "Zebra"];
                assert_eq!(expected, s.iter().collect::<Vec<_>>());
            }
            other => panic!("unexpected value {:?}", other),
        }
    }

    #[test]
    fn test_number_set_attribute() {
        let value = serde_json::json!({
            "NS": ["42.2", "-19", "7.5", "3.14"]
        });

        let attr: AttributeValue = serde_json::from_value(value).unwrap();
        match attr {
            AttributeValue::NumberSet(s) => {
                let expected = vec![42.2, -19.00, 7.5, 3.14];
                assert_eq!(expected, s);
            }
            other => panic!("unexpected value {:?}", other),
        }
    }

    #[test]
    fn test_binary_set_attribute() {
        let value = serde_json::json!({
            "BS": ["U3Vubnk=", "UmFpbnk=", "U25vd3k="]
        });

        let attr: AttributeValue = serde_json::from_value(value).unwrap();
        match attr {
            AttributeValue::BinarySet(s) => {
                let expected = vec!["U3Vubnk=", "UmFpbnk=", "U25vd3k="]
                    .into_iter()
                    .flat_map(|s| base64::decode(&s))
                    .collect::<Vec<_>>();
                assert_eq!(expected, s);
            }
            other => panic!("unexpected value {:?}", other),
        }
    }

    #[test]
    fn test_attribute_list_attribute() {
        let value = serde_json::json!({
            "L": [ {"S": "Cookies"} , {"S": "Coffee"}, {"N": "3.14159"}]
        });

        let attr: AttributeValue = serde_json::from_value(value).unwrap();
        match attr {
            AttributeValue::AttributeList(s) => {
                let expected = vec![
                    AttributeValue::String("Cookies".into()),
                    AttributeValue::String("Coffee".into()),
                    AttributeValue::Number(3.14159),
                ];
                assert_eq!(expected, s);
            }
            other => panic!("unexpected value {:?}", other),
        }
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
