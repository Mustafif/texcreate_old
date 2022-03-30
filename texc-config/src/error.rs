use crate::Config;
use failure;
use failure_derive::Fail;
use std::io::Error;

/// The TexCreate Errors that can occur
/// - Beamer Error: When Beamer template is chosen, but doc_class isn't beamer
/// - Invalid Template: Template doesn't exist
/// - Invalid Document Class: Document Class doesn't exist
/// - Empty Fields: A field in `config.toml` is left empty
/// - IO Error: Error caused by `std::io::Error`
/// - Invalid: Any other error
#[derive(Fail, Debug)]
pub enum TexCreateError {
    #[fail(
        display = "{} Document Class is illegal for Beamer, please set class to beamer",
        _0
    )]
    BeamerError(String),
    #[fail(
        display = "{} is an invalid Template, use texcreate list for list of available templates",
        _0
    )]
    InvalidTemplate(String),
    #[fail(display = "{} is an invalid Document Class!", _0)]
    InvalidDocClass(String),
    #[fail(display = "The {} field has an empty value!", _0)]
    EmptyFields(String),
    #[fail(display = "{}", _0)]
    IOError(#[cause] Error),
    #[fail(display = "Invalid {}", _0)]
    Invalid(String),
}

fn valid_templates() -> Vec<&'static str> {
    vec![
        "Basic", "Book", "Math", "Theatre", "Code", "Novel", "Beamer", "Lachaise",
    ]
}
fn valid_classes() -> Vec<&'static str> {
    vec![
        "article", "IEEEtran", "proc", "minimal", "report", "book", "slides", "memoir", "letter",
    ]
}

impl From<Error> for TexCreateError {
    fn from(e: Error) -> Self {
        Self::IOError(e)
    }
}
/// Result type for TexCreate
pub type TexCreateResult<T> = std::result::Result<T, TexCreateError>;

/// Checks if config has a beamer error, if so returns `TexCreateError::BeamerError`
pub fn check_beamer_error(config: &Config) -> TexCreateResult<()> {
    /*
    Beamer error occurs when the Document class is
    not set as beamer when the Beamer Template is chosen
     */
    if config.template == "Beamer" && config.document_class != "beamer" {
        Err(TexCreateError::BeamerError(config.document_class.clone()))
    } else {
        Ok(())
    }
}
/// Checks if config has an invalid template, if so returns `TexCreateError::InvalidTemplate`
pub fn check_invalid_template(config: &Config) -> TexCreateResult<()> {
    /*
    Invalid template error occurs when a user enters a template that
    does not exist, to do this we look at a vec and see if it matches
     */
    if !valid_templates().contains(&config.template.as_str()) {
        Err(TexCreateError::InvalidTemplate(config.template.clone()))
    } else {
        Ok(())
    }
}
/// Checks if the config has an invalid document class, if so returns `TexCreateError::InvalidDocClass`
pub fn check_invalid_class(config: &Config) -> TexCreateResult<()> {
    /*
    Invalid class error occurs when a user enters a document
    class that does not exist, we look at a vec and see if it matches
     */
    if !valid_classes().contains(&config.document_class.as_str()) {
        Err(TexCreateError::InvalidDocClass(
            config.document_class.clone(),
        ))
    } else {
        Ok(())
    }
}
/// Checks if the config has any empty fields, if so returns `TexCreateError::EmptyFields`
pub fn check_empty_field(config: &Config) -> TexCreateResult<()> {
    /*
    Checks each field if empty, ignores fields that are optional
     */
    if config.author.is_empty() {
        Err(TexCreateError::EmptyFields("Author".to_string()))
    } else if config.title.is_empty() {
        Err(TexCreateError::EmptyFields("Title".to_string()))
    } else if config.date.is_empty() {
        Err(TexCreateError::EmptyFields("Date".to_string()))
    } else if config.project_name.is_empty() {
        Err(TexCreateError::EmptyFields("Project Name".to_string()))
    } else if config.template.is_empty() {
        Err(TexCreateError::EmptyFields("Template".to_string()))
    } else if config.document_class.is_empty() {
        Err(TexCreateError::EmptyFields("Document Class".to_string()))
    } else if config.paper_size.is_empty() {
        Err(TexCreateError::EmptyFields("Paper Size".to_string()))
    } else if config.font_size.to_string().is_empty() {
        Err(TexCreateError::EmptyFields("Font Size".to_string()))
    } else {
        Ok(())
    }
}
/// Contains all checks helper functions into one function
pub fn check_errors(config: &Config) -> TexCreateResult<()> {
    /*
    Checks all errors in one function
     */
    check_beamer_error(config)?;
    check_invalid_template(config)?;
    check_invalid_class(config)?;
    check_empty_field(config)?;
    Ok(())
}
