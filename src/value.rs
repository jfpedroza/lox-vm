use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone)]
pub struct Value(pub f64);

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}
