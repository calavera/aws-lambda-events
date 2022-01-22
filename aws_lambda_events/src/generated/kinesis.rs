use super::super::encodings::{Base64Data, SecondTimestamp};
use crate::custom_serde::*;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KinesisEvent {
    #[serde(rename = "Records")]
    pub records: Vec<KinesisEventRecord>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KinesisEventRecord {
    /// nolint: stylecheck
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub aws_region: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "eventID")]
    pub event_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub event_name: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub event_source: Option<String>,
    /// nolint: stylecheck
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "eventSourceARN")]
    pub event_source_arn: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub event_version: Option<String>,
    /// nolint: stylecheck
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub invoke_identity_arn: Option<String>,
    pub kinesis: KinesisRecord,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KinesisRecord {
    pub approximate_arrival_timestamp: SecondTimestamp,
    pub data: Base64Data,
    pub encryption_type: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub partition_key: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub sequence_number: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub kinesis_schema_version: Option<String>,
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[test]
    #[cfg(feature = "kinesis")]
    fn example_kinesis_event() {
        let data = include_bytes!("fixtures/example-kinesis-event.json");
        let parsed: KinesisEvent = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: KinesisEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
