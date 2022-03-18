![Current Version](https://img.shields.io/crates/v/texcreate?style=flat)
![Downloads](https://img.shields.io/crates/d/texcreate?label=Downloads)
![Lang](https://img.shields.io/github/languages/top/MKProj/texcreate)
![License](https://img.shields.io/crates/l/texcreate?label=License)

# TexCreate From MKProjects  

---
TexCreate provides users to easily create projects by using a `config.toml` file 
to set metadata and templates. 

## Version 2 Beta 
Currently TexCreate is on Version 2 beta, and is recommended to stay in `1.2.0`
until a stable release is out. In this current beta, not everything is stable, and 
there is still some _"broken"_ code. Here are some of the following issues: 

- Edit, Doc and Web are still being developed 
  - Edit expected for beta 2 
  - Doc will come along with texcreate.mkproj.com
  - Web will come along with zip command 
- Basic, Math and Code templates are only available 
  - Other templates are still being developed with `tex-rs` backend 


## Changes 
New `config.toml` format: 
```toml
author = 'Author'
title = 'Title'
date = 'Date'
project_name = 'Project'
template = 'Basic'
paper_size = 'letterpaper'
document_class = 'article'
font_size = 12
packages = []
only-files = false
```

New project structure: 
```shell
out/
  Project.pdf
src/
  Project.tex
  structure.tex
README.md 
texcreate.toml    
```

Along with the new `compile` command, you can define your TeX Compiler in `texcreate.toml`, 
and it will compile into the `out` directory. 

### New Commands 
- compile: 
  - Allows user to compile their project
- migrate: 
  - Allows user to turn their `config.toml` from `v1` to `v2`
  

> Note: More information and proper documentation will be provided in TeXCreate website
> that is still being developed. 
  
> Disclaimer: This release is still considered very beta and it's main purpose 
> is to allow people to start migrating their projects, however for production uses 
> continue to use `v1.2.0`