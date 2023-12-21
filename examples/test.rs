use afire::{
    trace::{self, Level},
    Method, Server,
};
use afire_tls::AfireTls;

fn main() {
    trace::set_log_level(Level::Debug);

    let mut server = Server::<()>::new("localhost", 8080)
        .workers(8)
        .event_loop(AfireTls::new(
            include_bytes!("data/localhost.crt"),
            include_bytes!("data/localhost.key"),
        ));

    server.route(Method::GET, "/", |ctx| {
        ctx.text("Hello, world!").send()?;
        Ok(())
    });

    server.run().unwrap();
}
