/// Basic template
pub mod basic;
/// Book template
pub mod book;
/// Code template
pub mod code;
/// Novel template
pub mod novel;
/// Theatre template
pub mod theatre;
/// Beamer template
pub mod beamer;
/// Lachaise template
pub mod lachaise;

pub use lachaise::*;
pub use beamer::*;
pub use basic::*;
pub use book::*;
pub use code::*;
pub use novel::*;
use tex_rs::{Class, Latex, Metadata};
pub use theatre::*;


/// Used to set the `tex_rs::Class`
///
/// ```rust
/// use texc_latex::determine_class;
/// use tex_rs::Class;
/// fn main(){
///     let s = determine_class("article");
///     assert_eq!(s, Class::Article);
/// }
/// ```
pub fn determine_class(a: &str) -> Class {
    match a {
        "article" => Class::Article,
        "book" => Class::Book,
        "beamer" => Class::Beamer,
        "report" => Class::Report,
        _ => Class::Article,
    }
}
/// Sets all of the metadata for the templates
/// ```
/// use tex_rs::Latex;
/// use texc_latex::set;
/// fn main(){
///     let mut latex = Latex::new();
///     set(&mut latex, 11, "papersize", "documentclass", "author", "title", "date");
/// }
/// ```
pub fn set(latex: &mut Latex, fs: u8, ps: &str, dc: &str, author: &str, title: &str, date: &str) {
    latex.set_class(determine_class(dc));
    latex.set_metadata(Metadata::new(title, author, date));
    latex.set_class_options(fs, ps);
}
