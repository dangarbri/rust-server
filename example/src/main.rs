use std::error::Error;
use server::connection::Connection;

fn task(mut connection: Connection) -> Result<(), Box<dyn Error>> {
    connection.write_line("Hello\n");
    connection.close();
    Ok(())
}

fn main() {
    server::run(
        "certificate.pfx", // Certificate PKCS12 file
        "pfx_password",    // Password for PKCS12
        "0.0.0.0",         // IP address
        "12345",           // Port
        task
    );
}