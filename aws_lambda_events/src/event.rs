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

/// AWS Lambda event definitions for cognito.
#[cfg(feature = "cognito")]
pub use super::cognito;

/// AWS Lambda event definitions for dynamodb.
#[cfg(feature = "dynamodb")]
pub use super::dynamodb;

/// AWS Lambda event definitions for kinesis.
#[cfg(feature = "kinesis")]
pub use super::kinesis;

/// AWS Lambda event definitions for kinesis_analytics.
#[cfg(feature = "kinesis_analytics")]
pub use super::kinesis::analytics as kinesis_analytics;

/// AWS Lambda event definitions for s3.
#[cfg(feature = "s3")]
pub use super::s3;

/// AWS Lambda event definitions for s3_batch_job.
#[cfg(feature = "s3")]
pub use super::s3::batch_job as s3_batch_job;

/// AWS Lambda event definitions for SNS.
#[cfg(feature = "sns")]
pub use super::sns;

/// AWS Lambda event definitions for SQS.
#[cfg(feature = "sqs")]
pub use super::sqs;
