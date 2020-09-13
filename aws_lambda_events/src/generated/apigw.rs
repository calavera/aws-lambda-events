use crate::custom_serde::*;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json::Value;
use std::collections::HashMap;

/// `ApiGatewayProxyRequest` contains data coming from the API Gateway proxy
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayProxyRequest {
    /// The resource path defined in API Gateway
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub resource: Option<String>,
    /// The url path for the caller
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub path: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "httpMethod")]
    pub http_method: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    pub headers: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "multiValueHeaders")]
    pub multi_value_headers: HashMap<String, Vec<String>>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "queryStringParameters")]
    pub query_string_parameters: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "multiValueQueryStringParameters")]
    pub multi_value_query_string_parameters: HashMap<String, Vec<String>>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "pathParameters")]
    pub path_parameters: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "stageVariables")]
    pub stage_variables: HashMap<String, String>,
    #[serde(rename = "requestContext")]
    pub request_context: ApiGatewayProxyRequestContext,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub body: Option<String>,
    #[serde(rename = "isBase64Encoded")]
    pub is_base64_encoded: Option<bool>,
}

/// `ApiGatewayProxyResponse` configures the response to be returned by API Gateway for the request
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayProxyResponse {
    #[serde(rename = "statusCode")]
    pub status_code: i64,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    pub headers: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "multiValueHeaders")]
    pub multi_value_headers: HashMap<String, Vec<String>>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub body: Option<String>,
    #[serde(rename = "isBase64Encoded")]
    pub is_base64_encoded: Option<bool>,
}

/// `ApiGatewayProxyRequestContext` contains the information to identify the AWS account and resources invoking the
/// Lambda function. It also includes Cognito identity information for the caller.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayProxyRequestContext<T1 = Value>
where
    T1: DeserializeOwned,
    T1: Serialize,
{
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "accountId")]
    pub account_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "resourceId")]
    pub resource_id: Option<String>,
    #[serde(rename = "operationName")]
    pub operation_name: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub stage: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "domainName")]
    pub domain_name: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "domainPrefix")]
    pub domain_prefix: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "requestId")]
    pub request_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub protocol: Option<String>,
    pub identity: ApiGatewayRequestIdentity,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "resourcePath")]
    pub resource_path: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(bound = "")]
    pub authorizer: HashMap<String, T1>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "httpMethod")]
    pub http_method: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "requestTime")]
    pub request_time: Option<String>,
    #[serde(rename = "requestTimeEpoch")]
    pub request_time_epoch: i64,
    /// The API Gateway rest API Id
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "apiId")]
    pub apiid: Option<String>,
}

/// `ApiGatewayV2httpRequest` contains data coming from the new HTTP API Gateway
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayV2httpRequest {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub version: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "routeKey")]
    pub route_key: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "rawPath")]
    pub raw_path: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "rawQueryString")]
    pub raw_query_string: Option<String>,
    pub cookies: Option<Vec<String>>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    pub headers: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "queryStringParameters")]
    pub query_string_parameters: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "pathParameters")]
    pub path_parameters: HashMap<String, String>,
    #[serde(rename = "requestContext")]
    pub request_context: ApiGatewayV2httpRequestContext,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "stageVariables")]
    pub stage_variables: HashMap<String, String>,
    pub body: Option<String>,
    #[serde(rename = "isBase64Encoded")]
    pub is_base64_encoded: bool,
}

/// `ApiGatewayV2httpRequestContext` contains the information to identify the AWS account and resources invoking the Lambda function.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayV2httpRequestContext {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "routeKey")]
    pub route_key: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "accountId")]
    pub account_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub stage: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "requestId")]
    pub request_id: Option<String>,
    pub authorizer: Option<ApiGatewayV2httpRequestContextAuthorizerDescription>,
    /// The API Gateway HTTP API Id
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "apiId")]
    pub apiid: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "domainName")]
    pub domain_name: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "domainPrefix")]
    pub domain_prefix: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub time: Option<String>,
    #[serde(rename = "timeEpoch")]
    pub time_epoch: i64,
    pub http: ApiGatewayV2httpRequestContextHttpDescription,
}

/// `ApiGatewayV2httpRequestContextAuthorizerDescription` contains authorizer information for the request context.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayV2httpRequestContextAuthorizerDescription {
    pub jwt: ApiGatewayV2httpRequestContextAuthorizerJwtDescription,
}

/// `ApiGatewayV2httpRequestContextAuthorizerJwtDescription` contains JWT authorizer information for the request context.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayV2httpRequestContextAuthorizerJwtDescription {
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    pub claims: HashMap<String, String>,
    pub scopes: Vec<String>,
}

/// `ApiGatewayV2httpRequestContextHttpDescription` contains HTTP information for the request context.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayV2httpRequestContextHttpDescription {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub method: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub path: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub protocol: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "sourceIp")]
    pub source_ip: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "userAgent")]
    pub user_agent: Option<String>,
}

/// `ApiGatewayV2httpResponse` configures the response to be returned by API Gateway V2 for the request
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayV2httpResponse {
    #[serde(rename = "statusCode")]
    pub status_code: i64,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    pub headers: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "multiValueHeaders")]
    pub multi_value_headers: HashMap<String, Vec<String>>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub body: Option<String>,
    #[serde(rename = "isBase64Encoded")]
    pub is_base64_encoded: Option<bool>,
    pub cookies: Vec<String>,
}

/// `ApiGatewayRequestIdentity` contains identity information for the request caller.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayRequestIdentity {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "cognitoIdentityPoolId")]
    pub cognito_identity_pool_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "accountId")]
    pub account_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "cognitoIdentityId")]
    pub cognito_identity_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub caller: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "apiKey")]
    pub api_key: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "apiKeyId")]
    pub api_key_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "accessKey")]
    pub access_key: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "sourceIp")]
    pub source_ip: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "cognitoAuthenticationType")]
    pub cognito_authentication_type: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "cognitoAuthenticationProvider")]
    pub cognito_authentication_provider: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "userArn")]
    pub user_arn: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "userAgent")]
    pub user_agent: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub user: Option<String>,
}

/// `ApiGatewayWebsocketProxyRequest` contains data coming from the API Gateway proxy
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayWebsocketProxyRequest {
    /// The resource path defined in API Gateway
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub resource: Option<String>,
    /// The url path for the caller
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub path: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "httpMethod")]
    pub http_method: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    pub headers: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "multiValueHeaders")]
    pub multi_value_headers: HashMap<String, Vec<String>>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "queryStringParameters")]
    pub query_string_parameters: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "multiValueQueryStringParameters")]
    pub multi_value_query_string_parameters: HashMap<String, Vec<String>>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "pathParameters")]
    pub path_parameters: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "stageVariables")]
    pub stage_variables: HashMap<String, String>,
    #[serde(rename = "requestContext")]
    pub request_context: ApiGatewayWebsocketProxyRequestContext,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub body: Option<String>,
    #[serde(rename = "isBase64Encoded")]
    pub is_base64_encoded: Option<bool>,
}

/// `ApiGatewayWebsocketProxyRequestContext` contains the information to identify
/// the AWS account and resources invoking the Lambda function. It also includes
/// Cognito identity information for the caller.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayWebsocketProxyRequestContext<T1 = Value, T2 = Value>
where
    T1: DeserializeOwned,
    T1: Serialize,
    T2: DeserializeOwned,
    T2: Serialize,
{
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "accountId")]
    pub account_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "resourceId")]
    pub resource_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub stage: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "requestId")]
    pub request_id: Option<String>,
    pub identity: ApiGatewayRequestIdentity,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "resourcePath")]
    pub resource_path: Option<String>,
    #[serde(bound = "")]
    pub authorizer: Option<T1>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "httpMethod")]
    pub http_method: Option<String>,
    /// The API Gateway rest API Id
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "apiId")]
    pub apiid: Option<String>,
    #[serde(rename = "connectedAt")]
    pub connected_at: i64,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "connectionId")]
    pub connection_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "domainName")]
    pub domain_name: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub error: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "eventType")]
    pub event_type: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "extendedRequestId")]
    pub extended_request_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "integrationLatency")]
    pub integration_latency: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "messageDirection")]
    pub message_direction: Option<String>,
    #[serde(bound = "")]
    #[serde(rename = "messageId")]
    pub message_id: T2,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "requestTime")]
    pub request_time: Option<String>,
    #[serde(rename = "requestTimeEpoch")]
    pub request_time_epoch: i64,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "routeKey")]
    pub route_key: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub status: Option<String>,
}

/// `ApiGatewayCustomAuthorizerRequestTypeRequestIdentity` contains identity information for the request caller.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayCustomAuthorizerRequestTypeRequestIdentity {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "apiKey")]
    pub api_key: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "sourceIp")]
    pub source_ip: Option<String>,
}

/// `ApiGatewayCustomAuthorizerContext` represents the expected format of an API Gateway custom authorizer response.
/// Deprecated. Code should be updated to use the Authorizer map from APIGatewayRequestIdentity. Ex: Authorizer["principalId"]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayCustomAuthorizerContext {
    #[serde(rename = "principalId")]
    pub principal_id: Option<String>,
    #[serde(rename = "stringKey")]
    pub string_key: Option<String>,
    #[serde(rename = "numKey")]
    pub num_key: Option<i64>,
    #[serde(rename = "boolKey")]
    pub bool_key: Option<bool>,
}

/// `ApiGatewayCustomAuthorizerRequestTypeRequestContext` represents the expected format of an API Gateway custom authorizer response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayCustomAuthorizerRequestTypeRequestContext {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub path: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "accountId")]
    pub account_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "resourceId")]
    pub resource_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub stage: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "requestId")]
    pub request_id: Option<String>,
    pub identity: ApiGatewayCustomAuthorizerRequestTypeRequestIdentity,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "resourcePath")]
    pub resource_path: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "httpMethod")]
    pub http_method: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "apiId")]
    pub apiid: Option<String>,
}

/// `ApiGatewayCustomAuthorizerRequest` contains data coming in to a custom API Gateway authorizer function.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayCustomAuthorizerRequest {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "authorizationToken")]
    pub authorization_token: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "methodArn")]
    pub method_arn: Option<String>,
}

/// `ApiGatewayCustomAuthorizerRequestTypeRequest` contains data coming in to a custom API Gateway authorizer function.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayCustomAuthorizerRequestTypeRequest {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "methodArn")]
    pub method_arn: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub resource: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub path: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "httpMethod")]
    pub http_method: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    pub headers: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "multiValueHeaders")]
    pub multi_value_headers: HashMap<String, Vec<String>>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "queryStringParameters")]
    pub query_string_parameters: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "multiValueQueryStringParameters")]
    pub multi_value_query_string_parameters: HashMap<String, Vec<String>>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "pathParameters")]
    pub path_parameters: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "stageVariables")]
    pub stage_variables: HashMap<String, String>,
    #[serde(rename = "requestContext")]
    pub request_context: ApiGatewayCustomAuthorizerRequestTypeRequestContext,
}

/// `ApiGatewayCustomAuthorizerResponse` represents the expected format of an API Gateway authorization response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayCustomAuthorizerResponse<T1 = Value>
where
    T1: DeserializeOwned,
    T1: Serialize,
{
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "principalId")]
    pub principal_id: Option<String>,
    #[serde(rename = "policyDocument")]
    pub policy_document: ApiGatewayCustomAuthorizerPolicy,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(bound = "")]
    pub context: HashMap<String, T1>,
    #[serde(rename = "usageIdentifierKey")]
    pub usage_identifier_key: Option<String>,
}

/// `ApiGatewayCustomAuthorizerPolicy` represents an IAM policy
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayCustomAuthorizerPolicy {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "Version")]
    pub version: Option<String>,
    #[serde(rename = "Statement")]
    pub statement: Vec<IamPolicyStatement>,
}

/// `IamPolicyStatement` represents one statement from IAM policy with action, effect and resource
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct IamPolicyStatement {
    #[serde(rename = "Action")]
    pub action: Vec<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "Effect")]
    pub effect: Option<String>,
    #[serde(rename = "Resource")]
    pub resource: Vec<String>,
}
