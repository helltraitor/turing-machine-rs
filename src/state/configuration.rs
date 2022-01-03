use std::fmt::{Display, Error, Formatter};

use crate::instruction::Direction;
use crate::state::Tape;
use crate::Symbol;

/// Configuration is a struct which implements Turing machine state.
/// Machines do not implement their state as a part of self, instead
/// machines mutates configurations according to their program.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Configuration<S: Symbol> {
    tape: Tape<S>,
    index: usize,
    /// [`Configuration`] state is using by [`crate::TuringMachine`]
    /// and cannot be changed by self methods.
    pub state: u32,
}

impl<S: Symbol> Configuration<S> {
    /// Constructs a new [`Configuration`] from tape: [`Tape`],
    /// current index: [`usize`] and current state: [`u32`].
    ///
    /// Returns a new [`Ok(Configuration)`] if index is in tape bounds,
    /// otherwise a [`Err(String)`] with diagnostic information.
    pub fn new(tape: Tape<S>, index: usize, state: u32) -> Result<Self, String> {
        match tape.len() > index {
            true => Ok(Configuration { tape, index, state }),
            false => Err(format!(
                "index out of bounds: the len is {} but the index is {}",
                tape.len(),
                index
            )),
        }
    }

    /// Constructs a new [`Configuration`] from tape: [`Tape`],
    /// current index: `0` and current state: `1`.
    /// This configuration named `normal` or `nrm`.
    ///
    /// Returns a new [`Ok(Configuration)`] if the tape is not empty
    /// otherwise a [`Err(String)`] with diagnostic information.
    pub fn new_nrm(tape: Tape<S>) -> Result<Self, String> {
        Configuration::new(tape, 0, 1)
    }

    /// Constructs a new [`Configuration`] from tape: [`Tape`],
    /// current index: `tape.len() - 1` and current state: `1`.
    /// This configuration named `standart` or `std`.
    ///
    /// Returns a new [`Ok(Configuration)`] if the tape is not empty
    /// otherwise a [`Err(String)`] with diagnostic information.
    pub fn new_std(tape: Tape<S>) -> Result<Self, String> {
        let last = tape.len() - 1;
        Configuration::new(tape, last, 1)
    }

    /// Destructs [`Configuration`] into `(Tape<S>, usize, u32)`. May be used
    /// only with owned values, not a borrowed.
    pub fn destruct(self) -> (Tape<S>, usize, u32) {
        (self.tape, self.index, self.state)
    }

    /// Returns a [`Tape`] reference of current [`Configuration`]. Zero cost method.
    pub fn tape(&self) -> &Tape<S> {
        &self.tape
    }

    /// Returns a [`Tape`] copy of current [`Configuration`].
    pub fn into_tape(self) -> Tape<S> {
        self.tape
    }

    /// Returns a current symbol reference. This reference is always exists.
    ///
    /// # Panics
    /// [`Configuration`] could panic only if source code is broken - this
    /// would be a bug. [`Configuration`] controls its inner index so it always
    /// valid.
    ///
    /// So you could open an issue on [GitHub](https://github.com/Helltraitor/turing-machine-rs).
    pub fn get_symbol(&self) -> &S {
        self.tape
            .get(self.index)
            .expect("returned value must be not None because of bound checking")
    }

    /// Returns a current tape index. This value is always in tape bounds.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns `true` if tape contains symbols otherwise `false`.
    ///
    /// Note that Turing tape cannot be empty but this method can return `false`
    /// (because tape type is based on container type).
    pub fn is_empty(&self) -> bool {
        self.tape.is_empty()
    }

    /// Returns length of the [`Tape`].
    pub fn len(&self) -> usize {
        self.tape.len()
    }

    /// Sets `element` at `index` position in [`Tape`].
    ///
    /// # Panics
    /// [`Configuration`] could panic only if source code is broken - this
    /// would be a bug. [`Configuration`] controls its inner index so it always
    /// valid.
    ///
    /// So you could open an issue on [GitHub](https://github.com/Helltraitor/turing-machine-rs).
    pub fn set_symbol(&mut self, symbol: S) {
        self.tape.set(self.index, symbol);
    }

    /// Shifts [`Tape`] to left, right or stands still. If [`Configuration`]
    /// reachs begin or end of the tape then tape is extends by [`Tape::insert`]
    /// method, otherwise only changes self index.
    pub fn shift(&mut self, direction: Direction, default: S) {
        match direction {
            Direction::Left if self.index == 0 => self.tape.insert(0, default),
            Direction::Left => self.index -= 1,
            Direction::Center => {}
            Direction::Right => {
                self.index += 1;
                if self.index == self.tape.len() {
                    self.tape.insert(self.index, default);
                }
            }
        };
    }
}

impl<S: Symbol> Display for Configuration<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "Configuration {{ Tape: \"{}\", Index: {}, State: {} }}",
            self.tape, self.index, self.state
        )
    }
}
