use crate::custom_serde::*;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json::Value;
use std::collections::HashMap;

/// `CognitoEvent` contains data from an event sent from AWS Cognito Sync
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEvent {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "datasetName")]
    pub dataset_name: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "datasetRecords")]
    pub dataset_records: HashMap<String, CognitoDatasetRecord>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "eventType")]
    pub event_type: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "identityId")]
    pub identity_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "identityPoolId")]
    pub identity_pool_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub region: Option<String>,
    pub version: i64,
}

/// `CognitoDatasetRecord` represents a record from an AWS Cognito Sync event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoDatasetRecord {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "newValue")]
    pub new_value: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "oldValue")]
    pub old_value: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub op: Option<String>,
}

/// `CognitoEventUserPoolsPreSignup` is sent by AWS Cognito User Pools when a user attempts to register
/// (sign up), allowing a Lambda to perform custom validation to accept or deny the registration request
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsPreSignup {
    #[serde(rename = "CognitoEventUserPoolsHeader")]
    #[serde(flatten)]
    pub cognito_event_user_pools_header: CognitoEventUserPoolsHeader,
    pub request: CognitoEventUserPoolsPreSignupRequest,
    pub response: CognitoEventUserPoolsPreSignupResponse,
}

/// `CognitoEventUserPoolsPreAuthentication` is sent by AWS Cognito User Pools when a user submits their information
/// to be authenticated, allowing you to perform custom validations to accept or deny the sign in request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsPreAuthentication {
    #[serde(rename = "CognitoEventUserPoolsHeader")]
    #[serde(flatten)]
    pub cognito_event_user_pools_header: CognitoEventUserPoolsHeader,
    pub request: CognitoEventUserPoolsPreAuthenticationRequest,
    pub response: CognitoEventUserPoolsPreAuthenticationResponse,
}

/// `CognitoEventUserPoolsPostConfirmation` is sent by AWS Cognito User Pools after a user is confirmed,
/// allowing the Lambda to send custom messages or add custom logic.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsPostConfirmation {
    #[serde(rename = "CognitoEventUserPoolsHeader")]
    #[serde(flatten)]
    pub cognito_event_user_pools_header: CognitoEventUserPoolsHeader,
    pub request: CognitoEventUserPoolsPostConfirmationRequest,
    pub response: CognitoEventUserPoolsPostConfirmationResponse,
}

/// `CognitoEventUserPoolsPreTokenGen` is sent by AWS Cognito User Pools when a user attempts to retrieve
/// credentials, allowing a Lambda to perform insert, suppress or override claims
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsPreTokenGen {
    #[serde(rename = "CognitoEventUserPoolsHeader")]
    #[serde(flatten)]
    pub cognito_event_user_pools_header: CognitoEventUserPoolsHeader,
    pub request: CognitoEventUserPoolsPreTokenGenRequest,
    pub response: CognitoEventUserPoolsPreTokenGenResponse,
}

/// `CognitoEventUserPoolsPostAuthentication` is sent by AWS Cognito User Pools after a user is authenticated,
/// allowing the Lambda to add custom logic.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsPostAuthentication {
    #[serde(rename = "CognitoEventUserPoolsHeader")]
    #[serde(flatten)]
    pub cognito_event_user_pools_header: CognitoEventUserPoolsHeader,
    pub request: CognitoEventUserPoolsPostAuthenticationRequest,
    pub response: CognitoEventUserPoolsPostAuthenticationResponse,
}

/// `CognitoEventUserPoolsMigrateUser` is sent by AWS Cognito User Pools when a user does not exist in the
/// user pool at the time of sign-in with a password, or in the forgot-password flow.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsMigrateUser {
    #[serde(rename = "CognitoEventUserPoolsHeader")]
    #[serde(flatten)]
    pub cognito_event_user_pools_header: CognitoEventUserPoolsHeader,
    #[serde(rename = "request")]
    #[serde(flatten)]
    pub cognito_event_user_pools_migrate_user_request: CognitoEventUserPoolsMigrateUserRequest,
    #[serde(rename = "response")]
    #[serde(flatten)]
    pub cognito_event_user_pools_migrate_user_response: CognitoEventUserPoolsMigrateUserResponse,
}

/// `CognitoEventUserPoolsCallerContext` contains information about the caller
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsCallerContext {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "awsSdkVersion")]
    pub awssdk_version: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "clientId")]
    pub client_id: Option<String>,
}

/// `CognitoEventUserPoolsHeader` contains common data from events sent by AWS Cognito User Pools
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsHeader {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub version: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "triggerSource")]
    pub trigger_source: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub region: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "userPoolId")]
    pub user_pool_id: Option<String>,
    #[serde(rename = "callerContext")]
    pub caller_context: CognitoEventUserPoolsCallerContext,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "userName")]
    pub user_name: Option<String>,
}

/// `CognitoEventUserPoolsPreSignupRequest` contains the request portion of a PreSignup event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsPreSignupRequest {
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "userAttributes")]
    pub user_attributes: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "validationData")]
    pub validation_data: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "clientMetadata")]
    pub client_metadata: HashMap<String, String>,
}

/// `CognitoEventUserPoolsPreSignupResponse` contains the response portion of a PreSignup event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsPreSignupResponse {
    #[serde(rename = "autoConfirmUser")]
    pub auto_confirm_user: bool,
    #[serde(rename = "autoVerifyEmail")]
    pub auto_verify_email: bool,
    #[serde(rename = "autoVerifyPhone")]
    pub auto_verify_phone: bool,
}

/// `CognitoEventUserPoolsPreAuthenticationRequest` contains the request portion of a PreAuthentication event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsPreAuthenticationRequest {
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "userAttributes")]
    pub user_attributes: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "validationData")]
    pub validation_data: HashMap<String, String>,
}

/// `CognitoEventUserPoolsPreAuthenticationResponse` contains the response portion of a PreAuthentication event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsPreAuthenticationResponse;

/// `CognitoEventUserPoolsPostConfirmationRequest` contains the request portion of a PostConfirmation event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsPostConfirmationRequest {
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "userAttributes")]
    pub user_attributes: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "clientMetadata")]
    pub client_metadata: HashMap<String, String>,
}

/// `CognitoEventUserPoolsPostConfirmationResponse` contains the response portion of a PostConfirmation event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsPostConfirmationResponse;

/// `CognitoEventUserPoolsPreTokenGenRequest` contains request portion of PreTokenGen event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsPreTokenGenRequest {
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "userAttributes")]
    pub user_attributes: HashMap<String, String>,
    #[serde(rename = "groupConfiguration")]
    pub group_configuration: GroupConfiguration,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "clientMetadata")]
    pub client_metadata: HashMap<String, String>,
}

/// `CognitoEventUserPoolsPreTokenGenResponse` containst the response portion of  a PreTokenGen event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsPreTokenGenResponse {
    #[serde(rename = "claimsOverrideDetails")]
    pub claims_override_details: ClaimsOverrideDetails,
}

/// `CognitoEventUserPoolsPostAuthenticationRequest` contains the request portion of a PostAuthentication event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsPostAuthenticationRequest {
    #[serde(rename = "newDeviceUsed")]
    pub new_device_used: bool,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "userAttributes")]
    pub user_attributes: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "clientMetadata")]
    pub client_metadata: HashMap<String, String>,
}

/// `CognitoEventUserPoolsPostAuthenticationResponse` contains the response portion of a PostAuthentication event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsPostAuthenticationResponse;

/// `CognitoEventUserPoolsMigrateUserRequest` contains the request portion of a MigrateUser event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsMigrateUserRequest {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub password: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "validationData")]
    pub validation_data: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "clientMetadata")]
    pub client_metadata: HashMap<String, String>,
}

/// `CognitoEventUserPoolsMigrateUserResponse` contains the response portion of a MigrateUser event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsMigrateUserResponse {
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "userAttributes")]
    pub user_attributes: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "finalUserStatus")]
    pub final_user_status: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "messageAction")]
    pub message_action: Option<String>,
    #[serde(rename = "desiredDeliveryMediums")]
    pub desired_delivery_mediums: Vec<String>,
    #[serde(rename = "forceAliasCreation")]
    pub force_alias_creation: bool,
}

/// `ClaimsOverrideDetails` allows lambda to add, suppress or override claims in the token
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ClaimsOverrideDetails {
    #[serde(rename = "groupOverrideDetails")]
    pub group_override_details: GroupConfiguration,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "claimsToAddOrOverride")]
    pub claims_to_add_or_override: HashMap<String, String>,
    #[serde(rename = "claimsToSuppress")]
    pub claims_to_suppress: Vec<String>,
}

/// `GroupConfiguration` allows lambda to override groups, roles and set a perferred role
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GroupConfiguration {
    #[serde(rename = "groupsToOverride")]
    pub groups_to_override: Vec<String>,
    #[serde(rename = "iamRolesToOverride")]
    pub iam_roles_to_override: Vec<String>,
    #[serde(rename = "preferredRole")]
    pub preferred_role: Option<String>,
}

/// `CognitoEventUserPoolsChallengeResult` represents a challenge that is presented to the user in the authentication
/// process that is underway, along with the corresponding result.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsChallengeResult {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "challengeName")]
    pub challenge_name: Option<String>,
    #[serde(rename = "challengeResult")]
    pub challenge_result: bool,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "challengeMetadata")]
    pub challenge_metadata: Option<String>,
}

/// `CognitoEventUserPoolsDefineAuthChallengeRequest` defines auth challenge request parameters
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsDefineAuthChallengeRequest {
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "userAttributes")]
    pub user_attributes: HashMap<String, String>,
    pub session: Vec<Option<CognitoEventUserPoolsChallengeResult>>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "clientMetadata")]
    pub client_metadata: HashMap<String, String>,
    #[serde(rename = "userNotFound")]
    pub user_not_found: bool,
}

/// `CognitoEventUserPoolsDefineAuthChallengeResponse` defines auth challenge response parameters
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsDefineAuthChallengeResponse {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "challengeName")]
    pub challenge_name: Option<String>,
    #[serde(rename = "issueTokens")]
    pub issue_tokens: bool,
    #[serde(rename = "failAuthentication")]
    pub fail_authentication: bool,
}

/// `CognitoEventUserPoolsDefineAuthChallenge` sent by AWS Cognito User Pools to initiate custom authentication flow
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsDefineAuthChallenge {
    #[serde(rename = "CognitoEventUserPoolsHeader")]
    #[serde(flatten)]
    pub cognito_event_user_pools_header: CognitoEventUserPoolsHeader,
    pub request: CognitoEventUserPoolsDefineAuthChallengeRequest,
    pub response: CognitoEventUserPoolsDefineAuthChallengeResponse,
}

/// `CognitoEventUserPoolsCreateAuthChallengeRequest` defines create auth challenge request parameters
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsCreateAuthChallengeRequest {
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "userAttributes")]
    pub user_attributes: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "challengeName")]
    pub challenge_name: Option<String>,
    pub session: Vec<Option<CognitoEventUserPoolsChallengeResult>>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "clientMetadata")]
    pub client_metadata: HashMap<String, String>,
}

/// `CognitoEventUserPoolsCreateAuthChallengeResponse` defines create auth challenge response rarameters
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsCreateAuthChallengeResponse {
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "publicChallengeParameters")]
    pub public_challenge_parameters: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "privateChallengeParameters")]
    pub private_challenge_parameters: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "challengeMetadata")]
    pub challenge_metadata: Option<String>,
}

/// `CognitoEventUserPoolsCreateAuthChallenge` sent by AWS Cognito User Pools to create a challenge to present to the user
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsCreateAuthChallenge {
    #[serde(rename = "CognitoEventUserPoolsHeader")]
    #[serde(flatten)]
    pub cognito_event_user_pools_header: CognitoEventUserPoolsHeader,
    pub request: CognitoEventUserPoolsCreateAuthChallengeRequest,
    pub response: CognitoEventUserPoolsCreateAuthChallengeResponse,
}

/// `CognitoEventUserPoolsVerifyAuthChallengeRequest` defines verify auth challenge request parameters
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsVerifyAuthChallengeRequest<T1 = Value>
where
    T1: DeserializeOwned,
    T1: Serialize,
{
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "userAttributes")]
    pub user_attributes: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "privateChallengeParameters")]
    pub private_challenge_parameters: HashMap<String, String>,
    #[serde(bound = "")]
    #[serde(rename = "challengeAnswer")]
    pub challenge_answer: Option<T1>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "clientMetadata")]
    pub client_metadata: HashMap<String, String>,
}

/// `CognitoEventUserPoolsVerifyAuthChallengeResponse` defines verify auth challenge response parameters
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsVerifyAuthChallengeResponse {
    #[serde(rename = "answerCorrect")]
    pub answer_correct: bool,
}

/// `CognitoEventUserPoolsVerifyAuthChallenge` sent by AWS Cognito User Pools to verify if the response from the end user
/// for a custom Auth Challenge is valid or not
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsVerifyAuthChallenge {
    #[serde(rename = "CognitoEventUserPoolsHeader")]
    #[serde(flatten)]
    pub cognito_event_user_pools_header: CognitoEventUserPoolsHeader,
    pub request: CognitoEventUserPoolsVerifyAuthChallengeRequest,
    pub response: CognitoEventUserPoolsVerifyAuthChallengeResponse,
}

/// `CognitoEventUserPoolsCustomMessage` is sent by AWS Cognito User Pools before a verification or MFA message is sent,
/// allowing a user to customize the message dynamically.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsCustomMessage {
    #[serde(rename = "CognitoEventUserPoolsHeader")]
    #[serde(flatten)]
    pub cognito_event_user_pools_header: CognitoEventUserPoolsHeader,
    pub request: CognitoEventUserPoolsCustomMessageRequest,
    pub response: CognitoEventUserPoolsCustomMessageResponse,
}

/// `CognitoEventUserPoolsCustomMessageRequest` contains the request portion of a CustomMessage event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsCustomMessageRequest<T1 = Value>
where
    T1: DeserializeOwned,
    T1: Serialize,
{
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(bound = "")]
    #[serde(rename = "userAttributes")]
    pub user_attributes: HashMap<String, T1>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "codeParameter")]
    pub code_parameter: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "usernameParameter")]
    pub username_parameter: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "clientMetadata")]
    pub client_metadata: HashMap<String, String>,
}

/// `CognitoEventUserPoolsCustomMessageResponse` contains the response portion of a CustomMessage event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsCustomMessageResponse {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "smsMessage")]
    pub sms_message: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "emailMessage")]
    pub email_message: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "emailSubject")]
    pub email_subject: Option<String>,
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[test]
    fn example_cognito_event() {
        let data = include_bytes!("fixtures/example-cognito-event.json");
        let parsed: CognitoEvent = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: CognitoEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
