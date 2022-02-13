use crate::custom_serde::*;
use chrono::{serde::ts_nanoseconds, DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};

pub mod attributes;
use self::attributes::AttributeValue;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StreamViewType {
    NewImage,
    OldImage,
    NewAndOldImages,
    KeysOnly,
}

impl fmt::Display for StreamViewType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = match self {
            StreamViewType::NewImage => "NEW_IMAGE",
            StreamViewType::OldImage => "OLD_IMAGE",
            StreamViewType::NewAndOldImages => "NEW_AND_OLD_IMAGES",
            StreamViewType::KeysOnly => "KEYS_ONLY",
        };
        write!(f, "{}", val)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StreamStatus {
    Enabling,
    Enabled,
    Disabling,
    Disabled,
}

impl fmt::Display for StreamStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = match self {
            StreamStatus::Enabling => "ENABLING",
            StreamStatus::Enabled => "ENABLED",
            StreamStatus::Disabling => "DISABLING",
            StreamStatus::Disabled => "DISABLED",
        };
        write!(f, "{}", val)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SharedIteratorType {
    TrimHorizon,
    Latest,
    AtSequenceNumber,
    AfterSequenceNumber,
}

impl fmt::Display for SharedIteratorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = match self {
            SharedIteratorType::TrimHorizon => "TRIM_HORIZON",
            SharedIteratorType::Latest => "LATEST",
            SharedIteratorType::AtSequenceNumber => "AT_SEQUENCE_NUMBER",
            SharedIteratorType::AfterSequenceNumber => "AFTER_SEQUENCE_NUMBER",
        };
        write!(f, "{}", val)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OperationType {
    Insert,
    Modify,
    Remove,
}

impl fmt::Display for OperationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = match self {
            OperationType::Insert => "INSERT",
            OperationType::Modify => "MODIFY",
            OperationType::Remove => "REMOVE",
        };
        write!(f, "{}", val)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum KeyType {
    Hash,
    Range,
}

impl fmt::Display for KeyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = match self {
            KeyType::Hash => "HASH",
            KeyType::Range => "RANGE",
        };
        write!(f, "{}", val)
    }
}

/// The `Event` stream event handled to Lambda
/// http://docs.aws.amazon.com/lambda/latest/dg/eventsources.html#eventsources-ddb-update
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Event {
    #[serde(rename = "Records")]
    pub records: Vec<EventRecord>,
}

/// EventRecord stores information about each record of a DynamoDb stream event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventRecord {
    /// The region in which the GetRecords request was received.
    pub aws_region: String,
    /// The main body of the stream record, containing all of the DynamoDB-specific
    /// fields.
    #[serde(rename = "dynamodb")]
    pub change: StreamRecord,
    /// A globally unique identifier for the event that was recorded in this stream
    /// record.
    #[serde(rename = "eventID")]
    pub event_id: String,
    /// The type of data modification that was performed on the DynamoDB table:
    ///
    /// * INSERT - a new item was added to the table.
    ///
    /// * MODIFY - one or more of an existing item's attributes were modified.
    ///
    /// * REMOVE - the item was deleted from the table
    pub event_name: String,
    /// The AWS service from which the stream record originated. For DynamoDB Streams,
    /// this is aws:dynamodb.
    pub event_source: String,
    /// The version number of the stream record format. This number is updated whenever
    /// the structure of Record is modified.
    ///
    /// Client applications must not assume that eventVersion will remain at a particular
    /// value, as this number is subject to change at any time. In general, eventVersion
    /// will only increase as the low-level DynamoDB Streams API evolves.
    pub event_version: String,
    /// The event source ARN of DynamoDB
    #[serde(rename = "eventSourceARN")]
    pub event_source_arn: String,
    /// Items that are deleted by the Time to Live process after expiration have
    /// the following fields:
    ///
    /// * Records[].userIdentity.type
    ///
    /// "Service"
    ///
    /// * Records[].userIdentity.principalId
    ///
    /// "dynamodb.amazonaws.com"
    pub user_identity: Option<UserIdentity>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserIdentity {
    #[serde(default)]
    pub type_: String,
    #[serde(default)]
    pub principal_id: String,
}

/// `DynamoDbStreamRecord` represents a description of a single data modification that was performed on an item
/// in a DynamoDB table.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamRecord {
    /// The approximate date and time when the stream record was created, in UNIX
    /// epoch time (http://www.epochconverter.com/) format.
    #[serde(rename = "ApproximateCreationDateTime")]
    #[serde(with = "ts_nanoseconds")]
    pub approximate_creation_date_time: DateTime<Utc>,
    /// The primary key attribute(s) for the DynamoDB item that was modified.
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "Keys")]
    pub keys: HashMap<String, AttributeValue>,
    /// The item in the DynamoDB table as it appeared after it was modified.
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "NewImage")]
    pub new_image: HashMap<String, AttributeValue>,
    /// The item in the DynamoDB table as it appeared before it was modified.
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "OldImage")]
    pub old_image: HashMap<String, AttributeValue>,
    /// The sequence number of the stream record.
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: Option<String>,
    /// The size of the stream record, in bytes.
    #[serde(rename = "SizeBytes")]
    pub size_bytes: i64,
    /// The type of data from the modified DynamoDB item that was captured in this
    /// stream record.
    #[serde(default)]
    #[serde(rename = "StreamViewType")]
    pub stream_view_type: Option<StreamViewType>,
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[test]
    #[cfg(feature = "dynamodb")]
    fn example_dynamodb_event() {
        let data = include_bytes!("../generated/fixtures/example-dynamodb-event.json");
        let parsed: Event = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: Event = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
