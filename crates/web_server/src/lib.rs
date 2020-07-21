use thread_pool::ThreadPool;
use std::net::{TcpListener, TcpStream};
use std::io::{Result, Read, Write};
use std::io::Error;
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;
use std::borrow::Borrow;
use std::ops::Deref;


pub struct WebServer {
    thread_pool: ThreadPool,
    listener: TcpListener,
    uri_map: HashMap<&'static str, fn(TcpStream)>,
}

impl WebServer {
    pub fn new(port: &str) -> Result<WebServer> {
        let port: u32 = port.parse().expect("Pass a number, please");
        if port < 1024 {
            panic!("Port must be more than 1024!")
        }
        let addr = format!("127.0.0.1:{}", port);
        Ok(
            WebServer {
                thread_pool: ThreadPool::new(4),
                listener: TcpListener::bind(addr)?,
                uri_map: HashMap::new(),
            }
        )
    }

    pub fn register_handler(&mut self, uri: &'static str, handler: fn(TcpStream))
    {
        self.uri_map.insert(uri, handler);
    }

    pub fn run(&self) {
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            self.handle_request(stream)
        }
    }

    fn handle_request(&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let st = String::from_utf8_lossy(&buffer);
        let req: Vec<&str> = st.split(" ").collect();
        let uri = req[1];
        let handler = self.uri_map.get(uri);
        match handler {
            Some(func) => {
                let func = *func;
                self.thread_pool.execute(move || {
                    func(stream);
                }
                )
            },
            None => {
                let status = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
                let file = r"G:\Programming\Rust\rest_api_web\404.html";
                let contents = fs::read_to_string(file).unwrap();

                let resp = format!("{}{}", status, contents);
                stream.write(resp.as_bytes()).unwrap();
                stream.flush().unwrap();
            },
        };
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
