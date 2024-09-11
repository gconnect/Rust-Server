
mod serve_file;

fn main() {
    // let app = Router::new()
    // .route("/", get(serve_file::serve_base_directory))
    // // .route("/expand/*path", get(serve_file::expand_directory))
    // .route("/*path", get(serve_file::serve_directory));
    // // .route("/read/*path", get(serve_file::read_file));


    //     // Run the server
    //     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    //     println!("Listening on http://{}", addr);
    //     axum::Server::bind(&addr)
    //         .serve(app.into_make_service())
    //         .await
    //         .unwrap();
    serve_file::serve();

}


