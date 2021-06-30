use super::http::{Request, Response, StatusCode, ParseError};
use std::io::Read;
use std::net::TcpListener;
use std::convert::TryFrom;

pub struct Server {
    address: String,
}

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_error(&mut self, error: &ParseError) -> Response {
        println!("Failed to parse request: {}", error);
        Response::new(StatusCode::BadRequest, None)
    }
}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Server listening on {}", self.address);
        println!("=====================================");

        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(error) => handler.handle_error(&error),
                            };

                            if let Err(error) = response.send(&mut stream) {
                                println!("Failed to send response: {}", error);
                            }
                        },
                        Err(error) => println!("Failed to read from connection: {}", error),
                    }
                },
                Err(error) => println!("Failed to establish a connection: {}", error)
            }
        }
    }
}
