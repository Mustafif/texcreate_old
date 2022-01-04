
use rocket::{Response, Request, get, routes};
use rocket::fs::{FileServer, NamedFile, relative};
use std::path::{Path, PathBuf};
use rocket::{Rocket, Build};
use crate::Web::tex_create;

#[get("/")]
pub async fn index() -> Option<NamedFile>{
    NamedFile::open("index.html").await.ok()
}

pub fn texweb() -> Rocket<Build> {
    rocket::build()
    .mount("/", routes![index, tex_create])
}