use crate::custom_serde::*;
use chrono::{DateTime, Utc};

pub type CodePipelineStageState = String;

pub type CodePipelineState = String;

pub type CodePipelineActionState = String;

/// CodePipelineEvent is documented at:
/// https://docs.aws.amazon.com/AmazonCloudWatch/latest/events/EventTypes.html#codepipeline_event_type
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CodePipelineCloudWatchEvent {
    /// Version is the version of the event's schema.
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub version: Option<String>,
    /// ID is the GUID of this event.
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub id: Option<String>,
    /// DetailType informs the schema of the Detail field. For deployment state-change
    /// events, the value should be equal to CodePipelineDeploymentEventDetailType.
    /// For instance state-change events, the value should be equal to
    /// CodePipelineInstanceEventDetailType.
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "detail-type")]
    pub detail_type: Option<String>,
    /// Source should be equal to CodePipelineEventSource.
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub source: Option<String>,
    /// AccountID is the id of the AWS account from which the event originated.
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "account")]
    pub account_id: Option<String>,
    /// Time is the event's timestamp.
    pub time: DateTime<Utc>,
    /// Region is the AWS region from which the event originated.
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub region: Option<String>,
    /// Resources is a list of ARNs of CodePipeline applications and deployment
    /// groups that this event pertains to.
    pub resources: Vec<String>,
    /// Detail contains information specific to a deployment event.
    pub detail: CodePipelineEventDetail,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CodePipelineEventDetail {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub pipeline: Option<String>,
    /// From live testing this is always int64 not string as documented
    pub version: i64,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "execution-id")]
    pub execution_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub stage: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub action: Option<String>,
    pub state: CodePipelineState,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub region: Option<String>,
    #[serde(rename = "type")]
    pub type_: CodePipelineEventDetailType,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CodePipelineEventDetailType {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub owner: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub category: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub provider: Option<String>,
    /// From published EventBridge schema registry this is always int64 not string as documented
    pub version: i64,
}
