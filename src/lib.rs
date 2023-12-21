use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;
use std::time::Duration;
use std::{io, net};

use afire::{
    error,
    internal::{
        event_loop::EventLoop,
        handle,
        socket::{Socket, SocketStream, Stream},
    },
    Server,
};
use rustls::{ServerConfig, ServerConnection, StreamOwned};

pub struct AfireTls {
    config: Arc<ServerConfig>,
}

impl AfireTls {
    pub fn new(cert: &[u8], key: &[u8]) -> Self {
        let mut cert = &mut io::Cursor::new(cert);
        let mut key = &mut io::Cursor::new(key);

        let key = rustls_pemfile::private_key(&mut key)
            .expect("failed to load private key")
            .unwrap();
        let certs = rustls_pemfile::certs(&mut cert)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let config = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(certs, key)
            .unwrap();

        Self {
            config: Arc::new(config),
        }
    }
}

impl<State: Send + Sync> EventLoop<State> for AfireTls {
    fn run(&self, server: Arc<Server<State>>, addr: SocketAddr) -> error::Result<()> {
        let listener = TcpListener::bind(addr)?;

        for i in listener.incoming() {
            if !server.running.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }

            let event = match i {
                Ok(event) => event,
                Err(err) => {
                    println!("Error accepting connection: {}", err);
                    continue;
                }
            };

            let connection = ServerConnection::new(self.config.clone()).unwrap();
            let stream = StreamOwned::new(connection, event);
            let event = Arc::new(Socket::new(TlsStream { inner: stream }));

            handle::handle(event, server.clone());
        }

        Ok(())
    }
}

struct TlsStream {
    inner: StreamOwned<ServerConnection, TcpStream>,
}

impl Read for TlsStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

impl Write for TlsStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

impl Stream for TlsStream {
    fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.inner.sock.peer_addr()
    }

    fn try_clone(&self) -> io::Result<SocketStream> {
        todo!()
    }

    fn shutdown(&self, shutdown: net::Shutdown) -> io::Result<()> {
        self.inner.sock.shutdown(shutdown)
    }

    fn set_timeout(&self, duration: Option<Duration>) -> io::Result<()> {
        self.inner.sock.set_read_timeout(duration)?;
        self.inner.sock.set_write_timeout(duration)?;
        Ok(())
    }
}
