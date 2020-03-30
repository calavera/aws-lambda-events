use super::custom_serde::*;
use chrono::{DateTime, Duration, Utc};
use std::ops::{Deref, DerefMut};

/// Binary data encoded in base64.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Base64Data(
    #[serde(deserialize_with = "deserialize_base64")]
    #[serde(serialize_with = "serialize_base64")]
    pub Vec<u8>,
);

impl Deref for Base64Data {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Base64Data {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Timestamp with millisecond precision.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct MillisecondTimestamp(
    #[serde(deserialize_with = "deserialize_milliseconds")]
    #[serde(serialize_with = "serialize_milliseconds")]
    pub DateTime<Utc>,
);

impl Deref for MillisecondTimestamp {
    type Target = DateTime<Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MillisecondTimestamp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Timestamp with second precision.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SecondTimestamp(
    #[serde(deserialize_with = "deserialize_seconds")]
    #[serde(serialize_with = "serialize_seconds")]
    pub DateTime<Utc>,
);

impl Deref for SecondTimestamp {
    type Target = DateTime<Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SecondTimestamp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Duration with second precision.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SecondDuration(
    #[serde(deserialize_with = "deserialize_duration_seconds")]
    #[serde(serialize_with = "serialize_duration_seconds")]
    pub Duration,
);

impl Deref for SecondDuration {
    type Target = Duration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SecondDuration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Duration with minute precision.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct MinuteDuration(
    #[serde(deserialize_with = "deserialize_duration_minutes")]
    #[serde(serialize_with = "serialize_duration_minutes")]
    pub Duration,
);

impl Deref for MinuteDuration {
    type Target = Duration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MinuteDuration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
