use axum::{
    body::Body,
    extract::Path,
    http::{header, StatusCode},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::{fs, io, net::SocketAddr, path::PathBuf};
use tokio::fs::read_dir;


pub async fn serve_base_directory() -> impl IntoResponse {
    serve_directory(Path("./".to_string())).await
}


// Serve files or directories as HTML
pub async fn serve_directory(Path(path): Path<String>) -> impl IntoResponse {
    let path = PathBuf::from(path.trim_start_matches('/'));

    // Check if the path is a directory
    if path.is_dir() || path.as_os_str().is_empty() {
        match list_directory(&path).await {
            Ok(html) => html.into_response(),
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read directory").into_response(),
        }
    } else if path.is_file() {
        // This reads the content of the files
        match tokio::fs::read(&path).await {
            Ok(contents) => (
                [(header::CONTENT_TYPE, mime_guess::from_path(&path).first_or_octet_stream().as_ref())],
                contents,
            )
                .into_response(),
            Err(_) => (StatusCode::NOT_FOUND, "File not found").into_response(),
        }
    } else {
        (StatusCode::NOT_FOUND, "Not found").into_response()
    }
}

// This displays an HTML page with a list of directories and files
async fn list_directory(path: &PathBuf) -> io::Result<Html<String>> {
    let mut html = String::new();
    // html.push_str("<h1>Directory listing</h1><ul>");
    html.push_str(r#"
    <html>
    <head>
        <link rel="stylesheet" type="text/css" href="/static/styles.css">
    </head>
    <body>
        <h1>Directory listing</h1>
        <div id="directory-content">
    "#);
 
    let dir = read_dir(path).await?;
    let mut entries = vec![];

    let root = if path.as_os_str().is_empty() {
        ".".to_string()
    } else {
        path.display().to_string()
    };

    html.push_str(&format!("<li><a href=\"/{}\">{}</a></li>", "..", "Parent Directory"));
    //     if !path.as_os_str().is_empty() {
    //     html.push_str(&format!(
    //         r#"<div class="directory-card"><strong><a href="javascript:void(0);" onclick="expandDirectory('{}')">
    //         <img src="https://img.icons8.com/fluent/48/000000/opened-folder.png" class="directory-icon" />Parent Directory</a></strong></div>"#,
    //         ".."
    //     ));
    // }

    let mut dir = dir;
    while let Some(entry) = dir.next_entry().await? {
        let entry_name = entry.file_name();
        let file_name = entry_name.to_string_lossy().into_owned();
        let href = format!("{}/{}", root, file_name);
        entries.push(format!("<li><a href=\"/{href}\">{file_name}</a></li>"));
    }


    entries.sort();
    html.push_str(&entries.join(""));
    html.push_str("</ul>");

    Ok(Html(html))
}
