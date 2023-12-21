# afire_tls

TLS support for the [afire] webserver framework.

The current implementation only supports [rustls](https://github.com/rustls/rustls), but [native-tls](https://github.com/sfackler/rust-native-tls) support is planned.

```rust
let mut server = Server::<()>::new("localhost", 8080)
    .event_loop(AfireTls::new(
        include_bytes!("localhost.crt"),
        include_bytes!("localhost.key"),
    ));

server.route(Method::GET, "/", |ctx| {
    ctx.text("Hello, world!").send()?;
    Ok(())
});

server.run().unwrap();
```
