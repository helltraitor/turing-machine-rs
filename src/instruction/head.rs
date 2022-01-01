use crate::Symbol;

use std::fmt::{Display, Error, Formatter};

/// Head is the first part of [`crate::instruction::Instruction`]
/// and is used as a container for state and symbol. Head fields
/// doesn't needs in control or protection so they are public.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Head<S: Symbol> {
    /// State of instucrion must be a non-negative number. This field could
    /// have another type (such as wrapped usize), but it's really hardly
    /// likely to reach limit.
    pub state: u32,
    /// Any struct or object which implements [`crate::Symbol`] trait.
    pub symbol: S,
}

impl<S: Symbol> Head<S> {
    /// Constructs a new [`Head`] with state and symbol. Head struct constructs
    /// immediatly that's why doesn't need to use type annotations.
    pub fn new(state: u32, symbol: S) -> Self {
        Head { state, symbol }
    }
}

impl<S: Symbol> Display for Head<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}, {}", self.state, self.symbol)
    }
}
