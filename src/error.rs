use crate::visitor::interpreter::Value;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    // Cli
    #[error("Usage: {} [FILENAME]", env!("CARGO_PKG_NAME"))]
    UsageError,
    #[error(transparent)]
    IOError {
        #[from]
        source: std::io::Error,
    },
    #[error(transparent)]
    ReadLine {
        #[from]
        source: rustyline::error::ReadlineError,
    },

    // Lexer
    #[error("Unknown character '{0}'.")]
    UnknownCharacter(String),
    #[error("Unterminated string.")]
    UnterminatedString,
    #[error("Found decimal point not followed by decimals.")]
    DecimalPointNotFollowedByDigits,

    // Parser
    #[error("{0}")]
    ConsumeError(String),

    #[error("{0}")]
    Parser(String),

    // Interpreter
    #[error("Undefined symbol '{0}'")]
    UndefinedSymbol(String),
    #[error("Invalid operands '{values:?}', expected '{expected}'")]
    InvalidArguments {
        expected: &'static str,
        values: Vec<Value>,
    },
    #[error("Expect {expected} args, got {actual}.")]
    WrongAmountOfArgs { expected: String, actual: usize },
}
