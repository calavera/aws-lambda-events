use crate::{custom_serde::*, encodings::Base64Data};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SqsEvent {
    #[serde(rename = "Records")]
    pub records: Vec<SqsMessage>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SqsMessage {
    /// nolint: stylecheck
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub message_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub receipt_handle: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub body: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub md5_of_body: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub md5_of_message_attributes: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    pub attributes: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    pub message_attributes: HashMap<String, SqsMessageAttribute>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "eventSourceARN")]
    pub event_source_arn: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub event_source: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub aws_region: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SqsMessageAttribute {
    pub string_value: Option<String>,
    pub binary_value: Option<Base64Data>,
    pub string_list_values: Vec<String>,
    pub binary_list_values: Vec<Base64Data>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub data_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SqsBatchResponse {
    pub batch_item_failures: Vec<BatchItemFailure>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchItemFailure {
    item_identifier: String,
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[test]
    #[cfg(feature = "sqs")]
    fn example_sqs_event() {
        let data = include_bytes!("../generated/fixtures/example-sqs-event.json");
        let parsed: SqsEvent = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: SqsEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }

    #[test]
    #[cfg(feature = "sqs")]
    fn example_sqs_batch_response() {
        // Example sqs batch response fetched 2022-05-13, from:
        // https://docs.aws.amazon.com/lambda/latest/dg/with-sqs.html#services-sqs-batchfailurereporting
        let data = include_bytes!("../generated/fixtures/example-sqs-batch-response.json");
        let parsed: SqsBatchResponse = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: SqsBatchResponse = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
