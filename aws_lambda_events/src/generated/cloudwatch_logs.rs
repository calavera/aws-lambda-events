use crate::custom_serde::*;

/// `CloudwatchLogsEvent` represents raw data from a cloudwatch logs event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudwatchLogsEvent {
    #[serde(rename = "awslogs")]
    pub aws_logs: CloudwatchLogsRawData,
}

/// `CloudwatchLogsRawData` contains gzipped base64 json representing the bulk
/// of a cloudwatch logs event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudwatchLogsRawData {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub data: Option<String>,
}

/// `CloudwatchLogsData` is an unmarshal'd, ungzip'd, cloudwatch logs event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudwatchLogsData {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub owner: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub log_group: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub log_stream: Option<String>,
    pub subscription_filters: Vec<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub message_type: Option<String>,
    pub log_events: Vec<CloudwatchLogsLogEvent>,
}

/// `CloudwatchLogsLogEvent` represents a log entry from cloudwatch logs
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudwatchLogsLogEvent {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub id: Option<String>,
    pub timestamp: i64,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub message: Option<String>,
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[test]
    #[cfg(feature = "cloudwatch_logs")]
    fn example_cloudwatch_logs_event() {
        let data = include_bytes!("fixtures/example-cloudwatch_logs-event.json");
        let parsed: CloudwatchLogsEvent = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: CloudwatchLogsEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
