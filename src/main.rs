use std::io::{Read, Write};
use std::net::TcpStream;
use ssh2::Session;

fn main() {
    println!("1. Connect to SSH...");
    let tcp = match TcpStream::connect("192.168.31.100:22") {
        Ok(stream) => {
            println!("1.1 Connect successful!");
            stream
        }
        Err(e) => {
            println!("1.1 Connect error: {}", e);
            return;
        }
    };

    let mut sess = match Session::new() {
        Ok(s) => {
            println!("1.2 Create session successful!");
            s
        }
        Err(e) => {
            println!("1.2 Create session error: {}", e);
            return;
        }
    };

    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    match sess.userauth_password("pmk", "chiiiina") {
        Ok(_) => println!("2. Authenticated with login & password!"),
        Err(e) => {
            println!("2. Auth failed: {}", e);
            return;
        }
    }

    if sess.authenticated() {
        println!("2.1 Authenticated successfully!");
    } else {
        println!("2.1 Authentication failed!");
    }
}
