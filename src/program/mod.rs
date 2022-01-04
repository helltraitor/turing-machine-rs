//! Provides [`Program`] realization for Turing machine.
//!
//! This module provides [`Program`] which is using for initialization
//! a [`crate::TuringMachine`] and [`Extend`] trait for [`Program`] which
//! can be used for activating additional functional (pretty usefull).

use std::fmt::{Display, Error, Formatter};
use std::mem::replace;

use crate::instruction::{Direction, Head, Instruction, Tail};
use crate::{Symbol, With};

/// Program is a vector-based struct which is implementing minimal api
/// for changing and extending and no api for removinf and shrink. The Program
/// for Turing machine have a constant size which equals to
/// `(STATES.count() - 1) * (ALPHABET.count())`.
///
/// If you want to extend program you can use extend method but you must be sure
/// that this struct can append all these instructions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Program<S: Symbol> {
    container: Vec<Instruction<S>>,
    alphabet: Vec<S>,
    l_state: u32,
}

impl<S: Symbol> Program<S> {
    /// Constructs a new [`Program`] from vector [`Vec<S>`] and last state
    /// [`u32`].
    #[rustfmt::skip]
    pub fn new(alphabet: Vec<S>, l_state: u32) -> Self {
        let capacity = alphabet.len() * (l_state as usize);
        let container = Vec::with_capacity(capacity);
        Program { alphabet, container, l_state }
    }

    /// Returns a alphabet reference. Zero cost.
    pub fn alphabet(&self) -> &Vec<S> {
        &self.alphabet
    }

    /// Returns [`Ok(Some)`] for [`Head`] if it contains in the program,
    /// [`Ok(None)`] if this head is not in the program and [`Err(String)`]
    /// if [`Head`] state is large then [`Program`] last state.
    pub fn get(&self, head: &Head<S>) -> Result<Option<&Instruction<S>>, String> {
        if self.l_state < head.state {
            return Err(format!(
                "get error: required state {} is large then largest {}",
                head.state, self.l_state
            ));
        }
        Ok(self
            .container
            .iter()
            .find(|inst: &&Instruction<S>| &inst.head == head))
    }

    /// Returns the program last state.
    pub fn l_state(&self) -> u32 {
        self.l_state
    }

    #[rustfmt::skip]
    /// Setting [`Instruction`] to the program.
    ///
    /// Returns [`Err(String)`] if [`Head`] state equals to `0`, [`Head`]
    /// or [`Tail`] symbols are not in [`Program`] alphabet or [`Program`]
    /// last state is less then [`Head`] or [`Tail`] states. Otherwise returns
    /// [`Ok(Some(Instruction))`] if the instruction with this [`Head`] already
    /// exitsts or [`Ok(None)`] if the instruction is new for this [`Program`].
    ///
    /// The [`Option`] is very useful in the collision check.
    pub fn insert(&mut self, inst: Instruction<S>) -> Result<Option<Instruction<S>>, String> {
        if inst.head.state == 0 {
            return Err(format!(
                "set error: instruction {} cannot have 0 state in head",
                inst
            ));
        }
        if !self.alphabet.contains(&inst.head.symbol)
            || !self.alphabet.contains(&inst.tail.symbol) {
            return Err(format!(
                "set error: instruction {} not for program with alphabet {:?}",
                inst, &self.alphabet
            ));
        }
        if self.l_state < inst.head.state || self.l_state < inst.tail.state {
            return Err(format!(
                "set error: instruction {} have states which is large then program largest state {}",
                inst, self.l_state
            ));
        }
        let position = self
            .container
            .iter()
            .position(|cand: &Instruction<S>| cand.head == inst.head);
        match position {
            Some(index) => Ok(Some(replace(&mut self.container[index], inst))),
            None => {
                self.container.push(inst);
                Ok(None)
            }
        }
    }
}

/// Helper trait which allows to implement extend method.
pub trait Extend<I: ?Sized> {
    /// Extends the program with some object depends to realization.
    fn extend(&mut self, iterable: I) -> Result<(), String>;
}

impl<S: Symbol> With<Program<S>> for Program<S> {
    type Output = Result<Program<S>, String>;

    /// Merges this program by another according to these rules:
    /// 1. All [`Tail`] parts of instructions of this struct will changes their
    ///     to `self.l_state` if tail state equals to `0`
    /// 2. All [`Head`] parts of instructions of another struct will increase
    ///     by `self.l_state`
    /// 3. All [`Tail`] parts of instructions of another struct will also
    ///     increase by `self.l_state` but only if tail state not equals to `0`
    /// 4. This struct l_state increase by other l_state (really thats happend
    ///     before setting new instructions)
    fn with(&self, other: &Program<S>) -> Result<Program<S>, String> {
        if self.alphabet != other.alphabet {
            return Err(format!(
                "extend error: alphabet {:?} and {:?} must be equal",
                &self.alphabet, &other.alphabet
            ));
        }
        let mut program = Program::new(self.alphabet.clone(), self.l_state + other.l_state);
        // `self` and `other` are `Program` instances so it doesn't need to use insert method.
        let extension = self.container.iter().map(|inst| match inst.tail.state {
            0 => {
                let mut inst = inst.clone();
                inst.tail.state = self.l_state + 1;
                inst
            }
            _ => inst.clone(),
        });
        program.container.extend(extension);

        let extension = other.container.iter().map(|inst| {
            let mut inst = inst.clone();
            inst.head.state += self.l_state;
            inst.tail.state += match inst.tail.state {
                0 => 0,
                _ => self.l_state,
            };
            inst
        });
        program.container.extend(extension);

        Ok(program)
    }
}

impl<S: Symbol, I> Extend<I> for Program<S>
where
    I: IntoIterator<Item = (u32, S, u32, S, Direction)>,
{
    /// Extends the program by tuple `(u32, S, u32, S, Direction)` first two
    /// elements are going to [`Head`] and the last three are going to [`Tail`]
    fn extend(&mut self, iterable: I) -> Result<(), String> {
        for (h_state, h_symbol, t_state, t_symbol, t_direction) in iterable {
            self.insert(Instruction::new(
                Head::new(h_state, h_symbol),
                Tail::new(t_state, t_symbol, t_direction),
            ))?;
        }
        Ok(())
    }
}

impl<S: Symbol> Display for Program<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        use std::any::type_name;

        write!(
            f,
            "Program<{}> {{ alphabet {:?} instuctions: {}, l_state: {} }}",
            type_name::<S>(),
            self.alphabet,
            self.container.len(),
            self.l_state
        )
    }
}
