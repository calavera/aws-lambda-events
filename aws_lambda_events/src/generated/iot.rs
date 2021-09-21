use super::super::encodings::Base64Data;
use crate::custom_serde::*;
use http::HeaderMap;

/// `IoTCustomAuthorizerRequest` contains data coming in to a custom IoT device gateway authorizer function.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct IoTCustomAuthorizerRequest {
    #[serde(rename = "httpContext")]
    pub http_context: Option<IoThttpContext>,
    #[serde(rename = "mqttContext")]
    pub mqtt_context: Option<IoTmqttContext>,
    #[serde(rename = "tlsContext")]
    pub tls_context: Option<IoTtlsContext>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "token")]
    pub authorization_token: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "tokenSignature")]
    pub token_signature: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct IoThttpContext {
    #[serde(deserialize_with = "http_serde::header_map::deserialize", default)]
    #[serde(serialize_with = "serialize_headers")]
    pub headers: HeaderMap,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "queryString")]
    pub query_string: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct IoTmqttContext {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "clientId")]
    pub client_id: Option<String>,
    pub password: Base64Data,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub username: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct IoTtlsContext {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "serverName")]
    pub server_name: Option<String>,
}

/// `IoTCustomAuthorizerResponse` represents the expected format of an IoT device gateway authorization response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct IoTCustomAuthorizerResponse {
    #[serde(rename = "isAuthenticated")]
    pub is_authenticated: bool,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "principalId")]
    pub principal_id: Option<String>,
    #[serde(rename = "disconnectAfterInSeconds")]
    pub disconnect_after_in_seconds: i32,
    #[serde(rename = "refreshAfterInSeconds")]
    pub refresh_after_in_seconds: i32,
    #[serde(rename = "policyDocuments")]
    pub policy_documents: Vec<String>,
}
