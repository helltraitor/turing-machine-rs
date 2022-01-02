use std::fmt::{Display, Error, Formatter};

/// Direction is a part of the [`crate::instruction::Tail`] and it's uses by
/// [`crate::state::Configuration`] for moving. For any others structs direction
/// value have no matter. Turing Machine RS can works only with 1 dimension tape.
/// That's why [`Direction`] contains two directions ([`Direction::Left`],
/// [`Direction::Right`]) and one [`Direction::Center`] variant.
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Direction {
    #[allow(missing_docs)]
    Left,
    #[allow(missing_docs)]
    Center,
    #[allow(missing_docs)]
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Direction::Left => write!(f, "<"),
            Direction::Center => write!(f, "^"),
            Direction::Right => write!(f, ">"),
        }
    }
}
