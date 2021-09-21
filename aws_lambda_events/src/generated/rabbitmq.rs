use crate::custom_serde::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RabbitMqEvent {
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
    #[serde(rename = "rmqMessagesByQueue")]
    pub messages_by_queue: HashMap<String, Vec<RabbitMqMessage>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RabbitMqMessage {
    #[serde(rename = "basicProperties")]
    pub basic_properties: RabbitMqBasicProperties,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub data: Option<String>,
    pub redelivered: bool,
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[test]
    fn example_rabbitmq_event() {
        let data = include_bytes!("fixtures/example-rabbitmq-event.json");
        let parsed: RabbitMqEvent = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: RabbitMqEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
