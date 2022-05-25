![Current Version](https://img.shields.io/crates/v/texcreate?style=flat)
![Downloads](https://img.shields.io/crates/d/texcreate?label=Downloads)
![Lang](https://img.shields.io/github/languages/top/MKProj/texcreate)
![License](https://img.shields.io/crates/l/texcreate?label=License)

# TexCreate From MKProjects

---

[TexCreate Website](http://texcreate.mkproj.com)

---

### Documentation 
- [tex-rs](https://docs.rs/tex-rs/) : `Build LaTeX with Rust`
- [texc-config](https://docs.rs/texc-config/): `TexCreate Config.toml library`
- [texc-latex](https://docs.rs/texc-latex/): `TexCreate LaTeX Templates library`
- [texc-utils](https://docs.rs/texc-utils/): `TexCreate Utility library`
- [texc-web](https://docs.rs/texc-web): `TexCreate Local Web Library`

---

TexCreate provides users to easily create projects by using a `config.toml` file
to set metadata and templates.

All news related to TexCreate are found in the [dev.to series](https://dev.to/mustafif/series/17101)

---

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

--- 

### Installing 
You can install the stable `v2` by: 
```shell
$ cargo install texcreate 
```

---

## Changes for v2.3

- Linux's systems will utilize `tectonic` for the `compile` command
  - Other systems will use the current method due to complication with dependencies

---

More changes can be seen in the [CHANGELOG](CHANGELOG.md)