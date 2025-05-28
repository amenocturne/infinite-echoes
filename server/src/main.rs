use std::fs;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

const PORT: u16 = 1234;

#[derive(Clone, Eq, PartialEq)]
enum HotReloading {
    Enabled,
    Disabled,
}

#[derive(Clone, Eq, PartialEq)]
enum MimeType {
    Html,
    JavaScript,
    Wasm,
    Png,
}
impl MimeType {
    fn as_str(&self) -> &'static str {
        match self {
            MimeType::Html => "text/html",
            MimeType::JavaScript => "text/javascript",
            MimeType::Wasm => "application/wasm",
            MimeType::Png => "image/png",
        }
    }
}

#[derive(Clone)]
struct FileData {
    route: String,
    file_path: String,
    content: Vec<u8>,
    mime_type: MimeType,
    hot_reloading: HotReloading,
}

impl FileData {
    fn new(
        route: &str,
        file_path: &str,
        mime_type: MimeType,
        hot_reloading: HotReloading,
    ) -> FileData {
        let content = read_file_or_panic(file_path);
        let route = route.to_string();
        let file_path = file_path.to_string();
        FileData {
            route,
            file_path,
            content,
            mime_type,
            hot_reloading,
        }
    }
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
                f.mime_type.as_str(),
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
                let new_f = match f.hot_reloading {
                    HotReloading::Enabled => &FileData {
                        content: read_file_or_panic(&f.file_path.to_string()),
                        ..f.clone()
                    },
                    HotReloading::Disabled => f,
                };
                ok_response(new_f)
            })
            .unwrap_or(not_found_response.into());

        stream.write_all(&response).unwrap();
    };
    stream.flush().unwrap();
}

fn read_file_or_panic(path: &str) -> Vec<u8> {
    fs::read(path).unwrap_or_else(|_| panic!("Failed to read file: {}", path))
}

fn main() {
    let files: Vec<FileData> = vec![
        FileData::new(
            "/",
            "./web/index.html",
            MimeType::Html,
            HotReloading::Enabled,
        ),
        FileData::new(
            "/miniquad_runtime.js",
            "./web/miniquad_runtime.js",
            MimeType::JavaScript,
            HotReloading::Disabled,
        ),
        FileData::new(
            "/game_bg.wasm",
            "./dist/game_bg.wasm",
            MimeType::Wasm,
            HotReloading::Enabled,
        ),
        FileData::new(
            "/game.js",
            "./dist/game.js",
            MimeType::JavaScript,
            HotReloading::Enabled,
        ),
        FileData::new(
            "/resources/piano.png",
            "./resources/piano.png",
            MimeType::Png,
            HotReloading::Enabled,
        ),
        FileData::new(
            "/resources/sine.png",
            "./resources/sine.png",
            MimeType::Png,
            HotReloading::Enabled,
        ),
    ];

    let address = format!("localhost:{}", PORT);
    let listener = TcpListener::bind(&address).unwrap();
    println!("Server running on http://{}", &address);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_client(stream, &files);
    }
}
