use std::fmt::{Display, Error, Formatter};

/// Move is a part of the [`crate::instruction::Tail`] and it's used for
/// tape shift direction determining by [`crate::state::Configuration`].
/// For any others structs direction value have no matter.
///
/// Turing Machine RS can works only with 1 dimension tape. That's why
/// [`Move`] enumeration contains two directions variant:
/// [`Move::Left`], [`Move::Right`].
///
/// Istead of using [`Option`], this enumeration provides one more
/// variant - [`Move::None`].
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Move {
    #[allow(missing_docs)]
    Left,
    #[allow(missing_docs)]
    Right,
    #[allow(missing_docs)]
    None,
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Move::Left => write!(f, "<"),
            Move::None => write!(f, "-"),
            Move::Right => write!(f, ">"),
        }
    }
}
