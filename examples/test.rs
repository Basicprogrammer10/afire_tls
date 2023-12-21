use afire::{
    prelude::*,
    trace::{self, Level},
};
use afire_tls::AfireTls;

fn main() {
    trace::set_log_level(Level::Debug);

    let mut server = Server::<()>::new("localhost", 8080);
    server.event_loop = Box::new(AfireTls::new(
        include_bytes!("data/localhost.crt"),
        include_bytes!("data/localhost.key"),
    ));

    server.route(Method::GET, "/", |ctx| {
        ctx.text("Hello, world!").send()?;
        Ok(())
    });

    server.run().unwrap();
}
