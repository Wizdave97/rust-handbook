use std::net::{TcpStream, TcpListener};
use std::io::Result;
use std::io::prelude::*;
use std::fs;
use web_server::ThreadPool;



fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap_or_else(| err | {
        panic!("Was unable to start server: {}", err)
    });
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        pool.execute(|| {
            handle_connection(stream).unwrap();
        })
        
    }
}

fn handle_connection(stream: Result<TcpStream>) -> Result<()> {
    const HOME: &[u8] = b"GET / HTTP/1.1";
    const SLEEP: &[u8] = b"GET /sleep HTTP/1.1";
    match stream {
        Ok(mut stream) => {
            let mut buffer = [0; 1024];
            if let Ok(_) = stream.read(&mut buffer) {
                
                
                let (status, filename) = if buffer.starts_with(HOME) {
                    ("200 OK", "hello.html")
                }
                else if buffer.starts_with(SLEEP) {
                    std::thread::sleep(std::time::Duration::from_secs(5));
                    ("200 OK", "hello.html")
                }
                else {
                    ("404 NOT FOUND", "404.html")
                };
                
                let html = fs::read_to_string(filename).unwrap();
                let resp = format!("HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}", status, html.len(), html);
                match stream.write(resp.as_bytes()) {
                    Ok(_) => {
                        stream.flush().unwrap();
                    },
                    Err(_) => println!("Response sending failed")
                }

            }
            else {
                println!("Unable to read request")
            }
            
        },
        Err(_) => {

        }
    };
    Ok(())
}
