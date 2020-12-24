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

fn error_type(err: &Error) -> &'static str {
    if is_type::<RuntimeError>(err) {
        "RuntimeError"
    } else {
        "Error"
    }
}

pub fn exit_code(err: &Error) -> i32 {
    if is_type::<RuntimeError>(err) {
        70
    } else {
        1
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "A dummy runtime error")
    }
}
