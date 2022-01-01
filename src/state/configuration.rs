use std::fmt::{Display, Error, Formatter};

use crate::instruction::Direction;
use crate::state::Tape;
use crate::Symbol;

#[derive(Clone, Debug, PartialEq)]
pub struct Configuration<S: Symbol> {
    tape: Tape<S>,
    index: usize,
    pub state: u32,
}

impl<S: Symbol> Configuration<S> {
    pub fn new(tape: Tape<S>, index: usize, state: u32) -> Self {
        assert!(
            tape.len() > index,
            "index out of bounds: the len is {} but the index is {}",
            tape.len(),
            index
        );
        Configuration { tape, index, state }
    }

    pub fn new_nrm(tape: Tape<S>) -> Configuration<S> {
        Configuration::new(tape, 0, 1)
    }

    pub fn new_std(tape: Tape<S>) -> Configuration<S> {
        let last = tape.len() - 1;
        Configuration::new(tape, last, 1)
    }

    pub fn destruct(self) -> (Tape<S>, usize, u32) {
        (self.tape, self.index, self.state)
    }

    pub fn tape(&self) -> &Tape<S> {
        &self.tape
    }

    pub fn into_tape(self) -> Tape<S> {
        self.tape
    }

    pub fn get_symbol(&self) -> &S {
        self.tape
            .get(self.index)
            .expect("Returned value must be not None because of bound checking")
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn is_empty(&self) -> bool {
        // Turing tape cannot be empty but our tape type can
        self.tape.is_empty()
    }

    pub fn len(&self) -> usize {
        self.tape.len()
    }

    pub fn set_symbol(&mut self, symbol: S) {
        self.tape.set(self.index, symbol);
    }

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
