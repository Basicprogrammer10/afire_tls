use std::fs;

use afire::prelude::*;
use afire_tls::{AfireTls, Identity};

fn main() {
    let mut server = Server::new("localhost", 8080);

    server.route(Method::GET, "/", |_| Response::new().text("Ello World"));
    AfireTls::new(
        Identity::from_pkcs12(&fs::read("./data/localhost.pfx").unwrap(), "Basking2021!").unwrap(),
    )
    .attatch(&mut server);

    server.start().unwrap();
}
