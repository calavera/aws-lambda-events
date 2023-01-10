/// AWS Lambda event definitions for activemq.
#[cfg(feature = "activemq")]
pub use super::activemq;

/// AWS Lambda event definitions for alb.
#[cfg(feature = "alb")]
pub use super::alb;
/// AWS Lambda event definitions for apigw.
#[cfg(feature = "apigw")]
pub use super::apigw;

/// AWS Lambda event definitions for appsync.
#[cfg(feature = "appsync")]
pub use super::appsync;

/// AWS Lambda event definitions for autoscaling.
#[cfg(feature = "autoscaling")]
pub use super::autoscaling;

/// AWS Lambda event definitions for chime_bot.
#[cfg(feature = "chime_bot")]
pub use super::chime_bot;

/// AWS Lambda event definitions for clientvpn.
#[cfg(feature = "clientvpn")]
pub use super::clientvpn;

/// CloudWatch Events payload
#[cfg(feature = "cloudwatch_events")]
pub use super::cloudwatch_events;

/// AWS Lambda event definitions for cloudwatch_logs.
#[cfg(feature = "cloudwatch_logs")]
pub use super::cloudwatch_logs;

/// AWS Lambda event definitions for code_commit.
#[cfg(feature = "code_commit")]
pub use super::code_commit;

/// AWS Lambda event definitions for codebuild.
#[cfg(feature = "codebuild")]
pub use super::codebuild;

/// AWS Lambda event definitions for codedeploy.
#[cfg(feature = "codedeploy")]
pub use super::codedeploy;

/// AWS Lambda event definitions for codepipeline_cloudwatch.
#[cfg(feature = "codepipeline_cloudwatch")]
pub use super::codepipeline_cloudwatch;

/// AWS Lambda event definitions for codepipeline_job.
#[cfg(feature = "codepipeline_job")]
pub use super::codepipeline_job;

/// AWS Lambda event definitions for cognito.
#[cfg(feature = "cognito")]
pub use super::cognito;

/// AWS Lambda event definitions for config.
#[cfg(feature = "config")]
pub use super::config;

/// AWS Lambda event definitions for connect.
#[cfg(feature = "connect")]
pub use super::connect;

/// AWS Lambda event definitions for dynamodb.
#[cfg(feature = "dynamodb")]
pub use super::dynamodb;

/// AWS Lambda event definitions for ecr_scan.
#[cfg(feature = "ecr_scan")]
pub use super::ecr_scan;

/// AWS Lambda event definitions for firehose.
#[cfg(feature = "firehose")]
pub use super::firehose;

/// AWS Lambda event definitions for iam.
#[cfg(feature = "iam")]
pub use super::iam;

/// AWS Lambda event definitions for iot.
#[cfg(feature = "iot")]
pub use super::iot;

/// AWS Lambda event definitions for iot_1_click.
#[cfg(feature = "iot_1_click")]
pub use super::iot_1_click;

/// AWS Lambda event definitions for iot_button.
#[cfg(feature = "iot_button")]
pub use super::iot_button;

/// AWS Lambda event definitions for iot_deprecated.
#[cfg(feature = "iot_deprecated")]
pub use super::iot_deprecated;

/// AWS Lambda event definitions for kafka.
#[cfg(feature = "kafka")]
pub use super::kafka;

/// AWS Lambda event definitions for kinesis.
#[cfg(feature = "kinesis")]
pub use super::kinesis;

/// AWS Lambda event definitions for kinesis_analytics.
#[cfg(feature = "kinesis_analytics")]
pub use super::kinesis::analytics as kinesis_analytics;

/// AWS Lambda event definitions for lambda_function_urls.
#[cfg(feature = "lambda_function_urls")]
pub use super::lambda_function_urls;

/// AWS Lambda event definitions for lex.
#[cfg(feature = "lex")]
pub use super::lex;

/// AWS Lambda event definitions for rabbitmq.
#[cfg(feature = "rabbitmq")]
pub use super::rabbitmq;

/// AWS Lambda event definitions for s3.
#[cfg(feature = "s3")]
pub use super::s3;

/// AWS Lambda event definitions for s3_batch_job.
#[cfg(feature = "s3")]
pub use super::s3::batch_job as s3_batch_job;

/// AWS Lambda event definitions for ses.
#[cfg(feature = "ses")]
pub use super::ses;

/// AWS Lambda event definitions for SNS.
#[cfg(feature = "sns")]
pub use super::sns;

/// AWS Lambda event definitions for SQS.
#[cfg(feature = "sqs")]
pub use super::sqs;

/// AWS Lambda event definitions for streams.
#[cfg(feature = "streams")]
pub use super::streams;
