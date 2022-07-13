use std::error::Error;
use std::fmt;
use std::io;

use crate::alloc::api::AllocError;
use crate::alloc::BlockError;

// source code position
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SourcePos {
    pub line: u32,
    pub column: u32,
}

impl SourcePos {
    fn new(line: u32, column: u32) -> SourcePos {
        SourcePos { line, column }
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    IOError(String),
    LexerError(String),
    ParseError(String),
    EvalError(String),
    BadAllocationRequest,
    IntOverflow,
    OutOfMemory,
    BoundsError,
    MutableBorrowError,
}

#[derive(Debug, PartialEq)]
pub struct RuntimeError {
    kind: ErrorKind,
    pos: Option<SourcePos>,
}

impl RuntimeError {
    pub fn new(kind: ErrorKind) -> RuntimeError {
        RuntimeError {
            kind: kind,
            pos: None,
        }
    }

    pub fn with_pos(kind: ErrorKind, pos: SourcePos) -> RuntimeError {
        RuntimeError {
            kind: kind,
            pos: Some(pos),
        }
    }

    pub fn error_kind(&self) -> &ErrorKind { &self.kind }
    pub fn error_pos(&self) -> Option<SourcePos> { self.pos }

    pub fn print_with_source(&self, source: &str) {
        if let Some(ref pos) = self.pos {
            let mut iter = source.lines().enumerate();

            while let Some((count, line)) = iter.next() {
                if count + 1 == pos.line as usize {
                    println!("error: {}", self);
                    println!("{:5}|{}", pos.line, line);
                    println!("{:5}|{:width$}^",
                             " ", " ",
                             width = pos.column as usize
                    );
                    println!("{:5}|", " ");
                    return;
                }
            }
        } else {
            println!("error: {}", self);
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::IOError(ref reason) => write!(f,
                "IO Error: {}", reason
            ),
            ErrorKind::LexerError(ref reason) => write!(f,
                "Parse Error: {}", reason
            ),
            ErrorKind::ParseError(ref reason) => write!(f,
                "Parse Error: {}", reason
            ),
            ErrorKind::EvalError(ref reason) => write!(f,
                "Eval Error: {}", reason
            ),
            ErrorKind::BadAllocationRequest => write!(f,
                "Invalid memory size allocation requested"
            ),
            ErrorKind::IntOverflow => write!(f, "Integer overflow"),
            ErrorKind::OutOfMemory => write!(f, "Out of memory"),
            ErrorKind::BoundsError => write!(f, "Indexing bounds error"),
            ErrorKind::MutableBorrowError => write!(f,
                "Attempted to modify container that is already mutably borrowed"
            ),
        }
    }
}

impl From<io::Error> for RuntimeError {
    fn from(other: io::Error) -> RuntimeError {
        RuntimeError::new(ErrorKind::IOError(format!("{}", other)))
    }
}

impl From<BlockError> for RuntimeError {
    fn from(other: BlockError) -> RuntimeError {
        match other {
            BlockError::OutOfMemory => RuntimeError::new(
                ErrorKind::OutOfMemory
            ),
            BlockError::BadRequest => RuntimeError::new(
                ErrorKind::BadAllocationRequest
            ),
        }
    }
}

impl From<AllocError> for RuntimeError {
    fn from(other: AllocError) -> RuntimeError {
        match other {
            AllocError::OutOfMemory => RuntimeError::new(
                ErrorKind::OutOfMemory
            ),
            AllocError::BadRequest => RuntimeError::new(
                ErrorKind::BadAllocationRequest
            ),
        }
    }
}

impl Error for RuntimeError {
    fn cause(&self) -> Option<&dyn Error> { None }
}

impl From<RuntimeError> for fmt::Error {
    fn from(_other: RuntimeError) -> fmt::Error { fmt::Error }
}

pub fn spos(line: u32, column: u32) -> SourcePos {
    SourcePos::new(line, column)
}

pub fn err_lexer(pos: SourcePos, reason: &str) -> RuntimeError {
    RuntimeError::with_pos(ErrorKind::LexerError(String::from(reason)), pos)
}

pub fn err_parser(reason: &str) -> RuntimeError {
    RuntimeError::new(ErrorKind::ParseError(String::from(reason)))
}

pub fn err_parser_wpos(reason: &str, pos: SourcePos) -> RuntimeError {
    RuntimeError::new(ErrorKind::ParseError(String::from(reason)), pos)
}

pub fn err_eval(reason: &str) -> RuntimeError {
    RuntimeError::new(ErrorKind::EvalError(String::from(reason)))
}
