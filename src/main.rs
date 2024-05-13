mod persistence;

use std::{
    io::{Read, Write}, net::{TcpListener, TcpStream}, str, sync::Arc, thread
};

use crate::persistence::{KeyValueStore, Persistence};

fn handle_client(mut stream: TcpStream, kv_clone: Arc<KeyValueStore>, persistence_clone: Arc<Persistence>) {
    // Handle the client connection here
    println!("Client connected from {}", stream.peer_addr().unwrap());
    // You can add your logic to handle client requests here
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    println!("Client disconnected!");
                    break;
                }
                match str::from_utf8(&buffer[..bytes_read]) {
                    Ok(string) => {
                        println!("String: {}", string);
                    }
                    Err(e) => {
                        println!("Error occurred {}", e);
                    }
                }
                stream.write_all(&buffer[..bytes_read]).unwrap();
                stream.flush().expect("failed to flush!");
            }
            Err(err) => {
                eprintln!("Error reading from socket: {}", err);
                break;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").expect("Failed to bind to the port");
    println!("Listening to port 6397...");

    let kv = Arc::new(KeyValueStore::new());
    let persistence = Arc::new(Persistence::new(kv.clone(), "data.gob".to_string()));
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let kv_clone = Arc::clone(&kv);
                let persistence_clone = Arc::clone(&persistence);
                thread::spawn(move || handle_client(stream, kv_clone, persistence_clone));
            }
            Err(e) => {
                println!("Error accepting the connection {}", e);
            }
        }
    }
}
