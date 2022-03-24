use rocket::post;
use rocket::form::Form;
use rocket::fs::NamedFile;
use texc_config::Config;
use tempdir::TempDir;


#[post("/", data = "<input>")]
pub async fn texc_post(input: Form<Config>) -> Option<NamedFile> {
    let temp_dir = TempDir::new("texc_temp").unwrap();
    let temp_path = temp_dir.path().join(format!("{}.zip", &input.project_name));
    if input.only_files.unwrap() {
        input.zip_files(temp_dir.path().to_owned()).await.unwrap();
    } else {
        input.zip_proj(temp_dir.path().to_owned()).await.unwrap();
    }
    NamedFile::open(temp_path).await.ok()
}