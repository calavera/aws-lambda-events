use crate::custom_serde::*;
use chrono::{DateTime, Utc};
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SnsEvent {
    #[serde(rename = "Records")]
    pub records: Vec<SnsEventRecord>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SnsEventRecord {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "EventVersion")]
    pub event_version: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "EventSubscriptionArn")]
    pub event_subscription_arn: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "EventSource")]
    pub event_source: Option<String>,
    #[serde(rename = "Sns")]
    pub sns: SnsEntity,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SnsEntity<T1 = Value>
where
    T1: DeserializeOwned,
    T1: Serialize,
{
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "Signature")]
    pub signature: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "MessageId")]
    pub message_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "Type")]
    pub type_: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "TopicArn")]
    pub topic_arn: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(bound = "")]
    #[serde(rename = "MessageAttributes")]
    pub message_attributes: HashMap<String, T1>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "SignatureVersion")]
    pub signature_version: Option<String>,
    #[serde(rename = "Timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "SigningCertUrl")]
    pub signing_cert_url: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "Message")]
    pub message: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "UnsubscribeUrl")]
    pub unsubscribe_url: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "Subject")]
    pub subject: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CloudWatchAlarmSnsPayload {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "AlarmName")]
    pub alarm_name: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "AlarmDescription")]
    pub alarm_description: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "AWSAccountId")]
    pub aws_account_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "NewStateValue")]
    pub new_state_value: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "NewStateReason")]
    pub new_state_reason: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "StateChangeTime")]
    pub state_change_time: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "Region")]
    pub region: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "AlarmArn")]
    pub alarm_arn: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "OldStateValue")]
    pub old_state_value: Option<String>,
    #[serde(rename = "Trigger")]
    pub trigger: CloudWatchAlarmTrigger,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CloudWatchAlarmTrigger {
    #[serde(rename = "Period")]
    pub period: i64,
    #[serde(rename = "EvaluationPeriods")]
    pub evaluation_periods: i64,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "ComparisonOperator")]
    pub comparison_operator: Option<String>,
    #[serde(rename = "Threshold")]
    pub threshold: f64,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "TreatMissingData")]
    pub treat_missing_data: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "EvaluateLowSampleCountPercentile")]
    pub evaluate_low_sample_count_percentile: Option<String>,
    #[serde(rename = "Metrics")]
    pub metrics: Option<Vec<CloudWatchMetricDataQuery>>,
    #[serde(rename = "MetricName")]
    pub metric_name: Option<String>,
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,
    #[serde(rename = "StatisticType")]
    pub statistic_type: Option<String>,
    #[serde(rename = "Statistic")]
    pub statistic: Option<String>,
    #[serde(rename = "Unit")]
    pub unit: Option<String>,
    #[serde(rename = "Dimensions")]
    pub dimensions: Option<Vec<CloudWatchDimension>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CloudWatchMetricDataQuery {
    #[serde(rename = "Expression")]
    pub expression: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[serde(rename = "Label")]
    pub label: Option<String>,
    #[serde(rename = "MetricStat")]
    pub metric_stat: Option<CloudWatchMetricStat>,
    #[serde(rename = "Period")]
    pub period: Option<i64>,
    #[serde(rename = "ReturnData")]
    pub return_data: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CloudWatchMetricStat {
    #[serde(rename = "Metric")]
    pub metric: CloudWatchMetric,
    #[serde(rename = "Period")]
    pub period: i64,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "Stat")]
    pub stat: Option<String>,
    #[serde(rename = "Unit")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CloudWatchMetric {
    #[serde(rename = "Dimensions")]
    pub dimensions: Option<Vec<CloudWatchDimension>>,
    #[serde(rename = "MetricName")]
    pub metric_name: Option<String>,
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CloudWatchDimension {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub name: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub value: Option<String>,
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[test]
    fn example_event() {
        let data = include_bytes!("fixtures/example-sns-event.json");
        let parsed: SnsEvent = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: SnsEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
