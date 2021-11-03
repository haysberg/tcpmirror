use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::time::Duration;

fn handle_client(stream: &mut TcpStream) {
    println!("INFO : New request from client {}", stream.peer_addr().unwrap());
    stream.set_read_timeout(Some(Duration::new(1, 0))).expect("set_read_timeout call failed");
    let mut buf = String::new();

    println!("INFO : Got a request !");
    println!("INFO : Set the read timeout successfully. Reading buffer...");
    // stream.write(stream.read(buf));
    let response = match stream.read_to_string(&mut buf){
        Ok(_) => buf,
        Err(_) => buf
    };
    
    if response.contains("HTTP"){
        print!("INFO : The request is an HTTP request. Sending an HTTP response...");
        let http_res = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            response.len(),
            response 
        );

        match stream.write(http_res.as_bytes()){
            Ok(_) => println!("Done."),
            Err(err) => println!("ERROR : {}", err)
        }
    }
    else {
        print!("INFO : Non-HTTP request. Sending raw bytes...");
        match stream.write(response.as_bytes()){
            Ok(_) => println!("Done."),
            Err(err) => println!("ERROR : {}", err)
        }
    }
    
    println!("Done.");
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(&mut stream?);
    }
    Ok(())
}