//! Provides [`Program`] realization for Turing machine.
//!
//! This module provides a [`Program`] which is used for initialization
//! a [`crate::TuringMachine`] and the [`Extend`] trait for the [`Program`]
//! which allows to extend the [`Program`] by tuples of
//! ([`usize`], [`crate::Symbol`], [`usize`], [`crate::Symbol`], [`crate::instruction::Move`]).

mod core;
pub use self::core::Program;

/// Helper trait which allows to implement extend method.
pub trait Extend<I: ?Sized> {
    /// Extends the program with some object depends to realization.
    fn extend(&mut self, iterable: I) -> Result<(), String>;
}
