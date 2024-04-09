use axum::{
    extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router
};

use std::{fs, io};
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/api/note/*name", get(get_note_contents).put(set_note_contents).delete(delete_note))
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

async fn set_note_contents(Path(name): Path<String>, content: String) -> impl IntoResponse {
    match _set_note_contents(name, content) {
        Ok(status) => {
            if status {
                StatusCode::CREATED
            } else {
                StatusCode::BAD_REQUEST
            }
        },
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }

}

fn _set_note_contents(name: String, contents: String) -> Result<bool, io::Error> {
    if _clean_path(&name) {
        //check if exists, if does, archive or git it
        fs::write(format!("notes/{}", name), contents)?;
        Ok(true)
    } else {
        Ok(false)
    }
}

async fn delete_note(Path(name): Path<String>) -> impl IntoResponse {
    match _delete_note(name) {
        Ok(status) => {
            if status {
                StatusCode::NO_CONTENT
            } else {
                StatusCode::BAD_REQUEST
            }
        },
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

fn _delete_note(name: String) -> Result<bool, io::Error>{
    if _clean_path(&name) {
        fs::remove_file(format!("notes/{}", name))?;
        Ok(true)
    } else {
        Ok(false)
    }
}

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
            'a'..='z' => (),
            'A'..='Z' => (),
            '0'..='9' => (),
            '/'       => (),
            _         => return false
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{_clean_path, _delete_note, _set_note_contents, get_note_contents};

    #[test]
    fn test_clean_path() {
        assert!(  _clean_path(&String::from("this/is/67/aGOODpath")));
        assert!(! _clean_path(&String::from("this/IS/67/a<>badpath")));

        assert!(  _clean_path(&String::from("")));
        assert!(! _clean_path(&String::from(" ")));
    }

    #[tokio::test]
    async fn note_storage() {
        let filename = String::from("test");
        let contents = String::from("gaben was\nhere\n8892");
 
        assert!(_set_note_contents(filename.clone(), contents.clone()).unwrap());
        assert_eq!(get_note_contents(axum::extract::Path(filename.clone())).await, contents);
        _delete_note(filename.clone()).unwrap();
        assert!(fs::metadata(&filename).is_err());
    }
}