use mdbook::MDBook;
use mdbook::Config;
use mdbook::errors::Error;
use std::io::{self, Cursor};
const TEX_TEMP: &str = "https://github.com/MKProj/Basics_Template/releases/download/tex/tex.zip";
pub type TexResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn path(project_name: &str) -> String {
    format!("{}/", project_name)
}

async fn tex_create(path: &str)-> TexResult<()>{
    let response = reqwest::get(TEX_TEMP).await?;
        let p = format!("{}tex.zip",path);
        let mut file = std::fs::File::create(&p)?;
        let mut content = Cursor::new(response.bytes().await?);
        io::copy(&mut content, &mut file)?;
    println!("Attempting to unzip tex.zip");
    std::process::Command::new("unzip").args([&p, "-d", &format!("{}", &path)]).spawn().unwrap();
    Ok(())
}


pub async fn create(project_name: &str, title: &str, author: &str) -> Result<(), Error> {
    let mut cfg = Config::default();
    cfg.book.title = Some(title.to_string());
    cfg.book.authors.push(author.to_string());
    println!("Creating markdown book project...");
    MDBook::init(path(project_name))
        .create_gitignore(true)
        .with_config(cfg)
        .build()?;
    println!("Downloading tex.zip into project...");
    tex_create(&path(project_name)).await.unwrap();
    
    Ok(())
}