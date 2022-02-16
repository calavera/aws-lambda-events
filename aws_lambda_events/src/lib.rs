#[macro_use]
extern crate serde_derive;
extern crate base64;
extern crate bytes;
extern crate chrono;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
extern crate serde;
#[cfg(test)]
#[macro_use]
extern crate serde_json;
extern crate http;
extern crate http_serde;
#[cfg(not(test))]
extern crate serde_json;
extern crate query_map;

/// AWS Lambda event definitions for alb.
#[cfg(feature = "alb")]
pub mod alb;
/// AWS Lambda event definitions for apigw.
#[cfg(feature = "apigw")]
pub mod apigw;

/// CloudWatch Events payload
#[cfg(feature = "cloudwatch_events")]
pub mod cloudwatch_events;

/// AWS Lambda event definitions for dynamodb.
#[cfg(feature = "dynamodb")]
pub mod dynamodb;

mod custom_serde;
/// Encodings used in AWS Lambda json event values.
pub mod encodings;
/// AWS Lambda event definitions.
pub mod event;

mod generated;
