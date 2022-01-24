
use rocket::{get, routes};
use rocket::fs::NamedFile;
use std::path::Path;
use rocket::{Rocket, Build};
use crate::Web::tex_create;


#[get("/")]
pub async fn index() -> Option<NamedFile>{
    let index = Path::new("texcreate.html");
    NamedFile::open("texcreate.html").await.ok()
}

pub fn texweb() -> Rocket<Build> {
    rocket::build()
    .mount("/", routes![index, tex_create])
}