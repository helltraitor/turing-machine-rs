//! Program implementation for Turing Machine RS
//!
//! This module provides [`Program`] which is using for initialization
//! a [`crate::TuringMachine`] and [`ExtendBy`] trait for [`Program`] which
//! can be used for activating additional functional (pretty usefull).
//!
//! # Warning
//! [`Program`] can panic!
//!
//! # Examples
//! ```rust
//! use turing_machine_rs::instruction::Direction;
//! use turing_machine_rs::program::{ExtendBy, Program};
//!
//! fn main() {
//!     let mut program = Program::new(vec!['0', '1'], 3);
//!     program.extend_by([
//!         (1, '0', 1, '0', Direction::Right),
//!         (1, '1', 2, '1', Direction::Right),
//!         (2, '0', 3, '0', Direction::Left),
//!         (2, '1', 2, '1', Direction::Right),
//!         (3, '0', 0, '0', Direction::Center),
//!         (3, '1', 3, '0', Direction::Left)
//!     ]);
//! }
//! ```

use std::fmt::{Display, Error, Formatter};

use crate::instruction::{Direction, Head, Instruction, Tail};
use crate::Symbol;

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
    ///
    /// # Panics
    /// Panics when `alphabet.is_empty` or l_state equals to `0`.
    ///
    /// # Example
    /// ```rust
    /// use turing_machine_rs::program::Program;
    ///
    /// fn main() {
    ///     // This is fine
    ///     let _ = Program::new(vec!['0'], 1);
    ///     // These will panic!
    ///     // let _ = Program::new(vec![], 1);
    ///     // let _ = Program::new(vec!['0'], 0);
    /// }
    /// ```
    #[rustfmt::skip]
    pub fn new(alphabet: Vec<S>, l_state: u32) -> Self {
        assert!(!alphabet.is_empty(), "new error: alphabet cannot be empty");
        assert!(
            l_state > 0,
            "new error: l_state must have (be) 1 state at least (start)"
        );

        let capacity = alphabet.len() * (l_state as usize);
        let container = Vec::with_capacity(capacity);
        Program { alphabet, container, l_state }
    }

    /// Returns a alphabet reference. Zero cost.
    pub fn alphabet(&self) -> &Vec<S> {
        &self.alphabet
    }

    /// Extends this program by another according to these rules:
    /// 1. All [`Tail`] parts of instructions of this struct will changes their
    ///     to `self.l_state` if tail state equals to `0`
    /// 2. All [`Head`] parts of instructions of another struct will increase
    ///     by `self.l_state`
    /// 3. All [`Tail`] parts of instructions of another struct will also
    ///     increase by `self.l_state` but only if tail state not equals to `0`
    /// 4. This struct l_state increase by other l_state (really thats happend
    ///     before setting new instructions)
    /// # Panics
    /// Panics when alphabets are different or when this struct can store such
    /// many instructions. The last is depend to l_state count.
    ///
    /// # MUST NOT BE USED
    pub fn extend(&mut self, other: &Program<S>) {
        assert!(
            self.alphabet == other.alphabet,
            "extend error: alphabet {:?} and {:?} must be equal",
            &self.alphabet,
            &other.alphabet
        );
        assert!(
            self.container.len() + other.container.len() > self.container.capacity(),
            "extend error: program type has limited size (count of alphabet * (count of states - 1))"
        );
        let old_l_state = self.l_state;
        self.l_state += other.l_state;

        for inst in self.container.iter_mut() {
            if inst.tail.state == 0 {
                inst.tail.state = old_l_state + 1;
            }
        }

        for inst in other.container.iter() {
            let mut inst = inst.clone();
            inst.head.state += old_l_state;
            inst.tail.state += match inst.tail.state {
                0 => 0,
                _ => old_l_state,
            };
            self.set(inst);
        }
    }

    /// Returns [`Some(Instruction)`] for [`Head`] if it exitsts in the program
    /// otherwise [`None`].
    ///
    /// # Panics
    /// Panics when head l_state is large then self l_state.
    pub fn get(&self, head: &Head<S>) -> Option<&Instruction<S>> {
        assert!(
            self.l_state >= head.state,
            "get error: required state {} is large then largest {}",
            head.state,
            self.l_state
        );
        self.container
            .iter()
            .find(|inst: &&Instruction<S>| &inst.head == head)
    }

    /// Returns the program last state.
    pub fn l_state(&self) -> u32 {
        self.l_state
    }

    /// Setting [`Instruction`] to the program.
    ///
    /// # Panics
    /// Panics when head state equals to zero, head or tail symbol
    /// is not in the alphabet or `self.l_state` is less the head or tail state.
    pub fn set(&mut self, inst: Instruction<S>) {
        assert!(
            inst.head.state != 0,
            "set error: instruction {} cannot have 0 state in head",
            inst
        );
        assert!(
            self.alphabet.contains(&inst.head.symbol) && self.alphabet.contains(&inst.tail.symbol),
            "set error: instruction {} not for program with alphabet {:?}",
            inst,
            &self.alphabet
        );
        assert!(
            self.l_state >= inst.head.state && self.l_state >= inst.tail.state,
            "set error: instruction {} have states which is large then program largest state {}",
            inst,
            self.l_state
        );
        let position = self
            .container
            .iter()
            .position(|cand: &Instruction<S>| cand.head == inst.head);
        match position {
            Some(index) => self.container[index] = inst,
            None => self.container.push(inst),
        };
    }
}

/// Helper trait which allows to implement extend_by method.
pub trait ExtendBy<I: ?Sized> {
    /// Extends the program with some object depends to realization.
    fn extend_by(&mut self, iterable: I);
}

impl<S: Symbol, I> ExtendBy<I> for Program<S>
where
    I: IntoIterator<Item = (u32, S, u32, S, Direction)>,
{
    /// Extends the program by tuple `(u32, S, u32, S, Direction)` first two
    /// elements are going to [`Head`] and the last three are going to [`Tail`]
    fn extend_by(&mut self, iterable: I) {
        for (h_state, h_symbol, t_state, t_symbol, t_direction) in iterable {
            self.set(Instruction::new(
                Head::new(h_state, h_symbol),
                Tail::new(t_state, t_symbol, t_direction),
            ));
        }
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
