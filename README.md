# simple-socket

![build workflow](https://github.com/lockhart9/simple-socket/actions/workflows/rust.yml/badge.svg)

Socket programming exercise using Rust.
Simple echo server.


## How to build & run

Build.

```
$ cargo build --release
```

Run server.

```
$ ./target/release/server --port 9009
```

Run client. This client sends 'hello' to the server and prints the bytes returned from the server.
Make sure the port you specify matches the server port.

```
$ ./target/release/client --host localhost --port 9009
```
