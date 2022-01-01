use crate::Symbol;

use std::fmt::{Display, Error, Formatter};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Head<S: Symbol> {
    pub state: u32,
    pub symbol: S,
}

impl<S: Symbol> Head<S> {
    pub fn new(state: u32, symbol: S) -> Self {
        Head { state, symbol }
    }
}

impl<S: Symbol> Display for Head<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}, {}", self.state, self.symbol)
    }
}
