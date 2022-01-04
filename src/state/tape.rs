use std::fmt::{Display, Error, Formatter};
use std::iter::FromIterator;

use crate::Symbol;

/// [`Tape`] type is the main part of the [`crate::state::Configuration`]
/// and it is based on the [`Vec`] type and has similar methods.
///
/// [`Tape`] can be created with the [`Tape::new`] method and [`IntoIterator`]
/// object or with the [`Tape::from`] method as a [`Tape<char>`] but only
/// for [`str`] and [`String`] types.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tape<S: Symbol> {
    tape: Vec<S>,
}

impl<S: Symbol> Tape<S> {
    /// Constructs a new [`Tape`] from [`IntoIterator`] object.
    #[rustfmt::skip]
    pub fn new(symbols: impl IntoIterator<Item = S>) -> Self {
        Tape { tape: Vec::from_iter(symbols) }
    }

    /// Returns an immutable [`Vec`] reference to the inner container.
    ///
    /// Zero cost method.
    pub fn as_vec(&self) -> &Vec<S> {
        &self.tape
    }

    /// Returns [`Option::Some`] when the index is in the [`Tape`] bounds,
    /// otherwise [`Option::None`].
    pub fn get(&self, index: usize) -> Option<&S> {
        self.tape.get(index)
    }

    /// Inserts the element that implements the [`Symbol`] trait
    /// at the index [`usize`] position. When the index is less
    /// than the length of the [`Tape`], all items are moved to the right.
    ///
    /// # Panics
    /// Panics when the index is large then the [`Tape`] length.
    pub fn insert(&mut self, index: usize, element: S) {
        self.tape.insert(index, element);
    }

    /// Returns `true` if the [`Tape`] contains symbols, otherwise `false`.
    ///
    /// Note that Turing tape cannot be empty but this method can return `false`
    /// (because the [`Tape`] type is based on the [`Vec`] type).
    pub fn is_empty(&self) -> bool {
        self.tape.is_empty()
    }

    /// Returns the [`Tape`] length. Because symbols cannot be removed
    /// from the [`Tape`] (only replaced), the length of the [`Tape`]
    /// grows indefinitely.
    pub fn len(&self) -> usize {
        self.tape.len()
    }

    /// Sets the [`Symbol`] element at index [`usize`] position.
    ///
    /// # Panics
    /// Panic if the index is out of bounds.
    pub fn set(&mut self, index: usize, element: S) {
        self.tape[index] = element;
    }
}

impl<S: Symbol> Display for Tape<S> {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", &self.tape.clone()
                                  .into_iter()
                                  .map(|s| s.to_string())
                                  .collect::<String>())
    }
}

impl From<&str> for Tape<char> {
    fn from(string: &str) -> Self {
        Tape::new(string.chars())
    }
}

impl From<String> for Tape<char> {
    fn from(string: String) -> Self {
        Tape::new(string.chars())
    }
}
