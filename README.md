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
date = "YYYY-MM-DD"
project_name = "Project Name"
template = "Math" #Make sure to have first letter upercased

[Document]
paper_size = "letterpaper"
font_size = 11 #font size number
document_class = "article"
```
More customizations will come in the future as I figure out more effective ways 
to create the projects, and get more templates on the way.    