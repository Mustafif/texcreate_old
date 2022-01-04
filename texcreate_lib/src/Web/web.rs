
use rocket::{Response, Request, get, routes};
use rocket::fs::{FileServer, NamedFile, relative};
use std::path::{Path, PathBuf};
use rocket::{Rocket, Build};
use crate::Web::{tex_create, tc_html};


#[get("/")]
pub async fn index() -> Option<NamedFile>{
    let index = Path::new("index.html");
    if !index.exists(){
        std::fs::File::create(&index).unwrap();
        std::fs::write(&index, tc_html.as_bytes()).unwrap();
    }
    NamedFile::open("index.html").await.ok()
}

pub fn texweb() -> Rocket<Build> {
    rocket::build()
    .mount("/", routes![index, tex_create])
}