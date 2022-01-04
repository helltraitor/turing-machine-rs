use std::fmt::{Display, Error, Formatter};

use crate::instruction::{Move, State};
use crate::state::Tape;
use crate::Symbol;

/// [`Configuration`] is a struct that represents the state of a Turing machine.
/// Machines do not implement their state as a part of themselves;
/// instead, machines mutate configurations according to their program.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Configuration<S: Symbol> {
    tape: Tape<S>,
    index: usize,
    /// [`Configuration`] [`State`] is used by [`crate::TuringMachine`]
    /// and cannot be changed by self-methods.
    pub state: State,
}

impl<S: Symbol> Configuration<S> {
    /// Constructs a new [`Configuration`] from the [`Tape`],
    /// the index [`usize`] and the [`State`].
    ///
    /// Returns a new [`Ok(Configuration)`] if the index is within
    /// the bounds of the [`Tape`], otherwise an [`Err(String)`]
    /// with diagnostic information.
    pub fn new(tape: Tape<S>, index: usize, state: State) -> Result<Self, String> {
        match tape.len() > index {
            true => Ok(Configuration { tape, index, state }),
            false => Err(format!(
                "index out of bounds: the len is {} but the index is {}",
                tape.len(),
                index
            )),
        }
    }

    /// Constructs a new [`Configuration`] from the [`Tape`],
    /// the index `0` and the state `1`.
    /// This configuration named `normal` or `nrm`.
    ///
    /// Returns a new [`Ok(Configuration)`] if the [`Tape`] is not empty
    /// otherwise an [`Err(String)`] with diagnostic information.
    pub fn new_nrm(tape: Tape<S>) -> Result<Self, String> {
        Configuration::new(tape, 0, State(1))
    }

    /// Constructs a new [`Configuration`] from the [`Tape`],
    /// the index `tape.len() - 1` and the state: `1`.
    /// This configuration named `standart` or `std`.
    ///
    /// Returns a new [`Ok(Configuration)`] if the [`Tape`] is not empty
    /// otherwise an [`Err(String)`] with diagnostic information.
    pub fn new_std(tape: Tape<S>) -> Result<Self, String> {
        let last = tape.len() - 1;
        Configuration::new(tape, last, State(1))
    }

    /// Destructs [`Configuration`] into `(Tape<S>, usize, State)`. May be used
    /// only with owned values.
    pub fn destruct(self) -> (Tape<S>, usize, State) {
        (self.tape, self.index, self.state)
    }

    /// Returns the [`Tape`] reference of the [`Configuration`].
    ///
    /// Zero cost method.
    pub fn tape(&self) -> &Tape<S> {
        &self.tape
    }

    /// Returns the [`Tape`] copy of the [`Configuration`].
    pub fn into_tape(self) -> Tape<S> {
        self.tape
    }

    /// Returns the current symbol reference. This reference is always exists.
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
            .expect("get_symbol error: returned value must be Some because of bound checking")
    }

    /// Returns the current [`Tape`] index. This value is always in tape bounds.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns `true` if the [`Tape`] is not empty, otherwise `false`.
    ///
    /// Note that Turing [`Tape`] cannot be empty but this method can return
    /// `false` (because [`Tape`] type is based on the [`Vec`] type).
    pub fn is_empty(&self) -> bool {
        self.tape.is_empty()
    }

    /// Returns the [`Tape`] length.
    pub fn len(&self) -> usize {
        self.tape.len()
    }

    /// Sets [`Symbol`] at index position in the [`Tape`].
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

    /// Shifts the [`Tape`] to left or right if [`Move`] is [`Move::Left`]
    /// or [`Move::Right`], otherwise do nothing (when [`Move::None`]).
    /// If [`Configuration`] reachs the begin or the end of the [`Tape`]
    /// then [`Tape`] extends by [`Tape::insert`] method, otherwise only
    /// changes self index.
    pub fn shift(&mut self, movement: Move, default: S) {
        match movement {
            Move::Left if self.index == 0 => self.tape.insert(0, default),
            Move::Left => self.index -= 1,
            Move::None => {}
            Move::Right => {
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
