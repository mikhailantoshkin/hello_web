use std::{fs, thread};
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::time::Duration;
use web_server::WebServer;


fn main(){
    let mut app = WebServer::new("8888").unwrap();
    app.register_handler("/home", home_handler);
    app.register_handler("/set", set_handler);
    app.run();
}

fn home_handler(mut stream: TcpStream) {
    let status = "HTTP/1.1 200 OK\r\n\r\n";
    let file = r"G:\Programming\Rust\rest_api_web\hello.html";
    let contents = fs::read_to_string(file).unwrap();

    let resp = format!("{}{}", status, contents);

    stream.write(resp.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn set_handler(mut stream: TcpStream) {

}
