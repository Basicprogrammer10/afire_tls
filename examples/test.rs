use afire::prelude::*;
use afire_tls::AfireTls;

fn main() {
    let mut server = Server::new("localhost", 8080);

    server.route(Method::GET, "/", |_| Response::new().text("Ello World"));
    AfireTls::new(
        include_bytes!("../data/localhost.crt").to_vec(),
        include_bytes!("../data/localhost.key").to_vec(),
    );
    // .attatch(&mut server);

    server.start().unwrap();
}
