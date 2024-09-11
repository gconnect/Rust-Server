use axum::extract::Path;
use axum::{
    body::Body,
    http::{header, StatusCode},
    response::{Html, IntoResponse},
    routing::get,
    Router,
  };
  use std::{fs, io, net::SocketAddr, path::PathBuf};
  use tokio::fs::read_dir;
  use dirs::home_dir;
mod serve_file;

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(serve_file::serve_base_directory))
    // .route("/expand/*path", get(serve_file::expand_directory))
    .route("/*path", get(serve_file::serve_directory));
    // .route("/read/*path", get(serve_file::read_file));


        // Run the server
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        println!("Listening on http://{}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
}


