# Server
Simple server interface

# Usage
1. Write a function that the server will launch which takes a connection and returns a result.
2. Use server::run to launch the task.

## Example
```rust
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
```

Then connect with a client such as openssl s_client
```bash
$ openssl s_client 127.0.0.1:12345
... verification output ...
---
Hello
closed
```