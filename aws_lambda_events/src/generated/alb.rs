use super::super::encodings::Body;
use crate::custom_serde::*;
use http::{HeaderMap, Method};
use std::collections::HashMap;

/// `AlbTargetGroupRequest` contains data originating from the ALB Lambda target group integration
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbTargetGroupRequest {
    #[serde(with = "http_method")]
    pub http_method: Method,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub path: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    pub query_string_parameters: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    pub multi_value_query_string_parameters: HashMap<String, Vec<String>>,
    #[serde(deserialize_with = "http_serde::header_map::deserialize", default)]
    #[serde(serialize_with = "serialize_headers")]
    pub headers: HeaderMap,
    #[serde(deserialize_with = "http_serde::header_map::deserialize", default)]
    #[serde(serialize_with = "serialize_multi_value_headers")]
    pub multi_value_headers: HeaderMap,
    pub request_context: AlbTargetGroupRequestContext,
    pub is_base64_encoded: bool,
    pub body: Option<String>,
}

/// `AlbTargetGroupRequestContext` contains the information to identify the load balancer invoking the lambda
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbTargetGroupRequestContext {
    pub elb: ElbContext,
}

/// `ElbContext` contains the information to identify the ARN invoking the lambda
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ElbContext {
    /// nolint: stylecheck
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub target_group_arn: Option<String>,
}

/// `AlbTargetGroupResponse` configures the response to be returned by the ALB Lambda target group for the request
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbTargetGroupResponse {
    pub status_code: i64,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub status_description: Option<String>,
    #[serde(deserialize_with = "http_serde::header_map::deserialize", default)]
    #[serde(serialize_with = "serialize_headers")]
    pub headers: HeaderMap,
    #[serde(deserialize_with = "http_serde::header_map::deserialize", default)]
    #[serde(serialize_with = "serialize_multi_value_headers")]
    pub multi_value_headers: HeaderMap,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Body>,
    pub is_base64_encoded: bool,
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[test]
    #[cfg(feature = "alb")]
    fn example_alb_lambda_target_request_headers_only() {
        let data = include_bytes!("fixtures/example-alb-lambda-target-request-headers-only.json");
        let parsed: AlbTargetGroupRequest = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: AlbTargetGroupRequest = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }

    #[test]
    #[cfg(feature = "alb")]
    fn example_alb_lambda_target_request_multivalue_headers() {
        let data =
            include_bytes!("fixtures/example-alb-lambda-target-request-multivalue-headers.json");
        let parsed: AlbTargetGroupRequest = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: AlbTargetGroupRequest = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }

    #[test]
    #[cfg(feature = "alb")]
    fn example_alb_lambda_target_response() {
        let data = include_bytes!("fixtures/example-alb-lambda-target-response.json");
        let parsed: AlbTargetGroupResponse = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: AlbTargetGroupResponse = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
