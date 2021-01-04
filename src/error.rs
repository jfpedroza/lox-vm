use crate::compiler::CompileError;
use crate::scanner::ScanningError;
use crate::vm::RuntimeError;
use ansi_term::Color::Red;
use failure::{Error, Fail};
use std::fmt::{Display, Formatter, Result as FmtResult};

pub fn print_err(err: &Error) {
    let mut fail = err.as_fail();
    eprintln!("{}: {}", Red.bold().paint(error_type(err)), fail);
    while let Some(cause) = fail.cause() {
        eprintln!("> {}", cause);
        fail = cause;
    }
}

fn is_type<T: Fail>(err: &Error) -> bool {
    err.downcast_ref::<T>().is_some()
}

fn is_compile_err(err: &Error) -> bool {
    is_type::<ScanningError>(err) || is_type::<CompileError>(err)
}

fn error_type(err: &Error) -> &'static str {
    if is_compile_err(err) {
        "CompileError"
    } else if is_type::<RuntimeError>(err) {
        "RuntimeError"
    } else {
        "Error"
    }
}

pub fn exit_code(err: &Error) -> i32 {
    if is_compile_err(err) {
        65
    } else if is_type::<RuntimeError>(err) {
        70
    } else {
        1
    }
}

impl Display for ScanningError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use ScanningError::*;
        match self {
            UnrecognizedCharacter(character, loc) => {
                write!(f, "[{}] Unrecognized character '{}'", loc, character)
            }
            UnterminatedString(loc) => write!(f, "[{}] Unterminated string", loc),
            UnterminatedBlockComment(loc) => write!(f, "[{}] Unterminated block comment", loc),
        }
    }
}

impl Display for CompileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "A dummy compile error")
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "A dummy runtime error")
    }
}
