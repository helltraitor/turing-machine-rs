use std::fmt::{Display, Error, Formatter};

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Center,
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
