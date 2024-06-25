use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum ParseError {
    #[error("failed to parse prefix at position {0} (found {1})")]
    PrefixError(usize, char),
    #[error("failed to parse command name at position {0} (found {1})")]
    NameError(usize, char),
    #[error("failed to escape character at position {0} (found {1})")]
    EscapeError(usize, char),
}