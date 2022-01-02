//! Provides [`Instruction`] and it's components: [`Head`], [`Tail`] and [`Direction`].
//!
//! This module provides unit struct named [`Instruction`] for implementing
//! this type for any types which implements [`Symbol`] trait.
//!
//! Instruction type and components doesn't know about meaning of their
//! fields for another structs. This module doesn't provides any checks
//! and warranties except of no panic, no errors and no self changing
//! (no one of methods can change these structs).
//!
//! All of these structs could be used without type annotations.

mod direction;
mod head;
mod tail;

pub use direction::Direction;
pub use head::Head;
pub use tail::Tail;

use std::fmt::{Display, Error, Formatter};

use crate::Symbol;

/// Instruction is a part of the [`crate::program::Program`]. This struct
/// contains head and tail structs and is used as unit for instructions in
/// program. Instruction fileds doesn't needs in control or protection so they
/// are public.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Instruction<S: Symbol> {
    /// First part of instruction, contains state [`u32`]
    /// and symbol of type which implements [`Symbol`] trait.
    pub head: Head<S>,
    /// First part of instruction, contains state [`u32`],
    /// symbol of type which implements [`Symbol`] trait
    /// and direction [`Direction`].
    pub tail: Tail<S>,
}

impl<S: Symbol> Instruction<S> {
    /// Constructs a new [`Instruction`] with head and tail. Instruction struct
    /// constructs immediatly that's why doesn't need to use type annotations.
    pub fn new(head: Head<S>, tail: Tail<S>) -> Self {
        Instruction { head, tail }
    }
}

impl<S: Symbol> Display for Instruction<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "({}) -> ({})", self.head, self.tail)
    }
}
