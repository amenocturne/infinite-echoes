use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

struct FileData {
    content: Vec<u8>,
    mime_type: String,
}

fn handle_client(mut stream: TcpStream, files: &HashMap<String, FileData>) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET /";
    if buffer.starts_with(get) {
        let request_line = String::from_utf8_lossy(&buffer);
        let path_start = request_line.find("GET /").unwrap() + 4;
        let path_end = request_line[path_start..].find(' ').unwrap() + path_start;
        let mut path = &request_line[path_start..path_end];
        if path.starts_with('/') {
            path = &path[1..];
        }

        if let Some(file_data) = files.get(path) {
            let response_header = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
                file_data.mime_type,
                file_data.content.len()
            );
            stream.write_all(response_header.as_bytes()).unwrap();
            stream.write_all(&file_data.content).unwrap();
        } else {
            let response = "HTTP/1.1 404 NOT FOUND\r\n\r\nFile not found";
            stream.write_all(response.as_bytes()).unwrap();
        }
        stream.flush().unwrap();
    }
}

fn read_file_or_panic(path: &str, mime_type: &str) -> FileData {
    let content = fs::read(path).unwrap_or_else(|_| panic!("Failed to read file: {}", path));

    FileData {
        content,
        mime_type: mime_type.to_string(),
    }
}

fn main() {
    let files: HashMap<String, FileData> = vec![
        (
            "index.html".to_string(),
            read_file_or_panic("index.html", "text/html"),
        ),
        (
            "game.wasm".to_string(),
            read_file_or_panic(
                "./target/wasm32-unknown-unknown/release/infinite_echoes.wasm",
                "application/wasm",
            ),
        ),
    ]
    .into_iter()
    .collect();

    let port = 1234;
    let address = format!("localhost:{}", port);
    let listener = TcpListener::bind(&address).unwrap();
    println!("Server running on http://{}", &address);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_client(stream, &files);
    }
}
