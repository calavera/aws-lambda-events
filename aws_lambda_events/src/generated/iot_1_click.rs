use crate::custom_serde::*;
use std::collections::HashMap;

/// `IoTOneClickEvent` represents a click event published by clicking button type
/// device.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct IoTOneClickEvent {
    #[serde(rename = "deviceEvent")]
    pub device_event: IoTOneClickDeviceEvent,
    #[serde(rename = "deviceInfo")]
    pub device_info: IoTOneClickDeviceInfo,
    #[serde(rename = "placementInfo")]
    pub placement_info: IoTOneClickPlacementInfo,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct IoTOneClickDeviceEvent {
    #[serde(rename = "buttonClicked")]
    pub button_clicked: IoTOneClickButtonClicked,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct IoTOneClickButtonClicked {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "clickType")]
    pub click_type: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "reportedTime")]
    pub reported_time: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct IoTOneClickDeviceInfo {
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    pub attributes: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "deviceId")]
    pub device_id: Option<String>,
    #[serde(rename = "remainingLife")]
    pub remaining_life: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct IoTOneClickPlacementInfo {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "projectName")]
    pub project_name: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "placementName")]
    pub placement_name: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    pub attributes: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    pub devices: HashMap<String, String>,
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[test]
    fn example_iot_1_click_event() {
        let data = include_bytes!("fixtures/example-iot_1_click-event.json");
        let parsed: IoTOneClickEvent = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: IoTOneClickEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
