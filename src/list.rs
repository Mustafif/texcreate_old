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
    pub fn read(path: &str) -> Result<Vec<Self>, Error> {
        let mut file = fs::File::open(path).expect("Unable to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read file");
        let list: Vec<List> = serde_json::from_str(&contents)?;
        Ok(list)
    }
    pub fn list(path: &str) {
        let json = Self::read(path).expect("Unable to read list.json");
        println!("//////////////////////////////////////");
        println!("// List of available templates:");
        println!("//////////////////////////////////////");
        for item in json {
            println!("// {} => {}", item.template, item.about);
        }
    }
}