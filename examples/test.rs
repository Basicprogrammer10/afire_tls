use afire::{
    prelude::*,
    trace::{self, Level},
};
use afire_tls::AfireTls;

fn main() {
    trace::set_log_level(Level::Debug);

    let mut server = Server::<()>::new("localhost", 8080);
    server.event_loop = Box::new(AfireTls::new(
        include_bytes!("../data/localhost.crt").to_vec(),
        include_bytes!("../data/localhost.key").to_vec(),
    ));

    server.route(Method::GET, "/", |ctx| {
        ctx.text("Hello, world!").send()?;
        Ok(())
    });

    server.run().unwrap();
}
