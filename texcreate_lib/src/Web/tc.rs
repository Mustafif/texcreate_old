use std::io::Write;
use rocket::post;
use zip::write::{FileOptions, ZipWriter};
use zip::CompressionMethod::Stored;
use rocket::form::{FromForm, Form};
use rocket::fs::NamedFile;
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
    let f_path = std::path::Path::new("tc_files");
    if !f_path.exists(){
        std::fs::create_dir(&f_path).unwrap();
    }
    let rand = "tc_files/9999999sisjsj.zip";
    let mut zip = ZipWriter::new(std::fs::File::create(rand).unwrap());
    let options = FileOptions::default().compression_method(Stored);
    let path = std::path::Path::new("tc_files").join(&input.project_name);
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


pub const TC_HTML: &str = r#"<!DOCTYPE html>
<html>
<title>TexCreate</title>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<link rel="stylesheet" href="https://www.w3schools.com/w3css/4/w3.css">
<link rel="stylesheet" href="https://fonts.googleapis.com/css?family=Montserrat">
<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css">
<style>
body, h1,h2,h3,h4,h5,h6 {font-family: "Montserrat", sans-serif}
.w3-row-padding img {margin-bottom: 12px}
/* Set the width of the sidebar to 120px */
.w3-sidebar {width: 120px;background: #222;}
/* Add a left margin to the "page content" that matches the width of the sidebar (120px) */
#main {margin-left: 120px}
/* Remove margins from "page content" on small screens */
@media only screen and (max-width: 600px) {#main {margin-left: 0}}
</style>
<body class="w3-black">
<!-- Page Content -->
<div class="w3-padding-large" id="main">
    <!-- Header/Home -->
    <header class="w3-container w3-padding-32 w3-center w3-black" id="home">
        <h1 class="w3-jumbo"><span class="w3-hide-small">TexCreate</span></h1>
    <!-- <img src="Banner.png" alt="boy" class="w3-image" width="*" height="0.25*"> -->
    </header>

    <!-- About Section -->
    <div class="w3-content w3-justify w3-text-grey w3-padding-64" id="about">
        <h2 class="w3-text-light-grey">BUILD A LaTeX PROJECT</h2>
        <hr style="width:200px" class="w3-opacity">
            <div class="w3-container w3-center">
<form method="post" action="/">
    <h3>Project</h3><br>
    <label for="author">Author</label><br>
    <input  class="w3-teal w3-button" type="text" name="author" id="author" placeholder="Author"><br>
    <label for="title">Title</label><br>
    <input class="w3-teal w3-button" type="text" name="title" id="title" placeholder="Title"><br>
    <label for="date">Date</label><br>
    <input class="w3-teal w3-button" type="text" name="date" id="date" placeholder="Date"><br>
    <label for="project_name">Project Name</label><br>
    <input class="w3-teal w3-button" type="text" name="project_name" id="project_name" placeholder="Project Name"><br>
    <label for="template">Template</label><br>
    <select class="w3-teal w3-button" type="text" name="template" id="template" placeholder="Template">
    <option value="Basic">Basic</option>
    <option value="Math">Math</option>
    <option value="Code">Code</option>
    <option value="Theatre">Theatre</option>
    <option value="Novel">Novel</option>
    <option value="Beamer">Beamer</option>
    <option value="Lachaise">Lachaise</option>
    </select>
    <br>
    <h3>Document</h3>
    <label for="paper_size">Paper Size</label><br>
    <input class="w3-teal w3-button" type="text" name="paper_size" id="paper_size" placeholder="Paper Size"><br>
    <label for="font_size">Font Size</label><br>
    <input class="w3-teal w3-button" type="text" name="font_size" id="font_size" placeholder="Font Size"><br>
    <label for="document_class">Document Class</label><br>
    <input class="w3-teal w3-button" type="text" name="document_class" id="document_class" placeholder="Document Class"><br>
    <br>
    <input class="w3-btn w3-teal" type="submit" value="Submit">
</form>
</div>

    </div>

    <!-- Footer -->
    <footer class="w3-content w3-padding-64 w3-text-grey w3-xlarge">
        <p class="w3-medium">Built by  <a href="https://github.com/MKProj/" target="_blank" class="w3-hover-text-green">MKProjects</a></p>
        <!-- End footer -->
    </footer>
    <!-- END PAGE CONTENT -->
</div>
</body>
</html>
"#;