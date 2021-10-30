// This creates the template used for MKProject Books
// There is two parts:
// 1. The mdbook part
// 2. The Latex section

use super::config::*;
use mdbook::config::Config as MdBookConfig;
use mdbook::MDBook;

pub fn create_book(path: &str) {
    let conf = Config::config(&format!("{}/config.toml", path));
    let mut cfg = MdBookConfig::default();
    cfg.book.title = Some(conf.Project.title);
    cfg.book.authors.push(conf.Project.author);

    MDBook::init(path)
        .create_gitignore(true)
        .with_config(cfg)
        .build()
        .expect("Failed to create book");
}
