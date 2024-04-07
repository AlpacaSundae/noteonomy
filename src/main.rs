use axum::{
    extract::Path, routing::get, Router
};

use std::fs;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/api/note/*name", get(get_note_contents).post(set_note_contents).delete(delete_note))
    .route("/api/note", get(get_note_list))
    .nest_service("/api", ServeFile::new("static/api.html"))
    .nest_service("/", ServeDir::new("static").not_found_service(ServeFile::new("static/404.html")));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Takes in a path to a note within the notes folder
// Returns the contents or provides an error string if it fails
async fn get_note_contents(Path(name): Path<String>) -> String {
    fs::read_to_string(format!("notes/{}", name)).unwrap_or_else( |err| {
        if err.kind() == std::io::ErrorKind::NotFound {
            format!("the note '{}' does not exist!", name)
        } else {
            println!("{err}");
            error_page()
        }
    })
}

async fn set_note_contents() {}
async fn delete_note() {}

async fn get_note_list() -> String {
    String::from("TODO: list all note directory files")
}

fn error_page() -> String {
    String::from("ERROR ERROR ERROR ERROR ERROR ERROR ")
}