use thiserror::Error;

/// Possible errors in this library.
#[derive(Error, Debug)]
pub enum MDTodoError {
    /// Issue while parsing the todo.
    #[error("Issue parsing a todo.")]
    TodoParseError,

    /// Represents a failure to read from input.
    #[error("Read error")]
    FileReadError { source: std::io::Error },

    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
