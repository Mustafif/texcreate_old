use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket::post;
use tempdir::TempDir;
use texc_config::Config;


/// Accepts a form of `Config` and returns the `zipped` file
/// Uses the `zip_files()` or `zip_proj()` functions depending on `only-files`
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
