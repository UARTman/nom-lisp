use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum LispError {
    #[error("Type error: {0}")]
    TypeError(String),
    #[error("Syntax error: {0}")]
    SyntaxError(String),
    #[error("Stack underflowed.")]
    StackEmpty,
    #[error("Variable {0} is not in scope.")]
    VariableNotFound(String),
    #[error("Runtime error: {0}")]
    Runtime(String),
}

impl From<&LispError> for LispError {
    fn from(value: &LispError) -> Self {
        value.clone()
    }
}

pub type Result<T> = std::result::Result<T, LispError>;
