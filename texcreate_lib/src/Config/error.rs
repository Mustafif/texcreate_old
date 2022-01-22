
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter, write};

/// Types of errors that can occur when creating a texcreate project:
/// - Using beamer template and not using a beamer document class
/// - Using an invalid template
/// - Using an invalid document class
/// - Empty project field
/// - IO error (Uses std::io::Error)

#[derive(Debug)]
pub enum Errors {
    BeamerError,
    InvalidTemplateError(String),
    InvalidDocumentClassError(String),
    EmptyError(String),
    IoError(std::io::Error),
}
// Implement From for Errors
impl From<std::io::Error> for Errors {
    fn from(i: std::io::Error) -> Self {
        Errors::IoError(i)
    }
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
            Errors::EmptyError(s) => write!(f, "ERROR: Empty {} Field in config.toml", s),
            Errors::IoError(e) => write!(f, "ERROR: {}", e.to_string()),
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
            Errors::EmptyError(_) => Some(self),
            Errors::IoError(_) => Some(self),
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

