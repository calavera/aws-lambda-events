use super::super::encodings::{Base64Data, MillisecondTimestamp};
use crate::custom_serde::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct KafkaEvent {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "eventSource")]
    pub event_source: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "eventSourceArn")]
    pub event_source_arn: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    pub records: HashMap<String, Vec<KafkaRecord>>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "bootstrapServers")]
    pub bootstrap_servers: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct KafkaRecord {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub topic: Option<String>,
    pub partition: i64,
    pub offset: i64,
    pub timestamp: MillisecondTimestamp,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "timestampType")]
    pub timestamp_type: Option<String>,
    pub key: Option<String>,
    pub value: Option<String>,
    pub headers: Vec<HashMap<String, Base64Data>>,
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[test]
    #[cfg(feature = "kafka")]
    fn example_kafka_event() {
        let data = include_bytes!("fixtures/example-kafka-event.json");
        let parsed: KafkaEvent = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: KafkaEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
