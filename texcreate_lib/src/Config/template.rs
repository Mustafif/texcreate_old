use serde_derive::Deserialize;

#[derive(Deserialize)]
pub enum Template {
    Basic,
    Math,
    Theatre,
    Book,
    Code,
    Novel,
    Beamer,
    Lachaise
}


impl Template{
    pub fn list() -> Vec<String> {
        vec![
            "Basic".to_string(),
            "Math".to_string(),
            "Theatre".to_string(),
            "Book".to_string(),
            "Code".to_string(),
            "Novel".to_string(),
            "Beamer".to_string(),
            "Lachaise".to_string()
        ]
    }
}