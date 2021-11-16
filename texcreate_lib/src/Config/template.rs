use serde_derive::Deserialize;

#[derive(Deserialize)]
pub enum Template {
    Basic,
    Math,
    Theatre,
    Book,
    Code, 
}