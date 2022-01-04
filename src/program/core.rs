use std::fmt::{Display, Error, Formatter};
use std::mem::replace;

use crate::instruction::{Head, Instruction, Move, State};
use crate::program::Extend;
use crate::{Symbol, With};

/// [`Program`] is a vector-based struct with a limited API for changing
/// and extending but no API for removing and shrinking. The [`Program`]
/// for the Turing machine has a constant size that equals to
/// `(STATES.count() - 1) * (ALPHABET.count())`.
///
/// If you want to extend the program, you can use the [`Extend::extend`] method,
/// but you should be sure that this program can accept all these instructions.
///
/// # Example
/// ```rust
/// extern crate turing_machine_rs;
/// use turing_machine_rs::instruction::{Move, State};
/// use turing_machine_rs::machines::Classic;
/// use turing_machine_rs::program::{Extend, Program};
/// use turing_machine_rs::state::Tape;
/// use turing_machine_rs::TuringMachine;
///
/// fn main() -> Result<(), String> {
///    let alphabet = vec!['t', 'e', 's', 'n', 'i', 'c', 'e', '_'];
///    let mut program = Program::new(alphabet, State(4));
///     program.extend([
///         (1, 't', 2, 'n', Move::Right),
///         (2, 'e', 3, 'i', Move::Right),
///         (3, 's', 4, 'c', Move::Right),
///         (4, 't', 0, 'e', Move::None),
///         // Revers
///         (1, 'n', 2, 't', Move::Right),
///         (2, 'i', 3, 'e', Move::Right),
///         (3, 'c', 4, 's', Move::Right),
///         (4, 'e', 0, 't', Move::None),
///     ])?;
///     let machine = Classic::new(program, '_')?;
///
///     let test = Tape::from("test");
///     let nice = machine.translate_nrm(test.clone())?;
///     println!(
///         "{} {}!",
///         String::from_iter(nice.as_vec()),
///         String::from_iter(test.as_vec())
///     );
///     Ok(())
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Program<S: Symbol> {
    container: Vec<Instruction<S>>,
    alphabet: Vec<S>,
    l_state: State,
}

impl<S: Symbol> Program<S> {
    #[rustfmt::skip]
    /// Constructs a new [`Program`] with the vector [`Vec<S>`]
    /// and the last state [`State`].
    ///
    /// [`Program`] has a limited size by definition, so it can only hold `(STATES.count() - 1) * (ALPHABET.count())` [`Instruction`]s.
    pub fn new(alphabet: Vec<S>, l_state: State) -> Self {
        let capacity = alphabet.len() * l_state.0;
        let container = Vec::with_capacity(capacity);
        Program { alphabet, container, l_state }
    }

    /// Returns an [`Vec`] alphabet reference.
    ///
    /// Zero cost method.
    pub fn alphabet(&self) -> &Vec<S> {
        &self.alphabet
    }

    /// Returns [`Ok(Some)`] when [`Head`] is in the program,
    /// [`Ok(None)`] when [`Head`] is not in the program
    /// and [`Err(String)`] when [`Head`] [`State`] is large
    /// then the [`Program`] last state.
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

    /// Returns [`State`] the program last state.
    pub fn l_state(&self) -> State {
        self.l_state
    }

    #[rustfmt::skip]
    /// Inserts [`Instruction`] in the [`Program`].
    ///
    /// Returns [`Err(String)`] when [`Head`] [`State`] equals to `0`,
    /// [`Head`] or [`Tail`] symbols are not in the [`Program`] alphabet
    /// or the [`Program`] last state is less then [`Head`] or [`crate::instruction::Tail`] states.
    ///
    /// Otherwise returns another [`Ok(Some(Instruction))`] when the [`Head`]
    /// already is in the [`Program`] and set inserting [`Instruction`]
    /// or [`Ok(None)`] when the [`Instruction`] is not in the [`Program`].
    ///
    /// The [`Option`] is very useful in the collision check.
    pub fn insert(&mut self, inst: Instruction<S>) -> Result<Option<Instruction<S>>, String> {
        if inst.head.state == State(0) {
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

impl<S: Symbol> With<Program<S>> for Program<S> {
    type Output = Result<Program<S>, String>;

    /// Returns a new [`Program`] by merging this program with another according to these rules:
    /// 1. All [`crate::instruction::Tail`] parts of [`Instruction`]s for this [`Program`]
    ///     will changes their [`State`]s to `self.l_state` if [`crate::instruction::Tail`]
    ///     [`State`] equals to `0`.
    /// 2. All [`Head`] parts of [`Instruction`]s for another [`Program`] will
    ///     increase (or shift) their [`State`]s by `self.l_state`.
    /// 3. All [`crate::instruction::Tail`] parts of [`Instruction`]s
    ///     for another program will also increase (or shift) by `self.l_state`
    ///     but only if [`crate::instruction::Tail`] [`State`] not equals to `0`.
    /// 4. A new [`Program`] `l_state` is set to `self.l_state + other.l_state`.
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
            State(0) => {
                let mut inst = inst.clone();
                inst.tail.state = self.l_state + State(1);
                inst
            }
            _ => inst.clone(),
        });
        program.container.extend(extension);

        let extension = other.container.iter().map(|inst| {
            let mut inst = inst.clone();
            inst.head.state += self.l_state;
            inst.tail.state += match inst.tail.state {
                State(0) => State(0),
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
    I: IntoIterator<Item = (usize, S, usize, S, Move)>,
{
    /// Extends the [`Program`] by tuples of ([`usize`], [`Symbol`], [`usize`],
    /// [`Symbol`], [`Move`]) the first two elements are going to [`Head`]
    /// and the last three are going to [`crate::instruction::Tail`].
    ///
    /// Returns [`Ok(())`] when the [`Program`] is extended successfully
    /// and the [`Err(String)`] otherwise.
    ///
    /// # Warning
    /// When the [`Instruction`] can be inserted into the [`Program`]
    /// the extending interrupt.
    fn extend(&mut self, iterable: I) -> Result<(), String> {
        for (h_state, h_symbol, t_state, t_symbol, t_movement) in iterable {
            self.insert(Instruction::build(
                State(h_state),
                h_symbol,
                State(t_state),
                t_symbol,
                t_movement,
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
