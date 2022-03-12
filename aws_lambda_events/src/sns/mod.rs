use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap};


/// The `Event` stream event handled to Lambda
/// http://docs.aws.amazon.com/lambda/latest/dg/eventsources.html#eventsources-ddb-update
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Event {
    #[serde(rename = "Records")]
    pub records: Vec<EventRecord>,
}

/// EventRecord stores information about each record of a SNS event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Attribute {
    #[serde(rename = "Type")]
    pub attribute_type: String,
    #[serde(rename = "Value")]
    pub value: String,
}

/// EventRecord stores information about each record of a SNS event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct EventRecord {
    pub event_source: String,
    pub event_version: String,
    pub event_subscription_arn: String,
    pub sns: SnsMessage,
}

/// SnsMessage stores information about each record of a SNS event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SnsMessage {
    #[serde(rename = "Type")]
    pub sns_message_type: String,
    pub message_id: String,
    pub topic_arn: String,
    pub subject: String,
    pub timestamp: DateTime<Utc>,
    pub signature_version: String,
    pub signature: String,
    pub signing_cert_url: String,
    pub unsubscribe_url: String,
    pub message: String,
    pub message_attributes: HashMap<String, Attribute>,
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[test]
    #[cfg(feature = "sns")]
    fn my_example_sns_event() {
        let data = include_bytes!("../generated/fixtures/example-sns-event.json");
        let parsed: Event = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: Event = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }

    // #[test]
    // #[cfg(feature = "sns")]
    // fn example_cloudwatch_alarm_sns_payload_multiple_metrics() {
    //     let data =
    //         include_bytes!("fixtures/example-cloudwatch-alarm-sns-payload-multiple-metrics.json");
    //     let parsed: SnsEvent = serde_json::from_slice(data).unwrap();
    //     let output: String = serde_json::to_string(&parsed).unwrap();
    //     let reparsed: SnsEvent = serde_json::from_slice(output.as_bytes()).unwrap();
    //     assert_eq!(parsed, reparsed);
    // }

    // #[test]
    // #[cfg(feature = "sns")]
    // fn example_cloudwatch_alarm_sns_payload_single_metric() {
    //     let data =
    //         include_bytes!("fixtures/example-cloudwatch-alarm-sns-payload-single-metric.json");
    //     let parsed: SnsEvent = serde_json::from_slice(data).unwrap();
    //     let output: String = serde_json::to_string(&parsed).unwrap();
    //     let reparsed: SnsEvent = serde_json::from_slice(output.as_bytes()).unwrap();
    //     assert_eq!(parsed, reparsed);
    // }
}
