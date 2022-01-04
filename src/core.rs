use std::fmt::{Debug, Display};

/// [`With`] trait provides the ability to concatenate several [`crate::TuringMachine`]
/// into one another Turing machine. This trait must be implemented individual
/// for machine types if it needs.
pub trait With<T> {
    /// Output type may have several variantions accoriding to the needs.
    type Output;

    /// Accepts machine type of `&T` and returns `Output` instance.
    /// Output must be superpostion of self and T.
    fn with(&self, other: &T) -> Self::Output;
}

/// [`Symbol`] provides the ability to use whatever you want (almost)
/// as a symbol of the [`crate::TuringMachine`] alhabet.
///
/// One of most important traits.
pub trait Symbol: Clone + Debug + Display + Eq + PartialEq {}

impl<T> Symbol for T where T: Clone + Debug + Display + Eq + PartialEq {}
