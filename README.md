# "Examine" Client & Service

Practice creating grpc services and clients with buf.build, Rust, and tonic.

## The Task

> You are implementing an API which must analyze an HTTP user agent and make a decision as to whether to block or allow it. If the user agent header is from a Safari browser, it should return a decision to block the request. If it is from a Firefox browser, it should allow the request.
> 
> Create a gRPC client and server in Rust or Go which implements a single endpoint that receives the user agent string, analyzes it, and returns the result. Create a terminal CLI which uses the client to call the API. Include appropriate tests and documentation.

## Built-in Tests

To run the built-in unit, integration, and doc tests, do:

```shell
cargo test
```

- Doc tests are in [`src/service.rs`](https://github.com/CleanCut/examine/blob/main/src/service.rs#L14-L26)
- Unit tests are (also) in [`src/service.rs`](https://github.com/CleanCut/examine/blob/main/src/service.rs#L66-L88)
- Integration tests are in [`tests/integration.rs`](https://github.com/CleanCut/examine/blob/main/tests/integration.rs)

## Protobuf Definitions

The (handwritten) protobuf file is in `proto/examine/v1/examine.proto`. `buf.build` was used to
generate the code.

- [Generated documentation for the `ExamineService` is hosted on a public `buf.build` repo.](https://buf.build/nathanstocks/examine)

## Server

The code for the server binary is in [`src/bin/server.rs`](https://github.com/CleanCut/examine/blob/main/src/bin/server.rs), while the logic for the service it runs
is in the library file [`src/service.rs`](https://github.com/CleanCut/examine/blob/main/src/service.rs).

To run the server you can do:

```shell
cargo run -r --bin server
```

OR...

```shell
cargo build -r
target/release/server
```

To test the server in isolation using `grpcurl`, first run the server, and then run commands like the one below. (Replace `USER-AGENT-STRING` with the actual User-Agent value.)


```shell
grpcurl -plaintext -import-path ./proto -proto proto/examine/v1/examine.proto -d '{"user_agent": "USER-AGENT-STRING"}' 127.0.0.1:8080 examine.v1.ExamineService/Examine
```

Here is an example of supplying an actual Safari User-Agent.

```shell
grpcurl -plaintext -import-path ./proto -proto proto/examine/v1/examine.proto -d '{"user_agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 13_5) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.5 Safari/605.1.15"}' 127.0.0.1:8080 examine.v1.ExamineService/Examine
```

...which outputs:

```json
{
  "action": "ACTION_BLOCK"
}
```

Here is an example of supplying an actual Firefox User-Agent.

```shell
grpcurl -plaintext -import-path ./proto -proto proto/examine/v1/examine.proto -d '{"user_agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 13.5; rv:109.0) Gecko/20100101 Firefox/116.0"}' 127.0.0.1:8080 examine.v1.ExamineService/Examine
```

...which outputs

```json
{
  "action": "ACTION_ALLOW"
}
```

## Client

The client reads in User-Agent string(s) and then outputs either `Allow` or `Block` for each input value, one per line.

To run the client you can do:

```shell
cargo run -r --bin client -- ARGUMENT  # where ARGUMENT is the User-Agent string(s), separated by newlines

# OR (...probably cleaner if you're running the command repeatedly)

cargo build -r
target/release/client ARGUMENT
```

To test the client, first run the server using the instructions [above](#server), and then:

```shell
# For help, pass in `-h`

target/release/client -h

`client` checks user agent string(s)and emits whether to Allow or Block for each one

Usage: client <USER_AGENT>

Arguments:
  <USER_AGENT>  User-Agent string of the browser. Use `-` to indicate reading from stdin instead

Options:
  -h, --help     Print help
  -V, --version  Print version

# The general form is:
target/release/client ARGUMENT # where ARGUMENT is the User-Agent string(s), separated by newlines

# If you would rather pass your User-Agent string(s) via stdin, use `-` as the argument

echo "some user agent" | target/release/client -
echo "user agent string 1\nuser agent string 2" | target/release/client -
target/release/client - < somefile.txt

# Here's an example of passing in an actual Safari User-Agent via positional argument

target/release/client "Mozilla/5.0 (Macintosh; Intel Mac OS X 13_5) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.5 Safari/605.1.15"

Block

# Here's an example of passing in an actual Firefox User-Agent via stdin

echo "Mozilla/5.0 (Macintosh; Intel Mac OS X 13.5; rv:109.0) Gecko/20100101 Firefox/116.0" | target/release/client -

Allow

# Here's an example of reading multiple User-Agents in via a file piped to stdin.
# This file is present in the repository if you would like to use it.

target/release/client - < tests/user_agents.txt 

Block
Allow
```
