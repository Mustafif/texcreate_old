
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter, write};

/// Types of errors that can occur when creating a texcreate project:
/// - Using beamer template and not using a beamer document class
/// - Using an invalid template
/// - Using an invalid document class
/// - Empty project name

#[derive(Debug)]
pub enum Errors {
    BeamerError,
    InvalidTemplateError(String),
    InvalidDocumentClassError(String),
    EmptyError,
}
// Implement Display for Errors
impl Display for Errors {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Errors::BeamerError => write!(f, "ERROR: Using beamer template and not using a beamer document class"),
            Errors::InvalidTemplateError(t) => {
                let s = format!("ERROR: Using an invalid template: {}", t);
                write!(f,"{}" ,&s)
            },
            Errors::InvalidDocumentClassError(d) => {
                let s = format!("ERROR: Using an invalid document class: {}", d);
                write!(f, "{}",&s)
            },
            Errors::EmptyError => write!(f, "ERROR: Empty project field in config.toml"),
        }
    }
}
// Implement Error for Errors
impl Error for Errors{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            Errors::BeamerError => None,
            Errors::InvalidTemplateError(_) => Some(self),
            Errors::InvalidDocumentClassError(_) => Some(self),
            Errors::EmptyError => None,
        }
    }
}
// Check if a string is a valid document class
pub fn invalid_class(class: &str)-> bool{
    let valid = vec![
        "article",
        "IEEEtran",
        "proc",
        "minimal",
        "report",
        "book",
        "slides",
        "memoir",
        "letter",
        "beamer"
    ];
    if !valid.contains(&class){
        true
    } else {
        false
    }
}

