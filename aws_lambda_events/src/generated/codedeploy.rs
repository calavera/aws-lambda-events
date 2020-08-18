use crate::custom_serde::*;
use chrono::{DateTime, Utc};

pub type CodeDeployDeploymentState = String;

/// `CodeDeployEvent` is documented at:
/// https://docs.aws.amazon.com/AmazonCloudWatch/latest/events/EventTypes.html#acd_event_types
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CodeDeployEvent {
    /// AccountID is the id of the AWS account from which the event originated.
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "account")]
    pub account_id: Option<String>,
    /// Region is the AWS region from which the event originated.
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub region: Option<String>,
    /// DetailType informs the schema of the Detail field. For deployment state-change
    /// events, the value should be equal to CodeDeployDeploymentEventDetailType.
    /// For instance state-change events, the value should be equal to
    /// CodeDeployInstanceEventDetailType.
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "detail-type")]
    pub detail_type: Option<String>,
    /// Source should be equal to CodeDeployEventSource.
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub source: Option<String>,
    /// Version is the version of the event's schema.
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub version: Option<String>,
    /// Time is the event's timestamp.
    pub time: DateTime<Utc>,
    /// ID is the GUID of this event.
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub id: Option<String>,
    /// Resources is a list of ARNs of CodeDeploy applications and deployment
    /// groups that this event pertains to.
    pub resources: Vec<String>,
    /// Detail contains information specific to a deployment event.
    pub detail: CodeDeployEventDetail,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CodeDeployEventDetail {
    /// InstanceGroupID is the ID of the instance group.
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "instanceGroupId")]
    pub instance_group_id: Option<String>,
    /// InstanceID is the id of the instance. This field is non-empty only if
    /// the DetailType of the complete event is CodeDeployInstanceEventDetailType.
    #[serde(rename = "instanceId")]
    pub instance_id: Option<String>,
    /// Region is the AWS region that the event originated from.
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub region: Option<String>,
    /// Application is the name of the CodeDeploy application.
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub application: Option<String>,
    /// DeploymentID is the id of the deployment.
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "deploymentId")]
    pub deployment_id: Option<String>,
    /// State is the new state of the deployment.
    pub state: CodeDeployDeploymentState,
    /// DeploymentGroup is the name of the deployment group.
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "deploymentGroup")]
    pub deployment_group: Option<String>,
}
