use std::fmt::{Display, Error, Formatter};

use crate::instruction::Move;
use crate::Symbol;

/// Tail is the second part of [`crate::instruction::Instruction`]
/// and is used as a container for state, symbol, and direction.
/// Tail fields doesn't needs in control or protection so they are public.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tail<S: Symbol> {
    /// State of instucrion must be a non-negative number. This field could
    /// have another type (such as wrapped usize), but it's really hardly
    /// likely to reach limit.
    pub state: u32,
    /// Any struct or object which implements [`Symbol`] trait.
    pub symbol: S,
    /// Direction enum variant (Left, Center, Right)
    /// which used by [`crate::state::Configuration`].
    pub movement: Move,
}

impl<S: Symbol> Tail<S> {
    /// Constructs a new [`Tail`] with state, symbol and direction. Tail struct
    /// constructs immediatly that's why doesn't need to use type annotations.
    #[rustfmt::skip]
    pub fn new(state: u32, symbol: S, movement: Move) -> Self {
        Tail { state, symbol, movement }
    }
}

impl<S: Symbol> Display for Tail<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}, {}, {}", self.state, self.symbol, self.movement)
    }
}
