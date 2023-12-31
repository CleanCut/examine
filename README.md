# "Examine" Client & Service

Practice creating grpc services and clients with buf.build, Rust, and tonic.

# The Task

> You are implementing an API which must analyze an HTTP user agent and make a decision as to whether to block or allow it. If the user agent header is from a Safari browser, it should return a decision to block the request. If it is from a Firefox browser, it should allow the request.
> 
> Create a gRPC client and server in Rust or Go which implements a single endpoint that receives the user agent string, analyzes it, and returns the result. Create a terminal CLI which uses the client to call the API. Include appropriate tests and documentation.

# Built-in Tests

To run the built-in unit, integration, and doc tests, do:

```shell
cargo test
```

- Doc tests are in [`src/service.rs`](https://github.com/CleanCut/examine/blob/main/src/service.rs#L14-L26)
- Unit tests are (also) in [`src/service.rs`](https://github.com/CleanCut/examine/blob/main/src/service.rs#L66-L88)
- Integration tests are in [`tests/integration.rs`](https://github.com/CleanCut/examine/blob/main/tests/integration.rs)

# Protobuf Definitions

The (handwritten) protobuf file is in `proto/examine/v1/examine.proto`. `buf.build` was used to
generate the code.

- [Generated documentation for the `ExamineService` is hosted on a public `buf.build` repo.](https://buf.build/nathanstocks/examine)

# Rust Library Documentation

_Most_ of the documentation for this project is right here in this readme, but there is also some Rust API documentation available. View it by running the command locally:

```shell
cargo doc --no-deps --open
```

# Server

The code for the server binary is in [`src/bin/server.rs`](https://github.com/CleanCut/examine/blob/main/src/bin/server.rs), while the logic for the service it runs
is in the library file [`src/service.rs`](https://github.com/CleanCut/examine/blob/main/src/service.rs).

### Running the server...

```shell
cargo run -r --bin server
```

OR...

```shell
cargo build -r
target/release/server
```

### Testing the server...

To test the server in isolation using `grpcurl`, first run the server, and then run commands like the one below. (Replace `USER-AGENT-STRING` with the actual User-Agent value.)

```shell
grpcurl -plaintext -import-path ./proto -proto proto/examine/v1/examine.proto -d '{"user_agent": "USER-AGENT-STRING"}' 127.0.0.1:8080 examine.v1.ExamineService/Examine
```

### Safari User-Agent Example

```shell
grpcurl -plaintext -import-path ./proto -proto proto/examine/v1/examine.proto -d '{"user_agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 13_5) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.5 Safari/605.1.15"}' 127.0.0.1:8080 examine.v1.ExamineService/Examine
```

...which outputs:

```json
{
  "action": "ACTION_BLOCK"
}
```

### Firefox User-Agent Example

```shell
grpcurl -plaintext -import-path ./proto -proto proto/examine/v1/examine.proto -d '{"user_agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 13.5; rv:109.0) Gecko/20100101 Firefox/116.0"}' 127.0.0.1:8080 examine.v1.ExamineService/Examine
```

...which outputs

```json
{
  "action": "ACTION_ALLOW"
}
```

# Client

The client reads in User-Agent string(s) and then outputs either `Allow` or `Block` for each input value, one per line.

### Running the client

While you can run the client through cargo, it's a bit...verbose. (`ARGUMENT` is the User-Agent string(s), separated by newlines)

```shell
cargo run -r --bin client -- ARGUMENT
```

It's probably easier to run the client by calling the binary directly:

```shell
cargo build -r
target/release/client ARGUMENT
```

### Client Usage

> Note: First run the server using the instructions [above](#server)!

For the client help output, pass in `-h`:

```shell
target/release/client -h
```

Which should output something like:

```text
`client` checks user agent string(s)and emits whether to Allow or Block for each one

Usage: client <USER_AGENT>

Arguments:
  <USER_AGENT>  User-Agent string of the browser. Use `-` to indicate reading from stdin instead

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Stdin

If you would rather pass your User-Agent string(s) via `stdin`, use `-` as the argument:

```shell
# One argument via pipe
echo "some user agent" | target/release/client -

# Multiple arguments via pipe
echo "user agent string 1\nuser agent string 2" | target/release/client -

# Any number of arguments via file
target/release/client - < somefile.txt
```

### Safari User-Agent Example (via positional argument)

```shell
target/release/client "Mozilla/5.0 (Macintosh; Intel Mac OS X 13_5) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.5 Safari/605.1.15"
```

...which outputs:

```text
Block
```

### Firefox User-Agent Example (via stdin)

```shell
echo "Mozilla/5.0 (Macintosh; Intel Mac OS X 13.5; rv:109.0) Gecko/20100101 Firefox/116.0" | target/release/client -
```

...which outputs:

```text
Allow
```

### Multiple User-Agents via a file

> This file is present in the repository if you would like to use it.

```shell
target/release/client - < tests/user_agents.txt
```

```text
Block
Allow
```
