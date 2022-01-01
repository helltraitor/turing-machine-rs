use std::fmt::{Display, Error, Formatter};

use crate::instruction::Direction;
use crate::Symbol;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tail<S: Symbol> {
    pub state: u32,
    pub symbol: S,
    pub direction: Direction,
}

impl<S: Symbol> Tail<S> {
    pub fn new(state: u32, symbol: S, direction: Direction) -> Self {
        Tail {
            state,
            symbol,
            direction,
        }
    }
}

impl<S: Symbol> Display for Tail<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}, {}, {}", self.state, self.symbol, self.direction)
    }
}
