use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::thread;
use mime_guess;

pub fn serve() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    println!("Listening on http://127.0.0.1:8000");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
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
    println!("Received request: {}", request); // Log the request

    let (status_line, contents) = if request.starts_with("GET / ") || request.starts_with("GET / HTTP/1.1") {
        serve_directory("./")
    } else {
        let path = request.split_whitespace().nth(1).unwrap_or("/");
        serve_file(path.trim_start_matches('/'))
    };

    let response = format!("HTTP/1.1 {}\r\n\r\n{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

// Serve files or directories based on the request path
fn serve_file(path: &str) -> (&str, String) {
    let path = Path::new(path);

    if path.is_file() {
        match fs::read(&path) {
            Ok(contents) => {
                let mime_type = mime_guess::from_path(&path)
                    .first_or_octet_stream()
                    .to_string(); // Fixed conversion to &str

                (
                    "200 OK",
                    format!(
                        "Content-Type: {}\r\n\r\n{}",
                        mime_type,
                        String::from_utf8_lossy(&contents)
                    ),
                )
            }
            Err(_) => ("404 NOT FOUND", "File not found".to_string()),
        }
    } else {
        ("404 NOT FOUND", "File not found".to_string())
    }
}

// Serve directory as HTML
fn serve_directory(path: &str) -> (&str, String) {
    let path = PathBuf::from(path);

    if path.is_dir() || path.as_os_str().is_empty() {
        let mut html = String::new();
        html.push_str(
            r#"
            <html>
            <head>
                <link rel="stylesheet" type="text/css" href="/static/styles.css">
            </head>
            <body>
                <h1>Directory listing</h1>
                <div id="directory-content">
            "#,
        );

        // Show the parent directory link only if not at the root
        if let Some(parent) = path.parent() {
            let parent_href = parent.to_str().unwrap_or("/");
            html.push_str(&format!(
                "<ul><li><a href=\"{}\">Parent Directory</a></li>",
                parent_href
            ));
        }

        // List directory entries
        let mut entries = vec![];
        match fs::read_dir(&path) {
            Ok(dir_entries) => {
                for entry in dir_entries {
                    match entry {
                        Ok(entry) => {
                            let entry_name = entry.file_name();
                            let file_name = entry_name.to_string_lossy().into_owned();
                            let href = format!("{}", file_name);
                            entries.push(format!("<li><a href=\"/{href}\">{file_name}</a></li>"));
                        }
                        Err(_) => continue, // Skip unreadable entries
                    }
                }
            }
            Err(_) => return ("404 NOT FOUND", "Directory not found".to_string()),
        }

        entries.sort();
        html.push_str(&entries.join(""));
        html.push_str("</ul></div></body></html>");

        ("200 OK", html)
    } else {
        ("404 NOT FOUND", "Directory not found".to_string())
    }
}
