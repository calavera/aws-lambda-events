use crate::custom_serde::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The `Event` notification event handled by Lambda
///
/// [https://docs.aws.amazon.com/lambda/latest/dg/with-sns.html](https://docs.aws.amazon.com/lambda/latest/dg/with-sns.html)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SnsEvent {
    pub records: Vec<SnsRecord>,
}

/// SnsRecord stores information about each record of a SNS event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SnsRecord {
    /// A string containing the event source.
    pub event_source: String,

    /// A string containing the event version.
    pub event_version: String,

    /// A string containing the event subscription ARN.
    pub event_subscription_arn: String,

    /// An SNS object representing the SNS message.
    pub sns: SnsMessage,
}

/// SnsMessage stores information about each record of a SNS event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SnsMessage {
    /// The type of SNS message. For a lambda event, this should always be **Notification**
    #[serde(rename = "Type")]
    pub sns_message_type: String,

    /// A Universally Unique Identifier, unique for each message published. For a notification that Amazon SNS resends during a retry, the message ID of the original message is used.
    pub message_id: String,

    /// The Amazon Resource Name (ARN) for the topic that this message was published to.
    pub topic_arn: String,

    /// The Subject parameter specified when the notification was published to the topic.
    ///
    /// The SNS Developer Guide states: *This is an optional parameter. If no Subject was specified, then this name-value pair does not appear in this JSON document.*
    ///
    /// Preliminary tests show this appears in the lambda event JSON as `Subject: null`, marking as Option with need to test additional scenarios
    #[serde(deserialize_with = "deserialize_lambda_string")]
    pub subject: Option<String>,

    /// The time (UTC) when the notification was published.
    pub timestamp: DateTime<Utc>,

    /// Version of the Amazon SNS signature used.
    pub signature_version: String,

    /// Base64-encoded SHA1withRSA signature of the Message, MessageId, Subject (if present), Type, Timestamp, and TopicArn values.
    pub signature: String,

    /// The URL to the certificate that was used to sign the message.
    pub signing_cert_url: String,

    /// A URL that you can use to unsubscribe the endpoint from this topic. If you visit this URL, Amazon SNS unsubscribes the endpoint and stops sending notifications to this endpoint.
    pub unsubscribe_url: String,

    /// The Message value specified when the notification was published to the topic.
    pub message: String,

    /// This is a HashMap of defined attributes for a message. Additional details can be found in the [SNS Developer Guide](https://docs.aws.amazon.com/sns/latest/dg/sns-message-attributes.html)
    #[serde(deserialize_with = "deserialize_lambda_map")]
    pub message_attributes: HashMap<String, MessageAttribute>,
}

/// Structured metadata items (such as timestamps, geospatial data, signatures, and identifiers) about the message.
///
/// Message attributes are optional and separate from—but are sent together with—the message body. The receiver can use this information to decide how to handle the message without having to process the message body first.
///
/// Additional details can be found in the [SNS Developer Guide](https://docs.aws.amazon.com/sns/latest/dg/sns-message-attributes.html)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct MessageAttribute {
    /// The data type of the attribute. Per the [SNS Developer Guide](https://docs.aws.amazon.com/sns/latest/dg/sns-message-attributes.html), lambda notifications, this will only be **String** or **Binary**.
    #[serde(rename = "Type")]
    pub data_type: String,

    /// The user-specified message attribute value.
    #[serde(rename = "Value")]
    pub value: String,
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[test]
    #[cfg(feature = "sns")]
    fn my_example_sns_event() {
        let data = include_bytes!("../generated/fixtures/example-sns-event.json");
        let parsed: SnsEvent = serde_json::from_slice(data).unwrap();
        println!("{:?}", parsed);
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: SnsEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
