use super::super::encodings::Base64Data;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct KinesisAnalyticsOutputDeliveryEvent {
    #[serde(default)]
    #[serde(rename = "invocationId")]
    pub invocation_id: Option<String>,
    #[serde(default)]
    #[serde(rename = "applicationArn")]
    pub application_arn: Option<String>,
    pub records: Vec<KinesisAnalyticsOutputDeliveryEventRecord>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct KinesisAnalyticsOutputDeliveryEventRecord {
    #[serde(default)]
    #[serde(rename = "recordId")]
    pub record_id: Option<String>,
    pub data: Base64Data,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct KinesisAnalyticsOutputDeliveryResponse {
    pub records: Vec<KinesisAnalyticsOutputDeliveryResponseRecord>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct KinesisAnalyticsOutputDeliveryResponseRecord {
    #[serde(default)]
    #[serde(rename = "recordId")]
    pub record_id: Option<String>,
    /// possible values include Ok and DeliveryFailed
    #[serde(default)]
    pub result: Option<String>,
}
