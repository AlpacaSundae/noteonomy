use axum::{
    extract::Path, routing::get, Router,
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
    if _clean_path(&name) {
        fs::read_to_string(format!("notes/{}", name)).unwrap_or_else( |err| {
            if err.kind() == std::io::ErrorKind::NotFound {
                format!("the note '{}' does not exist!\nAvailable notes:\nTODO", name)
            } else {
                println!("{err}");
                error_page()
            }
        })
    } else {
        format!("the path '{}' is invalid. We only accept alphanumericals in these paths", name)
    }
}

async fn set_note_contents() {
    // if file isnot exist, create it
}



async fn delete_note() {}

async fn get_note_list() -> String {
    String::from("TODO: list all note directory files")
}

fn error_page() -> String {
    String::from("ERROR ERROR ERROR ERROR ERROR ERROR ")
}

// Ensure a path has no funky characters in it before trying to access a file
// for now, we only accept alphanumeric paths 
fn _clean_path(name : &String) -> bool {
    for c in name.chars() {
        match c {
            'a'..='z'   => (),
            'A'..='Z'   => (),
            '0'..='9'   => (),
            '/'         => (),
            _           => return false
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use crate::_clean_path;

    #[test]
    fn test_clean_path() {
        assert!(  _clean_path(&String::from("this/is/67/aGOODpath")));
        assert!(! _clean_path(&String::from("this/IS/67/a<>badpath")));

        assert!(  _clean_path(&String::from("")));
        assert!(! _clean_path(&String::from(" ")));
    }
}