//! TexCreate Web Library <br>
//! This library is intended to serve a local web server of texcreate-web <br>
//! Developer: Mustafif Khan <br>
//! Project: TexCreate  <br>
//! License: MIT & GPLv2 <br>


/// The html raw string for the homepage
pub mod index;
/// Contains the creation of zipping files and processing the post request
pub mod web;

pub use index::INDEX;
use rocket::fs::NamedFile;
use rocket::{get, routes};
use rocket::{Build, Rocket};
use std::fs::File;
use std::io::Write;
use tempdir::TempDir;
pub use web::*;


/// Serves as the home page get request
#[get("/")]
pub async fn texc_home() -> Option<NamedFile> {
    let temp_dir = TempDir::new("html_temp").unwrap();
    let path = temp_dir.into_path().join("texcreate.html");

    let mut file = File::create(&path).unwrap();
    file.write_all(INDEX.as_bytes()).unwrap();

    NamedFile::open(&path.as_path()).await.ok()
}

/// Creates the web server, launch using `texweb().launch()`
pub fn texweb() -> Rocket<Build> {
    rocket::build().mount("/", routes![texc_home, texc_post])
}
