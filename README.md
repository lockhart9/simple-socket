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

## Todo

- [ ] Make client be able to send input text from stdin.
- [ ] Server should handle connection by threading.


## Example

https://user-images.githubusercontent.com/25946425/167276658-21a64b28-b348-4607-9c56-3e2d9333c861.mov


