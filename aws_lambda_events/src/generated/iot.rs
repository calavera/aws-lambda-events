use super::super::encodings::Base64Data;
use crate::custom_serde::*;
use http::HeaderMap;

/// `IoTCustomAuthorizerRequest` contains data coming in to a custom IoT device gateway authorizer function.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IoTCustomAuthorizerRequest {
    pub http_context: Option<IoThttpContext>,
    pub mqtt_context: Option<IoTmqttContext>,
    pub tls_context: Option<IoTtlsContext>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "token")]
    pub authorization_token: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub token_signature: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IoThttpContext {
    #[serde(deserialize_with = "http_serde::header_map::deserialize", default)]
    #[serde(serialize_with = "serialize_headers")]
    pub headers: HeaderMap,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub query_string: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IoTmqttContext {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub client_id: Option<String>,
    pub password: Base64Data,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub username: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IoTtlsContext {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub server_name: Option<String>,
}

/// `IoTCustomAuthorizerResponse` represents the expected format of an IoT device gateway authorization response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IoTCustomAuthorizerResponse {
    pub is_authenticated: bool,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub principal_id: Option<String>,
    pub disconnect_after_in_seconds: i32,
    pub refresh_after_in_seconds: i32,
    pub policy_documents: Vec<String>,
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[test]
    #[cfg(feature = "iot")]
    fn example_iot_custom_auth_request() {
        let data = include_bytes!("fixtures/example-iot-custom-auth-request.json");
        let parsed: IoTCustomAuthorizerRequest = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: IoTCustomAuthorizerRequest =
            serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }

    #[test]
    #[cfg(feature = "iot")]
    fn example_iot_custom_auth_response() {
        let data = include_bytes!("fixtures/example-iot-custom-auth-response.json");
        let parsed: IoTCustomAuthorizerResponse = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: IoTCustomAuthorizerResponse =
            serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
