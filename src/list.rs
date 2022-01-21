use serde_derive::Deserialize;
use serde_json::Error;
use std::fs;
use std::io::Read;
#[derive(Deserialize)]
pub struct List {
    pub template: String,
    pub about: String,
}

impl List {
    pub fn read() -> Result<Vec<Self>, Error> {
        let list: Vec<List> = serde_json::from_str(LIST)?;
        Ok(list)
    }
    pub fn list(path: &str) {
        let json = Self::read().expect("Unable to read list.json");
        println!("//////////////////////////////////////");
        println!("// List of available templates:");
        println!("//////////////////////////////////////");
        for item in json {
            println!("// {} => {}", item.template, item.about);
        }
    }
}

const LIST: &str = r#"[
    {
        "template": "Basic",
        "about": "A simple project with the basic setup you need"
    },
    {
        "template": "Book",
        "about": "A project that utilizes both a markdown and tex format"
    },
    {
        "template": "Math",
        "about": "A project that utilizes math related packages"
    },
    {
        "template": "Theatre",
        "about": "A project that is ideal to write scripts"
    },
    {
        "template": "Code",
        "about": "A project ideal to document about programming"
    },
    {
        "template": "Novel",
        "about": "A project to begin writing that novel you always delay"
    },
    {
        "template": "Beamer",
        "about": "A project to start making presentations in LaTeX"
    },
    {
        "template": "Lachaise",
        "about": "A project to write neat, nice documents/assignments"
    }
]
"#;