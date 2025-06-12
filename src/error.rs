use thiserror::Error;
use tree_sitter::LanguageError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("unable to load language")]
    UnableToLoadLanguage(#[from] LanguageError),
    #[error("unable to parse: {0}")]
    UnableToParse(String),
}
