use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use mime_guess;

fn main() {
    serve();
}

pub fn serve() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    println!("Listening on 127.0.0.1:8000");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);
    let path = request.trim().lines().next().unwrap().split_whitespace().nth(1).unwrap_or("/");
    let trimmed_path = path.trim_start_matches('/');

    let (status_line, contents) = match request.starts_with("GET ") {
        true => {
            if trimmed_path.is_empty() {
                serve_directory("./")
            } else {
                serve_file(trimmed_path)
            }
        }
        _ => (String::from("405 Method Not Allowed"), "Only GET requests are allowed".to_string()),
    };
    let response = format!("HTTP/1.1 {}\r\n\r\n{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn serve_file(path: &str) -> (String, String) {
    let current_dir = std::env::current_dir().unwrap();
    let full_path = current_dir.join(path);

    if full_path.is_file() {
        match fs::read(&full_path) {
            Ok(contents) => {
                let mime_type = mime_guess::from_path(&full_path)
                    .first_or_octet_stream()
                    .as_ref()
                    .to_string();
                (
                    "200 OK".to_string(),
                    format!("Content-Type: {}\r\n\r\n{}", mime_type, String::from_utf8_lossy(&contents)),
                )
            }
            Err(_) => ("404 NOT FOUND".to_string(), "File not found".to_string()),
        }
    } else {
        serve_directory(path)
    }
}

fn serve_directory(path: &str) -> (String, String) {
    let full_path = PathBuf::from(path);
    if full_path.is_dir() {
        let mut html = String::new();
        html.push_str(
            r#"
            <html>
            <head>
            <title>Directory Listing</title>
            <link rel="stylesheet" type="text/css" href="/static/styles.css">
            </head>
            <body>
            <h1>Directory listing</h1>
            <ul>
            "#,
        );
        // This handles the root and parent directory links
        let root = if path.is_empty() { "." } else { path };
        if !path.is_empty() {
            html.push_str("<li><a href=\"../\">Parent Directory</a></li>");
        }

        let mut entries = vec![];
        if let Ok(entries_iter) = fs::read_dir(&full_path) {
            for entry in entries_iter {
                if let Ok(entry) = entry {
                    let entry_name = entry.file_name();
                    let file_name = entry_name.to_string_lossy().into_owned();
                    let href = format!("{}/{}", root, file_name);

                    // This adds the file or directory entry to the list
                    entries.push(format!("<li><a href=\"/{}\">{}</a></li>", href, file_name));
                }
            }
        }
        entries.sort();
        html.push_str(&entries.join(""));
        html.push_str("</ul></body></html>");
        ("200 OK".to_string(), html)
    } else {
        ("404 NOT FOUND".to_string(), "Directory not found".to_string())
    }
}