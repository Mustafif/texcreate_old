# Adding New Templates

In TexCreate, templates are the foundation of the program, and most updates are just 
adding new templates. So how do we add one? Well there's a few steps to it, but by the end 
we should have a working template. 

> Note: Some templates may need special attention, such as `book` and `beamer`

A general new template contains the following files:
- `main.tex`
- `structure.tex`

The first is to contain the source code of the document, while structure.tex contains 
all the preamble for the document. So to add a new template...
1. Create a file in `texcreate_lib/src/Templates/<temp_name>.rs` and create 2 public
constants that are the <temp_name>_MAIN and <temp_name>_STRUCTURE.
2. Add the constants in the `mod.rs` file in the same directory as the templates:
```rust
pub mod basic;
pub mod book;
pub mod math;
pub mod code;
pub mod theatre;
pub mod novel;
pub mod beamer;
pub mod lachaise;
pub mod <temp_name>;

use basic::*;
use book::*;
use math::*;
use code::*;
use theatre::*;
use novel::*;
use beamer::*;
use lachaise::*;
use <temp_name>::*;
```
3. Add a match pattern to `load()` in `texcreate_lib/src/routes.rs` : 
```rust
pub fn load(template: &str) -> (String, String) {
    match template {
        "Basic" => const_conv(basic::BASIC_MAIN, basic::BASIC_STRUCTURE),
        "Code" => const_conv(code::CODE_MAIN, code::CODE_STRUCTURE),
        "Math" => const_conv(math::MATH_MAIN, math::MATH_STRUCTURE),
        "Theatre" => const_conv(theatre::THEATRE_MAIN, theatre::THEATRE_STRUCTURE),
        "Novel" => const_conv(novel::NOVEL_MAIN, novel::NOVEL_STRUCTURE),
        "Beamer" => const_conv(beamer::BEAMER_MAIN, beamer::BEAMER_STRUCTURE),
        "Lachaise" => const_conv(lachaise::LACHAISE_MAIN, lachaise::LACHAISE_STRUCTURE),
        "<temp_name>" => const_conv(<temp_name>::<temp_name>_MAIN, <temp_name>::<temp_name>_STRUCTURE),
        _ => panic!("Unknown template: {}", template),
    }
}
```
4. Add the template to `texcreate_lib/src/Config/template.rs` `Template` enum: 
```rust
#[derive(Deserialize)]
pub enum Template {
    Basic,
    Math,
    Theatre,
    Book,
    Code,
    Novel,
    Beamer,
    Lachaise, 
    <temp_name>,
}
```
5. Inside `texcreate_lib/src/Config/config.rs`, add the template name to `from_template()`: 
```rust
impl Config {
    pub fn init() {
        let mut file = fs::File::create("config.toml").expect("Unable to create config file");
        file.write_all(CONFIG_TOML.as_bytes())
            .expect("Unable to write to config file");
    }
    pub fn from_template(&self) -> Template {
        match self.Project.template.as_str() {
            "Basic" => Template::Basic,
            "Math" => Template::Math,
            "Theatre" => Template::Theatre,
            "Book" => Template::Book,
            "Code" => Template::Code,
            "Novel" => Template::Novel,
            "Beamer" => Template::Beamer,
            "Lachaise" => Template::Lachaise,
            "<temp_name>" => Template::<temp_name>,
            _ => Template::Basic,
        }
    }
    ...
```
6. Add the template to `list.json`:
```json
...
    {
        "template": "Beamer",
        "about": "A project to start making presentations in LaTeX"
    },
    {
        "template": "Lachaise",
        "about": "A project to write neat, nice documents/assignments"
    }, 
    {
        "template": "<temp_name>",
        "about": "About the template"
    },
]
```
With all work done in `texcreate_lib`, the last step is updating the `import` command in the `src/main.rs`: 
```rust
...
CLI::Import { file } => {
            let conf = Config::config(&file);
            match conf.from_template() {
                Template::Basic => {
                    import_temp!(conf, "Basic");
                }
                Template::Math => {
                    import_temp!(conf, "Math");
                }
                Template::Theatre => {
                    import_temp!(conf, "Theatre");
                }
                Template::Code => {
                    import_temp!(conf, "Code");
                }
                Template::Novel =>{
                    import_temp!(conf, "Novel");
                }
                Template::Beamer => {
                    import_temp!(conf, "Beamer");
                }
                Template::Lachaise => {
                    import_temp!(conf, "Lachaise");
                }
                Template::<temp_name> => {
                    import_temp!(conf, "<temp_name>");
                }
                Template::Book => mkcreate(
                    &conf.Project.project_name,
                    &conf.Project.title,
                    &conf.Project.author,
                )
                .await
                .expect("Couldn't make Book Project"),
            }
        }
    }
}
```
Now it's wise to run `cargo build` and test out the template in a `config.toml` file using 
`./target/debug/texcreate import` with the template as the one you want to test. If everything works, 
you can now create a push request to the __texcreate__ repository.