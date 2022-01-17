use crate::custom_serde::*;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json::Value;
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

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RabbitMqBasicProperties<T1 = Value>
where
    T1: DeserializeOwned,
    T1: Serialize,
{
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
    #[serde(rename = "contentEncoding")]
    pub content_encoding: Option<String>,
    /// Application or header exchange table
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(bound = "")]
    pub headers: HashMap<String, T1>,
    #[serde(rename = "deliveryMode")]
    pub delivery_mode: u8,
    pub priority: u8,
    #[serde(rename = "correlationId")]
    pub correlation_id: Option<String>,
    #[serde(rename = "replyTo")]
    pub reply_to: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub expiration: Option<String>,
    #[serde(rename = "messageId")]
    pub message_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub timestamp: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
    #[serde(rename = "appId")]
    pub app_id: Option<String>,
    #[serde(rename = "clusterId")]
    pub cluster_id: Option<String>,
    #[serde(rename = "bodySize")]
    pub body_size: u64,
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[test]
    #[cfg(feature = "rabbitmq")]
    fn example_rabbitmq_event() {
        let data = include_bytes!("fixtures/example-rabbitmq-event.json");
        let parsed: RabbitMqEvent = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: RabbitMqEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
