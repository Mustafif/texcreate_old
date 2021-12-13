![Current Version](https://img.shields.io/crates/v/texcreate?style=flat)
![Downloads](https://img.shields.io/crates/d/texcreate?label=Downloads)
![Lang](https://img.shields.io/github/languages/top/MKProj/texcreate)
![License](https://img.shields.io/crates/l/texcreate?label=License)
# TexCreate 
### Create LaTeX Projects with Prebuilt Templates

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
$ texcreate build 
# Note config.toml is default, any other file name 
# must be specified with -f <file_path>
# To create a config.toml, you can use the init command
$ texcreate init
```

### TexCreate Build & Multi Mode
The build command has some for functionality introduced in 0.9: 
- build modes: 
  - single : The default build mode 
    - Ideal for building single projects
  - multi: Ideal for building multiple projects in one file

To initialize a multi mode project, use `texcreate init --mode multi`
You are then introduced to the following `config.toml`: 
```toml
[[Project]]
author = "Author"
title = "Title"
date = "\\today"
project_name = "Project1"
template = "Math" #Make sure to have first letter upercased

[[Document]]
paper_size = "letterpaper"
font_size = 11 #font size number
document_class = "article"
packages = []

# Multi project
[[Project]]
author = "Author"
title = "Title"
date = "\\today"
project_name = "Project2"
template = "Code" #Make sure to have first letter upercased

[[Document]]
paper_size = "letterpaper"
font_size = 11 #font size number
document_class = "article"
packages = []

```
> Note: 
> - In multi-mode, Project and Document must be in an array form `[[ ]]`  
> - The book template isn't currently supported, but is planned 
to in future releases

Then you may run `texcreate build --mode multi` which creates 2 projects: 
```shell
$ texcreate build --mode multi 
...
$ ls
config.toml Project1 Project2
```
### All config.toml options
```toml
[Project]
author = "Author"
title = "Title"
date = "\\today"
project_name = "Project Name"
template = "Math" #Make sure to have first letter upercased

[Document]
paper_size = "letterpaper"
font_size = 11 #font size number
document_class = "article"
packages = []
```
> Note: Packages Phantom Data bug has been fixed to my knowledge, however if any occurences where
> the first package isn't added, please contact me at [mustafif0929@gmail.com](mailto:mustafif0929@gmail.com)

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
 

## Errors 
Added support for error handling is specific cases while using `config.toml`, the four errors are: 
- Empty Project Fields 
  - Fields in the Project section must contain a value, an empty string will result in this error 
- Beamer Error 
    - Beamer template must have a `beamer` document class, if not, this error will occur
- Invalid Template
  - The template must be one of the prebuilt templates, if not, this error will occur 
  - To get a list use `texcreate list`
- Invalid Document Class
  - Not implemented yet, but will be added in the future
  - This occurs when a document class is invalid or not supported by the template

## Features Yet To Be Implemented
- Add support for custom templates
- Better progress indication when creating a project
- Invalid Document Class
- Beamer Error for MultiMode
- A GUI counterpart
- A web counterpart