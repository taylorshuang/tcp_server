use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write, Error};

// Handles a single client
fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    //output incoming connection information
    println!("Incoming connection from: {}", stream.peer_addr()?);
    //define a mut value to store input data
    let mut buf = [0; 512];

    println!("hahah");
    //use a loop to read input data
    loop {
        //read data, and store it into buffer
        let bytes_read = stream.read(&mut buf)?;
        println!("{:?}", bytes_read);
        //judge whether success to read 
        if bytes_read == 0 { return Ok(()); }
        stream.write(&buf[..bytes_read])?;
    }
}


fn main() {
    //band ip and port, and judge whether the binding is Success.
    let listener = TcpListener::bind("0.0.0.0:8888")
    .expect("Could not bind");
    //fListen for TCP link information
    for stream in listener.incoming() {
    //Build pattern matching for stream
        match stream {
            //if have a err, will emit an event e
            Err(e) => { eprintln!("failed: {}", e) }
            // if ok, will excute handle_client function
            Ok(stream) => {
                // std::thread::spawn will creates a new thread
                thread::spawn(move || {handle_client(stream);});
                
            }
            
        }
    }
}