use crate::custom_serde::*;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ActiveMqEvent {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "eventSource")]
    pub event_source: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "eventSourceArn")]
    pub event_source_arn: Option<String>,
    pub messages: Vec<ActiveMqMessage>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ActiveMqMessage {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "messageID")]
    pub message_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "messageType")]
    pub message_type: Option<String>,
    pub timestamp: i64,
    #[serde(rename = "deliveryMode")]
    pub delivery_mode: i64,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "correlationID")]
    pub correlation_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "replyTo")]
    pub reply_to: Option<String>,
    pub destination: ActiveMqDestination,
    pub redelivered: bool,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub expiration: i64,
    pub priority: i64,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub data: Option<String>,
    #[serde(rename = "brokerInTime")]
    pub broker_in_time: i64,
    #[serde(rename = "brokerOutTime")]
    pub broker_out_time: i64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ActiveMqDestination {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "physicalName")]
    pub physical_name: Option<String>,
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[test]
    fn example_activemq_event() {
        let data = include_bytes!("fixtures/example-activemq-event.json");
        let parsed: ActiveMqEvent = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: ActiveMqEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
