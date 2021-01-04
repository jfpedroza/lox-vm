use crate::chunk::Chunk;
use crate::scanner::{Scanner, TokenKind};
use failure::Fallible;

pub struct Compiler;

#[derive(Debug, PartialEq, Fail)]
pub struct CompileError;

type CompileResult = Result<Chunk, CompileError>;

impl Compiler {
    pub fn compile(source: &str) -> Fallible<Chunk> {
        let mut scanner = Scanner::new(source);
        let mut line = usize::MAX;
        loop {
            let token = scanner.scan_token()?;

            if token.loc.line != line {
                print!("{:4} ", token.loc.line + 1);
                line = token.loc.line;
            } else {
                print!("   | ");
            }

            println!(
                "{:4} {:?} '{}'",
                token.loc.column + 1,
                token.kind,
                token.lexeme
            );

            if token.kind == TokenKind::EOF {
                break;
            }
        }

        Ok(Chunk::new())
    }
}
