use std::fmt::{Display, Error, Formatter};
use std::iter::FromIterator;

use crate::Symbol;

/// Tape is the main part of [`crate::state::Configuration`].
/// This is the [`Vec`] wrapper with similar methods.
///
/// Tape could be created within [`Tape::new`] with IntoIterator
/// or within [`Tape::from`] as [`Tape<char>`] but only for [`str`] and [`String`] types.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tape<S: Symbol> {
    tape: Vec<S>,
}

impl<S: Symbol> Tape<S> {
    /// Constructs a new [`Tape`] from [`IntoIterator`].
    /// Tape is wrapper over [`Vec`] that's why tape can grow.
    #[rustfmt::skip]
    pub fn new(symbols: impl IntoIterator<Item = S>) -> Self {
        Tape { tape: Vec::from_iter(symbols) }
    }

    /// Returns a immutable reference to inner container. Zero cost method.
    pub fn as_vec(&self) -> &Vec<S> {
        &self.tape
    }

    /// Returns [`Option::Some`] when index is in [`Tape`] bounds otherwise
    /// [`Option::None`].
    pub fn get(&self, index: usize) -> Option<&S> {
        self.tape.get(index)
    }

    /// Inserts `element` at `index` position. Moves all items to rigth
    /// when `index` is smaller then length of the [`Tape`].
    ///
    /// # Panics
    /// Panics when `index` is large then [`Tape::len`].
    pub fn insert(&mut self, index: usize, element: S) {
        self.tape.insert(index, element);
    }

    /// Returns `true` if tape contains symbols otherwise `false`.
    /// Note that Turing tape cannot be empty but this method can return `false`
    /// (because tape type is based on container type).
    pub fn is_empty(&self) -> bool {
        self.tape.is_empty()
    }

    /// Returns length of the [`Tape`]. Symbols cannot be removed from the Tape,
    /// so length is always grow.
    pub fn len(&self) -> usize {
        self.tape.len()
    }

    /// Sets `element` at `index` position.
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
    /// Constructs [`Tape<char>`] from [`str`]. Always available but needs using
    /// type annotation [`Tape<char>`] when feature `str-as-copy` is not using.
    fn from(string: &str) -> Self {
        Tape::new(string.chars())
    }
}

impl From<String> for Tape<char> {
    /// Constructs [`Tape<char>`] from [`String`]. Always available but needs using
    /// type annotation [`Tape<char>`] when feature `string-as-copy` is not using.
    fn from(string: String) -> Self {
        Tape::new(string.chars())
    }
}
