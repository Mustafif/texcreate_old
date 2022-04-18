use crate::TexCreateResult;
use async_std::fs::{create_dir, remove_dir_all, File};
use async_std::io::WriteExt;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;
use toml::to_string_pretty;
/// README.md explaining how to start the project
pub const README: &str = r#"# Let's Begin <project>
Welcome to <project>, let's begin with an overview of where all of the files are located.

- `src/`
    - Contains all of the source code
- `out/`
    - Contains the compiled pdf
- `texcreate.toml`
    - Contains the default to compile and view code

## Compiling
To compile a project, you can use:
```
$ texcreate compile
# or
$ <texcompiler> --output-directory out src/<project>.tex
```
"#;
/// Creates the README using the project name
pub fn readme_make(project: &str) -> String {
    let mut r = README.to_string();
    while r.contains("<project>") {
        r = r.replace("<project>", project);
    }
    r
}
/// TexcToml represents `texcreate.toml` which uses
/// - Project Name : Used to compile the project
/// - Compiler: Specific tex compiler to use
#[derive(Deserialize, Serialize)]
pub struct TexcToml {
    pub project_name: String,
    pub compiler: String,
}

impl Default for TexcToml {
    fn default() -> Self {
        Self {
            project_name: "".to_string(),
            compiler: "pdflatex".to_string(),
        }
    }
}

impl TexcToml {
    /// Compiles the project using specified project name and compiler
    pub async fn compile(&self) -> TexCreateResult<()> {
        remove_dir_all("out").await?;
        create_dir("out").await?;
        let s = format!("src/{}.tex", &self.project_name);
        let cmd = Command::new(&self.compiler)
            .arg("--output-directory")
            .arg("out")
            .arg(&s)
            .output()
            .unwrap();
        if cmd.status.success() {
            println!(
                "Successfully compiled src/{}.tex to out/{}.pdf",
                &self.project_name, &self.project_name
            );
        } else {
            eprintln!("Error in compiling: {}", cmd.status.to_string());
        }
        Ok(())
    }
}
/// Writes `texcreate.toml` using a path and project name from `Config.project_name`
pub async fn write_toml(path: PathBuf, project_name: &str) -> TexCreateResult<()> {
    let mut texc = TexcToml::default();
    texc.project_name = project_name.to_string();
    let s = to_string_pretty(&texc).unwrap();
    let mut file = File::create(path.join("texcreate.toml")).await?;
    file.write_all(s.as_bytes()).await?;
    Ok(())
}
