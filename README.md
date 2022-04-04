![Current Version](https://img.shields.io/crates/v/texcreate?style=flat)
![Downloads](https://img.shields.io/crates/d/texcreate?label=Downloads)
![Lang](https://img.shields.io/github/languages/top/MKProj/texcreate)
![License](https://img.shields.io/crates/l/texcreate?label=License)

# TexCreate From MKProjects

---
[Documentation](https://docs.rs/texcreate/) || [TexCreate Website](http://texcreate.mkproj.com)

TexCreate provides users to easily create projects by using a `config.toml` file
to set metadata and templates.

All news related to TexCreate are found in the [dev.to series](https://dev.to/mustafif/series/17101)
## Commands 
```shell
build      Build a project using a config.toml file
compile    Compile TexCreate Project with set TeX Compiler
doc        Opens the TexCreate documentation
edit       Quickly change config.toml parameters
help       Prints this message or the help of the given subcommand(s)
init       Initialize a config.toml file
list       Lists all the available templates
migrate    Migrate single mode config.toml from v1 to v2
update     Updates to latest version
web        Opens the TexCreate web version
zip        Zip TexCreate Project instead of building
```

### Installing 
You can install the stable `v2.0.0` by: 
```shell
$ cargo install texcreate 
```

## New Changes In Version 2
- Removed support for Multi mode 
- Added new commands 
  - compile
  - migrate 
  - zip
- Replaced `texcreate_lib` with...
  - `tex-rs`: For building LaTeX in Rust
  - `texc-utils`: For utility functions in TexCreate
  - `texc-web`: Library to run `texweb`
  - `texc-config`: Library for everything related to `config.toml`
  - `texc-latex`: Contains all of the LaTeX Templates
- New project structure 
- New `config.toml`
- Rewritten `web` and `edit` commands 
  - Web utilizes temporary directories instead of creating static files
- Templates rewritten in `tex-rs`

More changes can be seen in the [CHANGELOG](CHANGELOG.md)