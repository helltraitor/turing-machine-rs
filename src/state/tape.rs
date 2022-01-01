use std::fmt::{Display, Error, Formatter};
use std::iter::FromIterator;

use crate::Symbol;

/// Tape is the main part of [`crate::state::Configuration`].
/// This is the [`Vec`] wrapper with similar methods.
///
/// Tape could be created within [`Tape::new`] with IntoIterator
/// or within [`Tape::from`] but only for [`str`] and [`String`] types.
/// In the second case, you must be sure that you use type annotation
/// or use `str-as-copy` \ `string-as-copy` features.
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

    /// Returns [`Option`] which is [`Option::None`] when index is large
    /// then last index of the element and [`Option::Some`] when index
    /// in bounds.
    pub fn get(&self, index: usize) -> Option<&S> {
        self.tape.get(index)
    }

    /// Inserts `element` at `index` position. Moves all items to rigth
    /// when `index` is smaller then length of the [`Tape`].
    pub fn insert(&mut self, index: usize, element: S) {
        self.tape.insert(index, element);
    }

    /// Returns `true` if tape contains symbols otherwise `false`.
    /// Note that Turing tape cannot be empty but this method can return `false`
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
    ///
    /// # Example
    /// ```rust
    /// use turing_machine_rs::state::Tape;
    ///
    /// fn main() {
    ///     let mut tape = Tape::new(['0']);
    ///     // This is fine
    ///     tape.set(0, '1');
    ///     // This will panic!
    ///     // tape.set(1, '2');
    /// }
    /// ```
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
    ///
    /// # Example
    /// ```rust
    /// use turing_machine_rs::state::Tape;
    ///
    /// fn main() {
    ///     let _: Tape<char> = Tape::from("test");
    /// }
    /// ```
    fn from(string: &str) -> Self {
        Tape::new(string.chars())
    }
}

impl From<String> for Tape<char> {
    /// Constructs [`Tape<char>`] from [`String`]. Always available but needs using
    /// type annotation [`Tape<char>`] when feature `string-as-copy` is not using.
    ///
    /// # Example
    /// ```rust
    /// use turing_machine_rs::state::Tape;
    ///
    /// fn main() {
    ///     let _: Tape<char> = Tape::from(String::from("test"));
    /// }
    /// ```
    fn from(string: String) -> Self {
        Tape::new(string.chars())
    }
}

#[cfg(not(feature = "str-as-copy"))]
impl From<&str> for Tape<Box<char>> {
    /// Constructs [`Tape<Box<char>>`] from [`str`]. Available when feature
    /// `str-as-copy` is not using and needs using type annotation
    /// [`Tape<Box<char>>`]. Most cases of using this 'feature' is in tests
    /// as proof of concept. There is no reason to use more complicated
    /// [`Clone`] only structs.
    ///
    /// # Example
    /// ```rust
    /// use turing_machine_rs::state::Tape;
    ///
    /// fn main() {
    ///     let _: Tape<Box<char>> = Tape::from("test");
    /// }
    /// ```
    fn from(string: &str) -> Self {
        Tape::new(string.chars().map(Box::new))
    }
}

#[cfg(not(feature = "string-as-copy"))]
impl From<String> for Tape<Box<char>> {
    /// Constructs [`Tape<Box<char>>`] from [`String`]. Available when feature
    /// `string-as-copy` is not using and needs using type annotation
    /// [`Tape<Box<char>>`]. Most cases of using this 'feature' is in tests
    /// as proof of concept. There is no reason to use more complicated
    /// [`Clone`] only structs.
    ///
    /// # Example
    /// ```rust
    /// use turing_machine_rs::state::Tape;
    ///
    /// fn main() {
    ///     let _: Tape<Box<char>> = Tape::from(String::from("test"));
    /// }
    /// ```
    fn from(string: String) -> Self {
        Tape::new(string.chars().map(Box::new))
    }
}
