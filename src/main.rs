/**
 * Libraries import.
 * Please note that all of those are default Rust libraries
 * as noted by the std:: prefix
 */
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::time::Duration;

/**
 * handle_client() as the name implies processes
 * the different TcpStreams generated when a client connects
 * to our server.
 */
fn handle_client(stream: &mut TcpStream) {

    //We print the client IP address and port
    println!("INFO : New request from client {}", stream.peer_addr().unwrap());

    //We set a read timeout. If we don't the server reads incoming bytes indefinitely
    //even when the client is done sending his request
    stream.set_read_timeout(Some(Duration::new(1, 0))).expect("set_read_timeout call failed");
    
    //Printing some infos for more lisiblity
    println!("INFO : Got a request !");
    println!("INFO : Set the read timeout successfully. Reading buffer...");
    
    //We read the bytes received from the client, and put
    //them in a variable of type String
    let mut buf = String::new();
    let response = match stream.read_to_string(&mut buf){
        Ok(_) => buf,
        Err(_) => buf
    };
    
    //If the request contains "HTTP" it most likely means
    //that the request was done through an HTTP client.
    //We send an HTTP response instead of a raw TCP response.
    //If we don't it's recognized as HTTP 0.9 and gives us an error message in a lot of cases
    if response.contains("HTTP/"){
        print!("INFO : The request is an HTTP request. Sending an HTTP response...");

        //We format the response to be HTTP compliant.
        let http_res = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            response.len(),
            response 
        );

        //This is just error handling. Just in case we can't send our data
        //for whatever reason
        match stream.write(http_res.as_bytes()){
            Ok(_) => println!("Done."),
            Err(err) => println!("ERROR : {}", err)
        }
    }

    //If the request is not HTTP, for example using telnet
    else {
        print!("INFO : Non-HTTP request. Sending raw bytes...");
        match stream.write(response.as_bytes()){
            Ok(_) => println!("Done."),
            Err(err) => println!("ERROR : {}", err)
        }
    }
    
    println!("================REQUEST================");
    println!("{}", response);
    println!("=======================================");
    //Another line of feedback
    println!("Done.");
}

/**
 * Main Function.
 * Listens on localhost port 8080 and gives the incoming
 * TcpStream objects to the handle_client() function.
 */
fn main() -> std::io::Result<()> {
    //We create a new port bind
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("TCP Mirror server is up !");

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(&mut stream?);
    }
    Ok(())
}