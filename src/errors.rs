use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    LexError,
    ParserError,
    EvalError,
    Fatal,
}

#[derive(Debug)]
pub struct CompilationError {
    pub line: usize,
    pub message: String,
    where_it_was: Option<String>,
    pub kind: ErrorKind,
}

#[derive(Debug)]
pub struct RuntimeError {
    pub line: usize,
    pub message: String,
    pub kind: ErrorKind,
}

#[derive(Debug)]
pub struct FatalError {
    pub message: String,
}

#[derive(Debug)]
pub enum ReloxError {
    CompilationError(CompilationError),
    RuntimeError(RuntimeError),
    FatalError(FatalError),
}

impl ReloxError {
    pub fn new_compile_error(
        line: usize,
        message: String,
        where_it_was: Option<String>,
        kind: ErrorKind,
    ) -> Self {
        Self::CompilationError(CompilationError {
            line,
            message,
            where_it_was,
            kind,
        })
    }

    pub fn new_fatal_error(message: String) -> Self{
       Self::FatalError(FatalError { message })
    }

    pub fn new_runtime_error(line: usize, message: String, kind: ErrorKind) -> Self {
        Self::RuntimeError(RuntimeError {
            line,
            message,
            kind,
        })
    }

    pub fn kind(&self) -> ErrorKind {
        match self {
            ReloxError::CompilationError(error) => error.kind,
            ReloxError::RuntimeError(error) => error.kind,
            ReloxError::FatalError(_) => ErrorKind::Fatal,
        }
    }
}

impl fmt::Display for ReloxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReloxError::CompilationError(CompilationError {
                line,
                message,
                kind,
                where_it_was: None,
            }) => {
                write!(f, "[line {}] Error {:?}: {}", line, kind, message)
            }
            ReloxError::CompilationError(CompilationError {
                line,
                message,
                kind,
                where_it_was: Some(where_it_was),
            }) => {
                write!(
                    f,
                    "[line {}] Error {} {:?}: {}",
                    line, where_it_was, kind, message
                )
            }
            ReloxError::RuntimeError(RuntimeError {
                line,
                message,
                kind,
            }) => write!(f, "[line {}] RuntimeError {:?}: {}", line, kind, message),
            ReloxError::FatalError(FatalError { message }) => write!(f, "FatalError: {}", message),
        }
    }
}
