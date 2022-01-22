use crate::custom_serde::*;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientVpnConnectionHandlerRequest {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "connection-id")]
    pub connection_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "endpoint-id")]
    pub endpoint_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "common-name")]
    pub common_name: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub username: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "platform")]
    pub os_platform: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "platform-version")]
    pub os_platform_version: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "public-ip")]
    pub public_ip: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "client-openvpn-version")]
    pub client_open_vpn_version: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "schema-version")]
    pub schema_version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientVpnConnectionHandlerResponse {
    pub allow: bool,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "error-msg-on-failed-posture-compliance")]
    pub error_msg_on_failed_posture_compliance: Option<String>,
    #[serde(rename = "posture-compliance-statuses")]
    pub posture_compliance_statuses: Vec<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "schema-version")]
    pub schema_version: Option<String>,
}
