use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ChimeBotEvent {
    #[serde(rename = "Sender")]
    pub sender: ChimeBotEventSender,
    #[serde(rename = "Discussion")]
    pub discussion: ChimeBotEventDiscussion,
    #[serde(default)]
    #[serde(rename = "EventType")]
    pub event_type: Option<String>,
    #[serde(rename = "InboundHttpsEndpoint")]
    pub inbound_https_endpoint: Option<ChimeBotEventInboundHttpsEndpoint>,
    #[serde(rename = "EventTimestamp")]
    pub event_timestamp: DateTime<Utc>,
    #[serde(rename = "Message")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ChimeBotEventSender {
    #[serde(default)]
    #[serde(rename = "SenderId")]
    pub sender_id: Option<String>,
    #[serde(default)]
    #[serde(rename = "SenderIdType")]
    pub sender_id_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ChimeBotEventDiscussion {
    #[serde(default)]
    #[serde(rename = "DiscussionId")]
    pub discussion_id: Option<String>,
    #[serde(default)]
    #[serde(rename = "DiscussionType")]
    pub discussion_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ChimeBotEventInboundHttpsEndpoint {
    #[serde(default)]
    #[serde(rename = "EndpointType")]
    pub endpoint_type: Option<String>,
    #[serde(default)]
    #[serde(rename = "Url")]
    pub url: Option<String>,
}
