![Current Version](https://img.shields.io/crates/v/texcreate?style=flat)
![Downloads](https://img.shields.io/crates/d/texcreate?label=Downloads)
![Lang](https://img.shields.io/github/languages/top/MKProj/texcreate)
![License](https://img.shields.io/crates/l/texcreate?label=License)
# TexCreate 
### Create LaTeX Projects with Prebuilt Templates

Currently only has two templates, but more are on the way: 
- Basic Template : Good for a single file project
- Math Template : Good for a math project 

## Install
You can install via `Cargo`: 
```sh
$ cargo install texcreate
```

## Usage 
There are two ways to create projects, the `create` & `import` commands: 
- create: Use to quickly create a project supplying name, template and _path (optional)_
- import: Use a `config.toml` file to specify configurations to automatically generate when creating a project. 

```sh
$ texcreate create -n <name> -t <template>
# Optionally add -d <path>
$ texcreate import -f config.toml
# Note any <name>.toml works, but i prefer 
# using config.toml
# To create a config.toml, you can use the init command
$ texcreate init
```

### All config.toml options
```toml
[Project]
author = "Author"
title = "Title"
date = "\today"
project_name = "Project Name"
template = "Math" #Make sure to have first letter upercased

[Document]
paper_size = "letterpaper"
font_size = 11 #font size number
document_class = "article"
packages = ["PhantomData", ""]
```
> Note: Phantom Data is to show that the first element is not used, other packages after it will be added.  
More customizations will come in the future as I figure out more effective ways 
to create the projects, and get more templates on the way.

### Book Template
The Book template is the template that is used to create MKProject Books, which uses Rust's MDBook engine and a latex book template. This template is only avaiable through using `config.toml`, however it is a bit more special than the other templates where it will not use Document options (still keep it on the config.toml though).

The Book template creates the following structure: 
- `book`: The html output
- `src`: The src directory of md files
- `tex`: The tex directory of tex files
- `.gitignore`" Specifies which files to ignore when using git
- `book.toml`: The configuration file for the mdbook
- `tex.zip`: Dispose if you'd like 

Make sure to have the following installed: 
- mdbook : `cargo install mdbook`
- `unzip` : Should be preinstalled on most systems, verify with `unzip --version`
 