//! Provides [`Instruction`] and it's components: [`Head`], [`Move`],
//! [`State`], [`Tail`].
//!
//! This module provides a unit struct named [`Instruction`] for implementing
//! this type for any type that implements [`Symbol`] trait.
//!
//! # Examples
//! Creating a new [`Instruction`] through the [`Instruction::new`] method:
//! ```rust
//! use turing_machine_rs::instruction::{Head, Instruction, Move, State, Tail};
//!
//! let head = Head::new(State(1), '0');
//! let tail = Tail::new(State(0), '0', Move::Right);
//!
//! // Moves head and tail from the scope
//! let inst = Instruction::new(head, tail);
//! ```
//!
//! Creating a new [`Instruction`] through the [`Instruction::build`] method:
//! ```rust
//! use turing_machine_rs::instruction::{Instruction, Move, State};
//!
//! let inst = Instruction::build(State(1), '0', State(0), '0', Move::Right);
//! ```

mod head;
mod movement;
mod state;
mod tail;

pub use head::Head;
pub use movement::Move;
pub use state::State;
pub use tail::Tail;

use std::fmt::{Display, Error, Formatter};

use crate::Symbol;

/// [`Instruction`] is a component of [`crate::program::Program`]. This struct
/// contains a [`Head`] struct and a [`Tail`] struct and is used as a unit for
/// instructions in the program.
///
/// [`Instruction`] fileds doesn't needs in control or protection so they
/// are public.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Instruction<S: Symbol> {
    /// The first part of an instruction contains the state [`State`]
    /// and a symbol of the type that implements the [`Symbol`] trait.
    pub head: Head<S>,
    /// The second part of an instruction contains the state [`State`],
    /// a symbol of the type that implements the [`Symbol`] trait
    /// and the movement [`Move`].
    pub tail: Tail<S>,
}

impl<S: Symbol> Instruction<S> {
    /// Constructs a new [`Instruction`] with the [`Head`] and the [`Tail`].
    pub fn new(head: Head<S>, tail: Tail<S>) -> Self {
        Instruction { head, tail }
    }

    /// Builds an [`Instruction`] from the [`Head`] and the [`Tail`] parts.
    pub fn build(
        h_state: State,
        h_symbol: S,
        t_state: State,
        t_symbol: S,
        t_movement: Move,
    ) -> Self {
        Instruction::new(
            Head::new(h_state, h_symbol),
            Tail::new(t_state, t_symbol, t_movement),
        )
    }
}

impl<S: Symbol> Display for Instruction<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "({}) -> ({})", self.head, self.tail)
    }
}
