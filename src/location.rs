use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct Loc {
    pub line: usize,
    pub column: usize,
}

impl Loc {
    pub fn new(line: usize, column: usize) -> Self {
        Loc { line, column }
    }

    pub fn advance(&mut self) {
        self.column += 1;
    }

    pub fn new_line(&mut self) {
        self.line += 1;
        self.column = 0;
    }
}

impl Display for Loc {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}:{}", self.line + 1, self.column + 1)
    }
}

impl From<(usize, usize)> for Loc {
    fn from((line, column): (usize, usize)) -> Self {
        Loc { line, column }
    }
}
