use super::super::encodings::Body;
use crate::custom_serde::*;
use http::{HeaderMap, Method};
use std::collections::HashMap;

/// `AlbTargetGroupRequest` contains data originating from the ALB Lambda target group integration
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AlbTargetGroupRequest {
    #[serde(with = "http_method")]
    #[serde(rename = "httpMethod")]
    pub http_method: Method,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub path: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "queryStringParameters")]
    pub query_string_parameters: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "multiValueQueryStringParameters")]
    pub multi_value_query_string_parameters: HashMap<String, Vec<String>>,
    #[serde(deserialize_with = "http_serde::header_map::deserialize", default)]
    #[serde(serialize_with = "serialize_headers")]
    pub headers: HeaderMap,
    #[serde(deserialize_with = "http_serde::header_map::deserialize", default)]
    #[serde(serialize_with = "serialize_multi_value_headers")]
    #[serde(rename = "multiValueHeaders")]
    pub multi_value_headers: HeaderMap,
    #[serde(rename = "requestContext")]
    pub request_context: AlbTargetGroupRequestContext,
    #[serde(rename = "isBase64Encoded")]
    pub is_base64_encoded: bool,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub body: Option<String>,
}

/// `AlbTargetGroupRequestContext` contains the information to identify the load balancer invoking the lambda
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AlbTargetGroupRequestContext {
    pub elb: ElbContext,
}

/// `ElbContext` contains the information to identify the ARN invoking the lambda
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ElbContext {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "targetGroupArn")]
    pub target_group_arn: Option<String>,
}

/// `AlbTargetGroupResponse` configures the response to be returned by the ALB Lambda target group for the request
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AlbTargetGroupResponse {
    #[serde(rename = "statusCode")]
    pub status_code: i64,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "statusDescription")]
    pub status_description: Option<String>,
    #[serde(deserialize_with = "http_serde::header_map::deserialize", default)]
    #[serde(serialize_with = "serialize_headers")]
    pub headers: HeaderMap,
    #[serde(deserialize_with = "http_serde::header_map::deserialize", default)]
    #[serde(serialize_with = "serialize_multi_value_headers")]
    #[serde(rename = "multiValueHeaders")]
    pub multi_value_headers: HeaderMap,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Body>,
    #[serde(rename = "isBase64Encoded")]
    pub is_base64_encoded: bool,
}
