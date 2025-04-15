use std::io::{Read, Write};
use std::net::TcpStream;
use ssh2::Session;


fn main() {
    // https://docs.rs/ssh2/latest/ssh2/ docs
    println!("1. Connect to SSH...");
    let tcp = match TcpStream::connect("192.168.31.100:22"){
        Ok(stream) => {
            println!("1.1 Connect successful!");
            stream  // Возвращаем поток
        }
        Err(e) => {
            println!("1.1 Connect error {}", e);
            return;
        }
    };


    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    sess.userauth_password("pmk", "chiiiina").unwrap();
    assert!(sess.authenticated());
}