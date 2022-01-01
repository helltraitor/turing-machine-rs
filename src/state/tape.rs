use std::fmt::{Display, Error, Formatter};
use std::iter::FromIterator;

use crate::Symbol;

#[derive(Clone, Debug, PartialEq)]
pub struct Tape<S: Symbol> {
    tape: Vec<S>,
}

impl<S: Symbol> Tape<S> {
    pub fn new(chars: impl IntoIterator<Item = S>) -> Self {
        Tape {
            tape: Vec::from_iter(chars),
        }
    }

    pub fn as_vec(&self) -> &Vec<S> {
        &self.tape
    }

    pub fn get(&self, index: usize) -> Option<&S> {
        self.tape.get(index)
    }

    pub fn insert(&mut self, index: usize, element: S) {
        self.tape.insert(index, element);
    }

    pub fn is_empty(&self) -> bool {
        // Turing tape cannot be empty but our tape type can
        self.tape.is_empty()
    }

    pub fn len(&self) -> usize {
        self.tape.len()
    }

    pub fn set(&mut self, index: usize, element: S) {
        self.tape[index] = element;
    }
}

impl<S: Symbol> Display for Tape<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}",
            &self
                .tape
                .clone()
                .into_iter()
                .map(|s| s.to_string())
                .collect::<String>()
        )
    }
}

impl From<&str> for Tape<char> {
    fn from(string: &str) -> Self {
        Tape::new(string.chars())
    }
}

impl From<String> for Tape<char> {
    fn from(string: String) -> Self {
        Tape {
            tape: Vec::from_iter(string.chars()),
        }
    }
}

#[cfg(not(feature = "str-as-copy"))]
impl From<&str> for Tape<Box<char>> {
    fn from(string: &str) -> Self {
        Tape::new(string.chars().map(Box::new))
    }
}

#[cfg(not(feature = "string-as-copy"))]
impl From<String> for Tape<Box<char>> {
    fn from(string: String) -> Self {
        Tape {
            tape: Vec::from_iter(string.chars().map(Box::new)),
        }
    }
}
