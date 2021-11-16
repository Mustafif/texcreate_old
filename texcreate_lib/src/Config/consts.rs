pub const CONFIG_TOML: &str = r#"[Project]
author = "Author"
title = "Title"
date = "\\today"
project_name = "Project Name"
template = "Math" #Make sure to have first letter upercased

[Document]
paper_size = "letterpaper"
font_size = 11 #font size number
document_class = "article"
packages = ["PhantomData", ""]
"#;

pub const AUTHOR: &str = "\\author{}";
pub const TITLE: &str = "\\title{}";
pub const DATE: &str = "\\date{}";