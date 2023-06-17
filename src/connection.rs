use native_tls::TlsStream;
use std::io::{Read, Write};
pub type SecureStream = TlsStream<TcpStream>;

use std::net::TcpStream;

pub struct Connection {
    stream: SecureStream
}

impl Connection {
    pub fn new(stream: SecureStream) -> Connection {
        Connection { stream: stream }
    }

    pub fn read_line(&mut self) -> String {
        let mut buf = [0; 512];
        let nbytes = self.stream.read(&mut buf).expect("Failed to read from stream");
        let nbytes = if nbytes > buf.len() { buf.len() } else { nbytes };
        String::from_utf8_lossy(&buf[..nbytes]).into()
    }

    pub fn write_line(&mut self, message: &str) {
        self.stream.write(message.as_bytes()).expect("Failed to send data to client");
    }

    pub fn close(&mut self) {
        self.stream.shutdown().expect("Failed to shut down stream");
    }
}