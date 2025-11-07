#![windows_subsystem="windows"]
extern crate tiny_http;

use tiny_http::{Response, Server};
use std::{fs, thread};

#[link(name="webview")]
extern "C" {
    fn run_webview();
}

fn main() {
    let mut contents_dir = std::env::current_exe().unwrap();
    contents_dir.pop();
    contents_dir.push("assets");
    println!("{}", contents_dir.to_str().unwrap());

    let server = Server::http("127.0.0.1:6967").unwrap();
    let server_dir = contents_dir.clone();
    thread::spawn(move || {
        println!("Starting server at http://127.0.0.1:6967/");
        for request in server.incoming_requests() {
            let url_path = &request.url()[1..];
            let mut file_path = server_dir.clone();
            if url_path.is_empty() {
                file_path.push("index.html");
            } else {
                file_path.push(url_path);
            }

            let response = if file_path.exists() {
                let content = fs::read(&file_path).unwrap();
                Response::from_data(content)
            } else {
                Response::from_string("404 Not Found").with_status_code(404)
            };
            let _ = request.respond(response);
        }
    });

    unsafe {
        run_webview();
    }
}