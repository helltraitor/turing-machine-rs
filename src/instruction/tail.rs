use std::fmt::{Display, Error, Formatter};

use crate::instruction::{Move, State};
use crate::Symbol;

/// Tail is the second part of [`crate::instruction::Instruction`]
/// and is used as a container for the [`State`], the [`Symbol`], and the [`Move`].
///
/// [`Tail`] fields doesn't needs in control or protection so they are public.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tail<S: Symbol> {
    /// [`State`] which is a wrapper around usize.
    /// It's very hard (impossible) to reach the limit manually.
    pub state: State,
    /// Any struct or object which implements [`Symbol`] trait.
    pub symbol: S,
    /// [`Move`] enum variant ([`Move::Left`], [`Move::Right`], [`Move::None`])
    /// which is used by [`crate::state::Configuration`].
    pub movement: Move,
}

impl<S: Symbol> Tail<S> {
    /// Constructs a new [`Tail`] with the [`State`], the [`Symbol`] and the [`Move`].
    #[rustfmt::skip]
    pub fn new(state: State, symbol: S, movement: Move) -> Self {
        Tail { state, symbol, movement }
    }
}

impl<S: Symbol> Display for Tail<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}, {}, {}", self.state, self.symbol, self.movement)
    }
}
