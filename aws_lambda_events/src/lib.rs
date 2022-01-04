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

mod custom_serde;
/// Encodings used in AWS Lambda json event values.
pub mod encodings;
/// AWS Lambda event definitions.
pub mod event;
/// CloudWatch Events payload
pub mod cloudwatch_events;

mod generated;
