pub mod index;
pub mod web;

pub use index::INDEX;
use rocket::fs::NamedFile;
use rocket::{get, routes};
use rocket::{Build, Rocket};
use std::fs::File;
use std::io::Write;
use tempdir::TempDir;
pub use web::*;
#[get("/")]
pub async fn texc_home() -> Option<NamedFile> {
    let temp_dir = TempDir::new("html_temp").unwrap();
    let path = temp_dir.into_path().join("texcreate.html");

    let mut file = File::create(&path).unwrap();
    file.write_all(INDEX.as_bytes()).unwrap();

    NamedFile::open(&path.as_path()).await.ok()
}

pub fn texweb() -> Rocket<Build> {
    rocket::build().mount("/", routes![texc_home, texc_post])
}
