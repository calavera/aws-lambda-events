use crate::custom_serde::*;

/// `S3BatchJobEvent` encapsulates the detail of a s3 batch job
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct S3BatchJobEvent {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub invocation_schema_version: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub invocation_id: Option<String>,
    pub job: S3BatchJob,
    pub tasks: Vec<S3BatchJobTask>,
}

/// `S3BatchJob` whichs have the job id
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct S3BatchJob {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub id: Option<String>,
}

/// `S3BatchJobTask` represents one task in the s3 batch job and have all task details
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct S3BatchJobTask {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub task_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub s3_key: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub s3_version_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub s3_bucket_arn: Option<String>,
}

/// `S3BatchJobResponse` is the response of a iven s3 batch job with the results
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct S3BatchJobResponse {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub invocation_schema_version: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub treat_missing_keys_as: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub invocation_id: Option<String>,
    pub results: Vec<S3BatchJobResult>,
}

/// `S3BatchJobResult` represents the result of a given task
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct S3BatchJobResult {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub task_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub result_code: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub result_string: Option<String>,
}
