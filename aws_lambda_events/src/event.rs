// Additional layer of indirection in case we want to make upgrading easy
// or provide translations in the future.
pub use super::generated::*;

/// AWS Lambda event definitions for alb.
#[cfg(feature = "alb")]
pub use super::alb;
/// AWS Lambda event definitions for apigw.
#[cfg(feature = "apigw")]
pub use super::apigw;

/// CloudWatch Events payload
#[cfg(feature = "cloudwatch_events")]
pub use super::cloudwatch_events;

/// AWS Lambda event definitions for dynamodb.
#[cfg(feature = "dynamodb")]
pub use super::dynamodb;
