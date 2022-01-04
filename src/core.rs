//! Provides core traits: [`Symbol`], [`TuringMachine`] and [`With`].
//!
//! Short description:
//! - [`Symbol`] is a base of Turing machine which allows to use any types
//!     (almost).
//! - [`TuringMachine`] is a most important trait which allows to implement
//!     Turing machine behaviour and use it within other [`TuringMachine`]
//!     implementations (e.g. [`crate::machines::Classic`]).
//! - [`With`] is a abstract trait which is using for flexible concatenation
//!     two and more [`crate::machines::Classic`] Turing machines (optional).
//!
//! [`Symbol`] and [`TuringMachine`] are neccesry but [`With`] only optional
//! and can be raplaced with (ho-ho) [`Vec`] or another container.

use std::fmt::{Debug, Display};

/// [`With`] trait provides ability to concatenate several [`TuringMachine`]
/// into one another Turing machine. This trait must be implemented individual
/// for machine types if it needs.
///
/// For using examples see [`crate::machines::Classic`].
pub trait With<T> {
    /// Output type may have several variantions accoriding to needs.
    type Output;

    /// Accepts machine type of `&T` and returns `Output` instance.
    /// Output must be superpostion of self and T.
    fn with(&self, other: &T) -> Self::Output;
}

/// [`Symbol`] provides ability to use whatever you want as symbol
/// of [`TuringMachine`] alhabet.
///
/// One of most important traits.
pub trait Symbol: Clone + Debug + Display + Eq + PartialEq {}

impl<T> Symbol for T where T: Clone + Debug + Display + Eq + PartialEq {}
