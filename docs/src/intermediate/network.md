# Network Operations

## Overview

The `std::net` module provides TCP and UDP networking primitives. Create servers and clients for network communication with a simple, synchronous API.

## Code Example

```rust
use std::net::TcpListener;

// Simple TCP listener demonstration
let listener_result = TcpListener::bind("127.0.0.1:0");
match listener_result {
    Ok(listener) => {
        let local_addr = listener.local_addr().unwrap();
        println!("TCP listener bound to: {}", local_addr);
    }
    Err(e) => println!("TCP bind failed: {}", e),
}
```

## Explanation

- **`TcpListener::bind()`**: Binds to IP and port for incoming connections
- **Port `0`**: OS assigns random available port
- **`local_addr()`**: Gets the actual address bound to
- Returns `Result` for error handling

## Key Concepts

### TCP Server
```rust
use std::net::TcpListener;
use std::io::{Read, Write};

let listener = TcpListener::bind("127.0.0.1:8080")?;

for stream in listener.incoming() {
    match stream {
        Ok(mut stream) => {
            let mut buffer = [0; 512];
            stream.read(&mut buffer)?;
            stream.write(b"HTTP/1.1 200 OK\r\n\r\n")?;
        }
        Err(e) => eprintln!("Connection failed: {}", e),
    }
}
```

### TCP Client
```rust
use std::net::TcpStream;
use std::io::{Read, Write};

let mut stream = TcpStream::connect("example.com:80")?;
stream.write(b"GET / HTTP/1.0\r\n\r\n")?;

let mut response = String::new();
stream.read_to_string(&mut response)?;
println!("Response: {}", response);
```

### UDP Sockets
```rust
use std::net::UdpSocket;

let socket = UdpSocket::bind("127.0.0.1:0")?;
socket.send_to(b"Hello", "127.0.0.1:8080")?;

let mut buf = [0; 512];
let (amt, src) = socket.recv_from(&mut buf)?;
println!("Received {} bytes from {}", amt, src);
```

## Use Cases

- **Web Servers**: HTTP/HTTPS servers
- **APIs**: REST or RPC services
- **Chat Applications**: Real-time messaging
- **Game Servers**: Multiplayer games
- **IoT**: Device communication
- **Microservices**: Service-to-service communication

## Best Practices

✅ **Do:**
- Use `0.0.0.0` for public servers, `127.0.0.1` for local only
- Set timeouts with `set_read_timeout()`, `set_write_timeout()`
- Use thread pools or async for handling multiple connections
- Handle partial reads/writes in loops

❌ **Don't:**
- Block indefinitely without timeouts
- Forget to handle connection errors
- Create thread per connection (use thread pools)
- Assume all data arrives in one read

## Advanced Patterns

### Threaded Server
```rust
use std::net::TcpListener;
use std::thread;

let listener = TcpListener::bind("127.0.0.1:8080")?;

for stream in listener.incoming() {
    match stream {
        Ok(stream) => {
            thread::spawn(|| {
                handle_client(stream);
            });
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Connection Timeouts
```rust
use std::time::Duration;
use std::net::TcpStream;

let mut stream = TcpStream::connect("example.com:80")?;

stream.set_read_timeout(Some(Duration::from_secs(5)))?;
stream.set_write_timeout(Some(Duration::from_secs(5)))?;
```

### Non-Blocking I/O
```rust
use std::net::TcpStream;

let stream = TcpStream::connect("127.0.0.1:8080")?;
stream.set_nonblocking(true)?;

// Now read/write operations return immediately
// with WouldBlock error if not ready
```

## Protocol Implementation Example

### Simple Echo Server
```rust
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 512];
    
    loop {
        let n = stream.read(&mut buffer)?;
        if n == 0 {
            return Ok(());  // Connection closed
        }
        stream.write_all(&buffer[..n])?;
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Echo server listening on port 8080");
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    if let Err(e) = handle_client(stream) {
                        eprintln!("Error handling client: {}", e);
                    }
                });
            }
            Err(e) => eprintln!("Connection error: {}", e),
        }
    }
    
    Ok(())
}
```

## IPv4 vs IPv6

```rust
use std::net::{TcpListener, SocketAddr};

// IPv4
let v4 = TcpListener::bind("127.0.0.1:8080")?;

// IPv6
let v6 = TcpListener::bind("[::1]:8080")?;

// Both (dual-stack)
let addr: SocketAddr = "[::]:8080".parse()?;
let dual = TcpListener::bind(addr)?;
```

## Async Alternatives

For production use, consider async runtimes:
- **tokio**: Most popular async runtime
- **async-std**: Async version of std
- **smol**: Lightweight async runtime

```rust
// Example with tokio (not std)
// #[tokio::main]
// async fn main() {
//     let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
//     // ...
// }
```

## Related Features

- [Threading](../basic/threading.md) - Multi-threaded servers
- [Error Handling](../basic/error-handling.md) - Handling network errors

## Learn More

- [std::net documentation](https://doc.rust-lang.org/std/net/)
- [The Rust Book - Building a Multithreaded Web Server](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)
