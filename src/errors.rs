use std::fmt;

#[derive(Debug)]
pub enum ErrorKind {
    LexError,
}

#[derive(Debug)]
pub struct CompilationError {
    line: usize,
    message: String,
    where_it_was: String,
    kind: ErrorKind,
}

#[derive(Debug)]
pub struct RuntimeError {
    line: usize,
    message: String,
    kind: ErrorKind,
}

#[derive(Debug)]
pub enum ReloxError {
    CompilationError(CompilationError),
    RuntimeError(RuntimeError),
}

impl ReloxError {
    pub fn new_compile_error(line: usize, message: String, where_it_was: String, kind: ErrorKind) -> Self {
        Self::CompilationError( CompilationError { line, message, where_it_was, kind })
    }

    pub fn new_runtime_error(line: usize, message: String, kind: ErrorKind) -> Self {
        Self::RuntimeError( RuntimeError { line, message, kind })
    }
}

impl fmt::Display for ReloxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReloxError::CompilationError(CompilationError {
                line,
                message,
                where_it_was,
                kind,
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
            }) => {
                write!(f, "[line {}] RuntimeError {:?}: {}", line, kind, message)
            }
        }
    }
}
