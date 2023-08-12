# "Examine" Client & Service

Practice creating grpc services and clients with buf.build, Rust, and tonic

## Built-in Tests

To run the built-in unit-, integration-, and doc-tests, do:

```shell
cargo test
```

## Protobuf Definitions

The (handwritten) protobuf file is in `proto/examine/v1/examine.proto`. `buf.build` was used to
generate the code.

[Generated documentation for the `ExamineService` is hosted on a public `buf.build` repo.](https://buf.build/nathanstocks/examine)

## Server

The code for the server binary is in `src/bin/server.rs`, while the logic for the service it runs
is in the library file `src/service.rs`.

To run the server you can do:

```shell
cargo run -r --bin service

# or...

cargo build -r
target/release/server
```

To test the server in isolation using `grpcurl`, first run the server, and then run commands such as:

```shell
# The general form is this (replace USER-AGENT-STRING with the actual value)
grpcurl -plaintext -import-path ./proto -proto proto/examine/v1/examine.proto -d '{"user_agent": "USER-AGENT-STRING"}' 127.0.0.1:8080 examine.v1.ExamineService/Examine

# Here is an example of supplying an actual Safari User-Agent
grpcurl -plaintext -import-path ./proto -proto proto/examine/v1/examine.proto -d '{"user_agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 13_5) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.5 Safari/605.1.15"}' 127.0.0.1:8080 examine.v1.ExamineService/Examine

{
  "action": "ACTION_BLOCK"
}

# Here is an example of supplying an actual Firefox User-Agent
grpcurl -plaintext -import-path ./proto -proto proto/examine/v1/examine.proto -d '{"user_agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 13.5; rv:109.0) Gecko/20100101 Firefox/116.0"}' 127.0.0.1:8080 examine.v1.ExamineService/Examine

{
  "action": "ACTION_ALLOW"
}
```
