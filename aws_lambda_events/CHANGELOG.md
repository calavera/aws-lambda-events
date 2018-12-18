## master

- No changes yet

## [[0.2.0] - 2018-12-18](https://github.com/LegNeato/aws-lambda-events/releases/tag/v0.2.0)

- Add support for embedded fields, as seen in `cognito::CognitoEventUserPoolsPreSignup`. [[#4](https://github.com/LegNeato/aws-lambda-events/pull/4)]

## [[0.1.5] - 2018-12-17](https://github.com/LegNeato/aws-lambda-events/releases/tag/v0.1.5)

- Add `alb` events.

## [[0.1.4] - 2018-12-17](https://github.com/LegNeato/aws-lambda-events/releases/tag/v0.1.4)

- Inner fields of `Base64Data`, `MillisecondTimestamp`, `SecondTimestamp` are
  now public. [[#3](https://github.com/LegNeato/aws-lambda-events/pull/3)]
- Fields encoded as `serde_json::Value` may now optionally use a more-specific
  type. [[#1](https://github.com/LegNeato/aws-lambda-events/pull/1)]
