use std::path::PathBuf;
use async_std::fs::read_to_string;
use tex_rs::UserDefined;
use tex_rs::elements;
use tex_rs::Latex;
use texc_latex::set;
use tex_rs::Element;
use tex_rs::Level;
use tex_rs::Level::Meta;
use crate::{TexCreateError, TexCreateResult};



/// To get custom template, we will utilize the following strategy:
///
/// - custom_templates/
///     - main.texcreate
///     - structure.texcreate
/// <br>
/// Creates custom template by finding `main.texcreate` and `structure.texcreate` files in a specified path
pub async fn create_custom_template(path: PathBuf, fs: u8, ps: &str, dc: &str, author: &str, title: &str, date: &str, packages: &Vec<String>) -> TexCreateResult<Latex>{
    let mut latex = Latex::new();
    set(&mut latex, fs, ps, dc, author, title, date);
     if !path.exists(){
         return Err(TexCreateError::CannotFindCustom(path.to_str().unwrap().to_string()));
     }

    let main = path.join("main.texcreate");
    let structure = path.join("structure.texcreate");

    let main_content = read_to_string(main).await?;
    let str_content = read_to_string(structure).await?;

    latex.set_elements(elements![UserDefined::new(&main_content, Level::Body), UserDefined::new(&str_content, Level::Package), UserDefined::new(r"\input{src/structure.tex}", Meta)]);
    for i in packages{
        latex.add_package(i.to_string());
    }
    Ok(latex)
}