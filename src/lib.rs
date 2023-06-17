use native_tls::{Identity, TlsAcceptor};
use std::error::Error;
use std::fs::File;
use std::io::{Read};
use std::net::{TcpListener};
use std::sync::Arc;
use std::thread::{self};

pub mod connection;

pub type Task = fn(connection::Connection) -> Result<(), Box<dyn Error>>;

pub fn run(pkcs12_cert: &str, pkcs12_password: &str, address: &str, port: &str, task: Task) {
    let mut file = File::open(pkcs12_cert).expect(format!("Couldn't open {pkcs12_cert}").as_str());
    let mut identity = vec![];
    file.read_to_end(&mut identity).expect(format!("Failed to read {pkcs12_cert}").as_str());
    let identity = Identity::from_pkcs12(&identity, pkcs12_password).expect(format!("Failed to create identity from {pkcs12_cert}").as_str());
    drop(pkcs12_password);

    let host = format!("{address}:{port}");
    let listener = TcpListener::bind(&host).expect(format!("Failed to bind to {host}").as_str());
    let acceptor = TlsAcceptor::new(identity).expect("Failed to create TlsAcceptor");
    let acceptor = Arc::new(acceptor);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let acceptor = acceptor.clone();
                thread::spawn(move || {
                    let stream = acceptor.accept(stream).unwrap();
                    handle_client(task, stream);
                });
            }
            Err(_) => { println!("Connection failed") }
        }
    }
}

fn handle_client(task: Task, stream: connection::SecureStream) {
    let connection = connection::Connection::new(stream);
    task(connection);
}
