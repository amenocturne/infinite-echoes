use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use maud::{html, PreEscaped, DOCTYPE};

const PORT: u16 = 1234;

#[derive(Clone)]
struct FileData {
    route: String,
    file_path: Option<String>, // To dynamically reload on each request. If  NONE is set, then would be statically read once
    content: Vec<u8>,
    mime_type: String,
}

fn handle_client(mut stream: TcpStream, files: &Vec<FileData>) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET /";
    if buffer.starts_with(get) {
        let request_line = String::from_utf8_lossy(&buffer);
        let path_start = request_line.find("GET /").unwrap() + 4;
        let path_end = request_line[path_start..].find(' ').unwrap() + path_start;
        let path = &request_line[path_start..path_end];

        let ok_response = |f: &FileData| {
            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
                f.mime_type,
                f.content.len()
            )
            .as_bytes()
            .iter()
            .chain(f.content.iter())
            .cloned()
            .collect::<Vec<u8>>()
        };
        let not_found_response = "HTTP/1.1 404 NOT FOUND\r\n\r\nFile not found";

        let found_file = files.iter().find(|f| f.route == path);

        let response: Vec<u8> = found_file
            .map(|f| {
                let new_f = f
                    .to_owned()
                    .file_path
                    .map(|p| {
                        println!("Reading new content");
                        FileData {
                            content: read_file_or_panic(&p.to_string()),
                            ..f.clone()
                        }
                    })
                    .unwrap_or(f.to_owned());
                ok_response(&new_f)
            })
            .unwrap_or(not_found_response.into());

        stream.write_all(&response).unwrap();
    };
    stream.flush().unwrap();
}

fn read_file_or_panic(path: &str) -> Vec<u8> {
    fs::read(path).unwrap_or_else(|_| panic!("Failed to read file: {}", path))
}

// TODO: add hot reloading support for dev build
fn main() {

    let wasm_file_path = "./dist/game_bg.wasm".to_string();
    let js_file_path = "./dist/game.js".to_string();
    let index_path = "./index.html".to_string();

    let files: Vec<FileData> = vec![
        FileData {
            route: "/".to_string(),
            file_path: Some(index_path.to_owned()),
            content: read_file_or_panic(&index_path),
            mime_type: "text/html".to_string(),
        },
        FileData {
            route: "/game_bg.wasm".to_string(),
            file_path: Some(wasm_file_path.to_owned()),
            content: read_file_or_panic(&wasm_file_path),
            mime_type: "application/wasm".to_string(),
        },
        FileData {
            route: "/game.js".to_string(),
            file_path: Some(js_file_path.to_owned()),
            content: read_file_or_panic(&js_file_path),
            mime_type: "text/javascript".to_string(),
        },
    ];

    let address = format!("localhost:{}", PORT);
    let listener = TcpListener::bind(&address).unwrap();
    println!("Server running on http://{}", &address);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_client(stream, &files);
    }
}
