use super::super::encodings::{MinuteDuration, SecondDuration};
use crate::custom_serde::*;
use chrono::{DateTime, Utc};
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json::Value;

pub type CodeBuildPhaseStatus = String;

pub type CodeBuildPhaseType = String;

/// `CodeBuildEvent` is documented at:
/// https://docs.aws.amazon.com/codebuild/latest/userguide/sample-build-notifications.html#sample-build-notifications-ref
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CodeBuildEvent {
    /// AccountID is the id of the AWS account from which the event originated.
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "account")]
    pub account_id: Option<String>,
    /// Region is the AWS region from which the event originated.
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub region: Option<String>,
    /// DetailType informs the schema of the Detail field. For build state-change
    /// events, the value will be CodeBuildStateChangeDetailType. For phase-change
    /// events, it will be CodeBuildPhaseChangeDetailType.
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "detail-type")]
    pub detail_type: Option<String>,
    /// Source should be equal to CodeBuildEventSource.
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
    /// Resources is a list of ARNs of CodeBuild builds that this event pertains to.
    pub resources: Vec<String>,
    /// Detail contains information specific to a build state-change or
    /// build phase-change event.
    pub detail: CodeBuildEventDetail,
}

/// `CodeBuildEventDetail` represents the all details related to the code build event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CodeBuildEventDetail {
    #[serde(rename = "build-status")]
    pub build_status: CodeBuildPhaseStatus,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "project-name")]
    pub project_name: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "build-id")]
    pub build_id: Option<String>,
    #[serde(rename = "additional-information")]
    pub additional_information: CodeBuildEventAdditionalInformation,
    #[serde(rename = "current-phase")]
    pub current_phase: CodeBuildPhaseType,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "current-phase-context")]
    pub current_phase_context: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub version: Option<String>,
    #[serde(rename = "completed-phase-status")]
    pub completed_phase_status: CodeBuildPhaseStatus,
    #[serde(rename = "completed-phase")]
    pub completed_phase: CodeBuildPhaseType,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "completed-phase-context")]
    pub completed_phase_context: Option<String>,
    #[serde(rename = "completed-phase-duration-seconds")]
    pub completed_phase_duration: SecondDuration,
    #[serde(rename = "completed-phase-start")]
    pub completed_phase_start: CodeBuildTime,
    #[serde(rename = "completed-phase-end")]
    pub completed_phase_end: CodeBuildTime,
}

/// `CodeBuildEventAdditionalInformation` represents additional information to the code build event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CodeBuildEventAdditionalInformation {
    pub artifact: CodeBuildArtifact,
    pub environment: CodeBuildEnvironment,
    #[serde(rename = "timeout-in-minutes")]
    pub timeout: MinuteDuration,
    #[serde(rename = "build-complete")]
    pub build_complete: bool,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub initiator: Option<String>,
    #[serde(rename = "build-start-time")]
    pub build_start_time: CodeBuildTime,
    pub source: CodeBuildSource,
    pub logs: CodeBuildLogs,
    pub phases: Vec<CodeBuildPhase>,
}

/// `CodeBuildArtifact` represents the artifact provided to build
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CodeBuildArtifact {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "md5sum")]
    pub md5_sum: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "sha256sum")]
    pub sha256_sum: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub location: Option<String>,
}

/// `CodeBuildEnvironment` represents the environment for a build
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CodeBuildEnvironment {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub image: Option<String>,
    #[serde(rename = "privileged-mode")]
    pub privileged_mode: bool,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "compute-type")]
    pub compute_type: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(rename = "environment-variables")]
    pub environment_variables: Vec<CodeBuildEnvironmentVariable>,
}

/// `CodeBuildEnvironmentVariable` encapsulate environment variables for the code build
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CodeBuildEnvironmentVariable {
    /// Name is the name of the environment variable.
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub name: Option<String>,
    /// Type is PLAINTEXT or PARAMETER_STORE.
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// Value is the value of the environment variable.
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub value: Option<String>,
}

/// `CodeBuildSource` represent the code source will be build
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CodeBuildSource {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub location: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

/// `CodeBuildLogs` gives the log details of a code build
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CodeBuildLogs {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "group-name")]
    pub group_name: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "stream-name")]
    pub stream_name: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "deep-link")]
    pub deep_link: Option<String>,
}

/// `CodeBuildPhase` represents the phase of a build and its details
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CodeBuildPhase<T1 = Value>
where
    T1: DeserializeOwned,
    T1: Serialize,
{
    #[serde(bound = "")]
    #[serde(rename = "phase-context")]
    pub phase_context: Vec<T1>,
    #[serde(rename = "start-time")]
    pub start_time: CodeBuildTime,
    #[serde(rename = "end-time")]
    pub end_time: CodeBuildTime,
    #[serde(rename = "duration-in-seconds")]
    pub duration: SecondDuration,
    #[serde(rename = "phase-type")]
    pub phase_type: CodeBuildPhaseType,
    #[serde(rename = "phase-status")]
    pub phase_status: CodeBuildPhaseStatus,
}

pub type CodeBuildTime = DateTime<Utc>;
