pub mod basic;
pub mod book;
pub mod code;
pub mod novel;
pub mod theatre;
pub mod beamer;
pub mod lachaise;

pub use lachaise::*;
pub use beamer::*;
pub use basic::*;
pub use book::*;
pub use code::*;
pub use novel::*;
use tex_rs::{Class, Latex, Metadata};
pub use theatre::*;

pub fn determine_class(a: &str) -> Class {
    match a {
        "article" => Class::Article,
        "book" => Class::Book,
        "beamer" => Class::Beamer,
        "report" => Class::Report,
        _ => Class::Article,
    }
}

pub fn set(latex: &mut Latex, fs: u8, ps: &str, dc: &str, author: &str, title: &str, date: &str) {
    latex.set_class(determine_class(dc));
    latex.set_metadata(Metadata::new(title, author, date));
    latex.set_class_options(fs, ps);
}
