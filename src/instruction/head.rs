use crate::instruction::State;
use crate::Symbol;

use std::fmt::{Display, Error, Formatter};

/// [`Head`] is the first part of [`crate::instruction::Instruction`]
/// and is used as a container for the [`State`] and the [`Symbol`].
///
/// [`Head`] fields doesn't needs in control or protection so they are public.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Head<S: Symbol> {
    /// [`State`] which is a wrapper around usize.
    /// It's very hard (impossible) to reach the limit manually.
    pub state: State,
    /// Any struct or object which implements [`Symbol`] trait.
    pub symbol: S,
}

impl<S: Symbol> Head<S> {
    /// Constructs a new [`Head`] with the [`State`] and the [`Symbol`].
    pub fn new(state: State, symbol: S) -> Self {
        Head { state, symbol }
    }
}

impl<S: Symbol> Display for Head<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}, {}", self.state, self.symbol)
    }
}
