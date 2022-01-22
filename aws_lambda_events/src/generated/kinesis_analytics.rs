use super::super::encodings::Base64Data;
use crate::custom_serde::*;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KinesisAnalyticsOutputDeliveryEvent {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub invocation_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub application_arn: Option<String>,
    pub records: Vec<KinesisAnalyticsOutputDeliveryEventRecord>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KinesisAnalyticsOutputDeliveryEventRecord {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub record_id: Option<String>,
    pub data: Base64Data,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KinesisAnalyticsOutputDeliveryResponse {
    pub records: Vec<KinesisAnalyticsOutputDeliveryResponseRecord>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KinesisAnalyticsOutputDeliveryResponseRecord {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub record_id: Option<String>,
    /// possible values include Ok and DeliveryFailed
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub result: Option<String>,
}
