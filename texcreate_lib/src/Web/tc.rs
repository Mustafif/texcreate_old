use std::io::Write;
use rocket::post;
use zip::write::{FileOptions, ZipWriter};
use zip::CompressionMethod::Stored;
use rocket::form::{FromForm, Form};
use rocket::fs::NamedFile;
use rocket::futures::AsyncWriteExt;
use crate::{create, Config, Document, Project};
use zip::CompressionMethod;
use crate::Config::consts::{DATE, AUTHOR, TITLE};

#[derive(FromForm)]
pub struct Texcreate<'r>{
    pub author: &'r str,
    pub title: &'r str,
    pub date: &'r str,
    pub project_name: &'r str,
    pub template: &'r str,
    pub paper_size: &'r  str,
    pub font_size: &'r str,
    pub document_class: &'r str
}

#[post("/", data="<input>")]
pub async fn tex_create(input: Form<Texcreate<'_>>) -> Option<NamedFile>{
    let f_path = std::path::Path::new("files");
    if !f_path.exists(){
        std::fs::create_dir(&f_path).unwrap();
    }
    let rand = "files/9999999sisjsj.zip";
    let mut zip = ZipWriter::new(std::fs::File::create(rand).unwrap());
    let options = FileOptions::default().compression_method(Stored);
    let mut path = std::path::Path::new("files").join(&input.project_name);
    std::fs::create_dir(&path).unwrap();
    std::fs::File::create(&path.clone().join("main.tex")).unwrap();
    std::fs::File::create(&path.clone().join("structure.tex")).unwrap();
    let (mut m, s) = crate::load(input.template);
    // Replace step
    let title = format!("\\title{{{}}}", input.title);
    let author = format!("\\author{{{}}}", input.author);
    let date = format!("\\date{{{}}}", input.date);

    m = m.replace(TITLE, &title);
    m = m.replace(AUTHOR, &author);
    m = m.replace(DATE, &date);

    m = m.replace("letterpaper", input.paper_size);
    m = m.replace("11pt", &format!("{}pt", input.font_size));
    m = m.replace("article", input.document_class);
    //Zip step
    zip.start_file(path.clone().join("main.tex").to_str().unwrap(), options).unwrap();
    zip.write_all(m.as_bytes()).unwrap();
    zip.start_file(path.clone().join("structure.tex").to_str().unwrap(), options).unwrap();
    zip.write_all(s.as_bytes()).unwrap();
    zip.finish().unwrap();
    //Remove step
    std::fs::remove_file(&path.clone().join("main.tex")).unwrap();
    std::fs::remove_file(&path.clone().join("structure.tex")).unwrap();
    std::fs::remove_dir(&path).unwrap();
    NamedFile::open(rand).await.ok()
}