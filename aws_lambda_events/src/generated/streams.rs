use crate::custom_serde::*;

/// `KinesisEventResponse` is the outer structure to report batch item failures for KinesisEvent.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct KinesisEventResponse {
    #[serde(rename = "batchItemFailures")]
    pub batch_item_failures: Vec<KinesisBatchItemFailure>,
}

/// `KinesisBatchItemFailure` is the individual record which failed processing.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct KinesisBatchItemFailure {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "itemIdentifier")]
    pub item_identifier: Option<String>,
}

/// `DynamoDbEventResponse` is the outer structure to report batch item failures for DynamoDBEvent.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DynamoDbEventResponse {
    #[serde(rename = "batchItemFailures")]
    pub batch_item_failures: Vec<DynamoDbBatchItemFailure>,
}

/// `DynamoDbBatchItemFailure` is the individual record which failed processing.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DynamoDbBatchItemFailure {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "itemIdentifier")]
    pub item_identifier: Option<String>,
}

/// `SqsEventResponse` is the outer structure to report batch item failures for SQSEvent.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SqsEventResponse {
    #[serde(rename = "batchItemFailures")]
    pub batch_item_failures: Vec<SqsBatchItemFailure>,
}

/// `SqsBatchItemFailure` is the individual record which failed processing.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SqsBatchItemFailure {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "itemIdentifier")]
    pub item_identifier: Option<String>,
}
