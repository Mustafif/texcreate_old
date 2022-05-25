use std::fs;
use std::io::Write;
use std::path::PathBuf;
use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket::post;
use tempdir::TempDir;
use texc_config::{Config, README, TexcToml};
use zip::CompressionMethod::Stored;
use zip::write::{FileOptions, ZipWriter};
use async_std::fs::{create_dir, remove_dir, remove_dir_all, remove_file};
use toml::to_string_pretty;


/// Accepts a form of `Config` and returns the `zipped` file
/// Uses the `zip_files()` or `zip_proj()` functions depending on `only-files`
#[post("/", data = "<input>")]
pub async fn texc_post(input: Form<Config>) -> Option<NamedFile> {
    let mut input = input;
    input.custom_template = None;
    if input.template == None{
        input.template = Some("Basic".to_string());
    }
    create_dir("texc_temp").await.ok()?;
    let path = PathBuf::from("texc_temp").join(format!("{}.zip", &input.project_name));
    if input.only_files.unwrap() {
        let mut zip = ZipWriter::new(
            fs::File::create(path.join(format!("{}.zip", &input.project_name)))
                .unwrap(),
        );
        let options = FileOptions::default().compression_method(Stored);

        let main = path.join(format!("{}.tex", &input.project_name));
        let structure = path.join("structure.tex");
        let _ = fs::File::create(&main).ok()?;
        let _ = fs::File::create(&structure).ok()?;
        let temp_str = input.template().ok()?.split_string();

        zip.start_file(main.to_str().unwrap(), options).unwrap();
        zip.write_all(&temp_str.0.as_bytes()).unwrap();

        zip.start_file(structure.to_str().unwrap(), options)
            .unwrap();
        zip.write_all(&temp_str.1.as_bytes()).unwrap();

        zip.finish().unwrap();

        // Cleaning step
        remove_file(&main).await.ok()?;
        remove_file(&structure).await.ok()?;
    } else {
        let mut zip = ZipWriter::new(
            fs::File::create(&path).ok()?
        );
        let options = FileOptions::default().compression_method(Stored);

        // README.md
        let readme_path = path.join("README.md");
        let _ = fs::File::create(&readme_path).ok()?;
        zip.start_file(readme_path.to_str().unwrap(), options)
            .unwrap();
        zip.write_all(README.as_bytes()).unwrap();
        // texcreate.toml
        let toml_path = path.join("texcreate.toml");
        let _ = fs::File::create(&toml_path).unwrap();
        zip.start_file(toml_path.to_str().unwrap(), options)
            .unwrap();
        let mut tex_toml = TexcToml::default();
        tex_toml.project_name = input.clone().project_name;
        zip.write_all(to_string_pretty(&tex_toml).unwrap().as_bytes())
            .unwrap();
        // out/
        let out_path = path.join("out");
        create_dir(&out_path).await.ok()?;
        zip.add_directory(out_path.to_str().unwrap(), options).unwrap();
        // src/
        let src = path.join("src");
        create_dir(&src).await.ok()?;

        // src/*.tex
        let temp_str = input.template().ok()?.split_string();

        let main = src.join(format!("{}.tex", &input.project_name));
        let _ = fs::File::create(&main).unwrap();

        let structure = src.join("structure.tex");
        let _ = fs::File::create(&structure).unwrap();

        zip.start_file(main.to_str().unwrap(), options).unwrap();
        zip.write_all(&temp_str.0.as_bytes()).unwrap();

        zip.start_file(structure.to_str().unwrap(), options).unwrap();
        zip.write_all(&temp_str.1.as_bytes()).unwrap();

        zip.finish().unwrap();

        // Cleaning step
        remove_file(&readme_path).await.ok()?;
        remove_file(&toml_path).await.ok()?;
        remove_dir(&out_path).await.ok()?;
        remove_dir_all(&src).await.ok()?;
    }
    NamedFile::open(path).await.ok()
}
