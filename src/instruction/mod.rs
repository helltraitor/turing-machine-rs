mod direction;
mod head;
mod tail;

pub use direction::Direction;
pub use head::Head;
pub use tail::Tail;

use std::fmt::{Display, Error, Formatter};

use crate::Symbol;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Instruction<S: Symbol> {
    pub head: Head<S>,
    pub tail: Tail<S>,
}

impl<S: Symbol> Instruction<S> {
    pub fn new(head: Head<S>, tail: Tail<S>) -> Self {
        Instruction { head, tail }
    }
}

impl<S: Symbol> Display for Instruction<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "({}) -> ({})", self.head, self.tail)
    }
}
