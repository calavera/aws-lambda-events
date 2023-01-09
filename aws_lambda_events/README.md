# [aws_lambda_events](https://github.com/calavera/aws-lambda-events)

[![Documentation](https://docs.rs/aws_lambda_events/badge.svg)](https://docs.rs/aws_lambda_events)

This crate provides strongly-typed [AWS Lambda event structs](https://docs.aws.amazon.com/lambda/latest/dg/invoking-lambda-function.html) in Rust.

## Installation

Include the crate in your `Cargo.toml`:

```toml
[dependencies]
aws_lambda_events = "^0.7"
```

## Usage

The crate itself has no AWS Lambda handler logic and instead exists to serialize
and deserialize AWS Lambda events into strongly-typed Rust structs.

The types
defined in this crate are usually used with handlers / runtimes provided by the [official Rust runtime](https://github.com/awslabs/aws-lambda-rust-runtime).

For a list of supported AWS Lambda events and services, see [the crate reference documentation](https://docs.rs/aws_lambda_events).

## Conditional compilation of features

This crate divides all Lambda Events into features named after the service that the events are generated from. By default all events are enabled when you include this crate as a dependency to your project. If you only want to import specific events from this crate, you can disable the default features, and enable only the events that you need. This will make your project to compile a little bit faster, since rustc doesn't need to compile events that you're not going to use. Here's an example on how to do that:

```toml
[dependencies]
aws_lambda_events = { version = "^0.7", default-features = false, features = ["apigw", "alb"] }
```
