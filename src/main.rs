use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write};
use std::str;



// Handles a single client
fn handle_client(mut stream: TcpStream)  {
    //Diffine a buffer to store stream data
    let mut buf = [0; 512];
       
    //read data from tcp client, and store into buffer
    let bytes_read = stream.read(&mut buf);
    //if the length of buf is zero,we will return ok
    match bytes_read {
        Ok(_) =>{ 
            println!("Input stream is ok!");
        }
        Err(_err) => println!("Input stream is error!")
    }

    //print the request
    println!("Input data is {}", String::from_utf8_lossy(&buf[..]));

    //define return data 
    let echo = b"echo: Hello world!".to_vec();
    // convert the message to string
    let s = match str::from_utf8(&echo) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    //format a string 
    let response = format!(
        "{}",
        s
     );
    println!("{}",response);
    // write the message back to the stream
    stream.write(response.as_bytes()).unwrap();
    //clean stream
    stream.flush().unwrap();
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
                thread::spawn(move || {
                handle_client(stream);
                });
                
            }
            
        }
    }
}
