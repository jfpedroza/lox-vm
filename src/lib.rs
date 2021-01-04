#![feature(in_band_lifetimes)]

extern crate failure;
#[macro_use]
extern crate failure_derive;

mod chunk;
mod compiler;
pub mod error;
mod location;
mod scanner;
mod utils;
mod value;
mod vm;

use ansi_term::Color::{Blue, Cyan};
use compiler::Compiler;
use error::print_err;
use failure::{Fallible, ResultExt};
use rustyline::{config::Configurer, error::ReadlineError, Editor};
use std::ffi::OsStr;
use std::io::{stdin, Read};
use std::path::Path;
use value::Value;
use vm::VM;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Lox {
    vm: VM,
}

impl Lox {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Lox { vm: VM::new() }
    }

    pub fn run(&mut self, input: &str) -> Fallible<()> {
        let chunk = Compiler::compile(input)?;
        self.vm.interpret(chunk)?;

        Ok(())
    }

    pub fn run_file(&mut self, path: &OsStr) -> Fallible<()> {
        let content = if path == "-" {
            let mut content = String::new();
            stdin()
                .lock()
                .read_to_string(&mut content)
                .context("Could not read from stdin")?;
            content
        } else {
            let path = Path::new(path);
            let context = format!("Could not read '{}'", path.display());
            std::fs::read_to_string(path).context(context)?
        };

        self.run(&content)
    }

    pub fn run_prompt(&mut self) -> Fallible<()> {
        let mut rl = Editor::<()>::new();
        rl.set_auto_add_history(true);

        println!("Lox {}", VERSION);
        println!("Press Ctrl+D to exit\n");

        let prompt = format!("{}> ", Blue.bold().paint("lox"));

        loop {
            match rl.readline(&prompt) {
                Ok(line) if line.is_empty() => (),
                Ok(line) => match self.run_prompt_line(&line) {
                    Ok(()) => (),
                    Err(err) => print_err(&err),
                },
                Err(ReadlineError::Interrupted) => (),
                Err(ReadlineError::Eof) => break,
                Err(err) => return Err(err.into()),
            }
        }

        Ok(())
    }

    fn run_prompt_line(&mut self, input: &str) -> Fallible<()> {
        let chunk = Compiler::compile(input)?;
        self.vm.interpret(chunk)?;

        print!("=> ");
        print_value(&3.5);
        println!();
        Ok(())
    }
}

fn print_value(val: &Value) {
    let output = Cyan.paint(val.to_string());

    print!("{}", output);
}
