use std::fmt::{Display, Error, Formatter};
use std::ops::{Add, AddAssign};

/// Wrapper around the [`usize`] type. It is necessary for the decrease in value
/// mismatching (when several values have the same type but two different meanings)
/// but without the type limit.
///
/// Earlier implementations used the [`u32`] type when [`usize`] was only used for
/// indexes.Now there is no limit for type usage (it was a manual limit against
/// possible program bugs).
#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct State(pub usize);

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.0)
    }
}

impl Add for State {
    type Output = State;

    fn add(self, rhs: Self) -> Self {
        State(self.0 + rhs.0)
    }
}

impl AddAssign for State {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}
