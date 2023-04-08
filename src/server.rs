use std::{env, str};
use std::env::current_dir;
use std::net::{TcpListener, TcpStream};
use std::io::{Read};
use std::fs::File;
use crate::http::{Request, Response, StatusCode};
pub struct Server {
    address: String,
}

impl Server {
    // capital Self is an alias for whatever class self is
    pub fn new(address: String) -> Self {
        Server {
            address
        }
    }

    /**
    Main server loop, accepts and replies to connections
    **/
    pub fn run(&self) {
        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, socket_addr)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            match Request::try_from(&buffer[..] ) {
                                Ok(request) => {
                                    Server::fetch(request,stream);
                                },
                                Err(e) =>{}
                            }
                        },
                        Err(e) => {
                            println!("error {}", e);
                        },
                    }
                    println!()
                },
                Err(e) => {
                    // bind error
                    println!("{}", e);
                }
            }
        }
    }

    /**
    Fill a response buffer with a file, and then reply to the tcp stream
    **/
    fn fetch(request: Request,stream: TcpStream) -> std::io::Result<()>{
        let mut response_body = [0; 1024];
        let response = match File::open(request.getPath()) {
            Ok(mut file) => {
                match file.read(&mut response_body) {
                    Ok(_) =>  Response::new(StatusCode::Ok, Some(&response_body)),
                    Err(_) => Response::new(StatusCode::BadRequest, None)
                }
            },
            Err(_) => {
                Response::new(StatusCode::NotFound, None)
            }
        };

        response.reply(stream)
    }

}