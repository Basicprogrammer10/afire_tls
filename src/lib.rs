use std::io::{Cursor, Read, Write};
use std::iter;
use std::sync::Arc;

use afire::{internal::socket_handler::SocketHandler, Server};
use rustls::{Certificate, PrivateKey, ServerConfig, ServerConnection};
use rustls_pemfile::{read_one, Item};
// pub use rustls::I

pub struct AfireTls {
    config: Arc<ServerConfig>,
}

impl AfireTls {
    pub fn new(cert: Vec<u8>, key: Vec<u8>) -> Self {
        // Load Key
        let mut key = Cursor::new(key);
        let real_key = match read_one(&mut key).unwrap().unwrap() {
            Item::RSAKey(key) => key,
            Item::PKCS8Key(key) => key,
            Item::ECKey(key) => key,
            _ => panic!("Invalid Key"),
        };

        // Load Cert
        let mut cert = Cursor::new(cert);
        let real_cert = iter::from_fn(|| read_one(&mut cert).transpose())
            .into_iter()
            .map(|x| match x.unwrap() {
                Item::X509Certificate(cert) => cert,
                _ => panic!("Invalid Certificate"),
            })
            .map(|x| Certificate(x))
            .collect::<Vec<_>>();

        // Build server config
        let config = ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(real_cert, PrivateKey(real_key))
            .expect("Bad Certificate or Key");
        let config = Arc::new(config);

        Self { config }
    }

    pub fn attatch(self, server: &mut Server) {
        let cf1 = self.config.clone();
        let cf2 = self.config.clone();
        let cf3 = self.config.clone();
        let cf4 = self.config;

        server.socket_handler = SocketHandler {
            socket_read: Box::new(move |x, buff| {
                let mut conn = ServerConnection::new(cf1.to_owned()).ok()?;
                conn.read_tls(x).ok()?;
                let out = conn.reader().read(buff).ok();
                conn.process_new_packets().ok();
                out
            }),
            socket_read_exact: Box::new(move |x, buff| {
                let mut conn = ServerConnection::new(cf2.to_owned()).ok()?;
                conn.read_tls(x).ok()?;
                let out = conn.reader().read_exact(buff).ok();
                conn.process_new_packets().ok();
                out
            }),
            socket_flush: Box::new(move |x| {
                let mut conn = ServerConnection::new(cf3.to_owned()).ok()?;
                conn.write_tls(x).ok()?;
                let out = conn.writer().flush().ok();
                conn.process_new_packets().ok();
                out
            }),
            socket_write: Box::new(move |x, y| {
                let mut conn = ServerConnection::new(cf4.to_owned()).ok()?;
                conn.write_tls(x).ok()?;
                let out = conn.writer().write_all(y).ok();
                conn.process_new_packets().ok();
                out
            }),
        };
    }
}
