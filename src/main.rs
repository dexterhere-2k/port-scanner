use std::net::{TcpStream, ToSocketAddrs};
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Duration;

fn main() {
    let target = "127.0.0.1"; 
    let (tx, rx) = channel();

    for i in 1..1024 {
        let tx = tx.clone();
        thread::spawn(move || {
            scan(tx, i, target);
        });
    }

    drop(tx); 

    for open_port in rx {
        println!("Port {} is OPEN", open_port);
    }
}

fn scan(tx: Sender<u16>, port: u16, target: &str) {
    let address = format!("{}:{}", target, port);
    if TcpStream::connect_timeout(&address.to_socket_addrs().unwrap().next().unwrap(), Duration::from_millis(500)).is_ok() {
        tx.send(port).unwrap();
    }
}