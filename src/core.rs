//! Provides core traits: [`Symbol`], [`TuringMachine`] and [`With`].
//!
//! Short description:
//! - [`Symbol`] is a base of Turing machine which allows to use any types
//!     (almost).
//! - [`TuringMachine`] is a most important trait which allows to implement
//!     Turing machine behaviour and use it within other [`TuringMachine`]
//!     implementations (e.g. [`crate::machines::Classic`]).
//! - [`With`] is a abstract trait which is using for flexible concatenation
//!     two and more [`crate::machines::Classic`] Turing machines (optional).
//!
//! [`Symbol`] and [`TuringMachine`] are neccesry but [`With`] only optional
//! and can be raplaced with (ho-ho) [`Vec`] or another container.

use std::fmt::{Debug, Display};

use crate::state::{Configuration, Tape};

/// [`With`] trait provides ability to concatenate several [`TuringMachine`]
/// into one another Turing machine. This trait must be implemented individual
/// for machine types if it needs.
///
/// For using examples see [`crate::machines::Classic`].
pub trait With<T> {
    /// Output type may have several variantions accoriding to needs.
    type Output;

    /// Accepts machine type of `&T` and returns `Output` instance.
    /// Output must be superpostion of self and T.
    fn with(&self, other: &T) -> Self::Output;
}

/// [`Symbol`] provides ability to use whatever you want as symbol
/// of [`TuringMachine`] alhabet.
///
/// One of most important traits.
pub trait Symbol: Clone + Debug + Display + Eq + PartialEq {}

impl<T> Symbol for T where T: Clone + Debug + Display + Eq + PartialEq {}

/// Provides ability to execute [`crate::state::Configuration`]s and translate
/// [`crate::state::Tape`]s.
///
/// Most of the methods can be implement through the [`TuringMachine::execute_once`]
/// and [`TuringMachine::execute_until`] methods.
///
/// Most important trait.
///
/// # Examples
///
/// ```rust
/// extern crate turing_machine_rs;
///
/// use turing_machine_rs::instruction::Direction;
/// use turing_machine_rs::machines::Classic;
/// use turing_machine_rs::program::{ExtendBy, Program};
/// use turing_machine_rs::state::Tape;
/// use turing_machine_rs::TuringMachine;
///
/// fn main() {
///     let mut program = Program::new(vec!['t', 'e', 's', 'n', 'i', 'c', 'e', '_'], 4);
///     // Trait for more comfortable coding
///     program.extend_by([
///         // Instruction consists of Head and Tail parts
///         // Head state, Head symbol, Tail state, Tail symbol, Tail Direction
///         (1, 't', 2, 'n', Direction::Right),
///         (2, 'e', 3, 'i', Direction::Right),
///         (3, 's', 4, 'c', Direction::Right),
///         (4, 't', 0, 'e', Direction::Center),
///         // Revers
///         (1, 'n', 2, 't', Direction::Right),
///         (2, 'i', 3, 'e', Direction::Right),
///         (3, 'c', 4, 's', Direction::Right),
///         (4, 'e', 0, 't', Direction::Center),
///     ]);
///     let machine = Classic::new(program, '_');///
///
///     let test = Tape::from("test");
///     let nice = machine.translate_nrm(test.clone());
///     println!(
///         "{} {}!",
///         String::from_iter(nice.as_vec()),
///         String::from_iter(test.as_vec())
///     );
/// }
/// ```

pub trait TuringMachine<S: Symbol> {
    /// Executes program and returns a mutated [`Configuration`] using
    /// [`TuringMachine::execute_until`] method with `conf.state == 0`
    /// predicate. This is the most common use method for program execution.
    fn execute(&self, conf: Configuration<S>) -> Configuration<S> {
        self.execute_until(conf, |conf| conf.state == 0)
    }

    /// Turing machine msut have ability to execute program and change
    /// [`Configuration`] once. This is important for machine and it
    /// realization can variates depends to machine type.
    fn execute_once(&self, conf: Configuration<S>) -> Configuration<S>;

    /// Executes program untill stop predicate equals to `false` and returns
    /// a mutated [`Configuration`].
    ///
    /// # Examples
    /// ```rust
    /// use turing_machine_rs::instruction::Direction;
    /// use turing_machine_rs::machines::Classic;
    /// use turing_machine_rs::program::{ExtendBy, Program};
    /// use turing_machine_rs::state::{Configuration, Tape};
    /// use turing_machine_rs::TuringMachine;
    ///
    /// fn main() {
    ///     let mut program = Program::new(vec!['0', '1'], 3);
    ///     program.extend_by([
    ///         (1, '0', 2, '0', Direction::Right),
    ///         (1, '1', 1, '1', Direction::Left),
    ///         (2, '0', 3, '1', Direction::Left),
    ///         (2, '1', 2, '1', Direction::Right),
    ///         (3, '0', 0, '0', Direction::Center),
    ///         (3, '1', 3, '0', Direction::Left),
    ///     ]);
    ///     let machine = Classic::new(program, '0');
    ///
    ///     let conf = Configuration::new_std(Tape::from("010"));
    ///     let result = machine.execute_until(conf, |conf| conf.state == 3);
    ///
    ///     let expected = Configuration::new(Tape::from("0101"), 2, 3);
    ///     assert_eq!(expected, result);
    /// }
    /// ```
    fn execute_until(
        &self,
        conf: Configuration<S>,
        until: impl Fn(&Configuration<S>) -> bool,
    ) -> Configuration<S>;

    /// Translates and returns a mutated [`Tape`] using [`TuringMachine::execute`]
    /// method in standart begining [`Configuration::new_nrm`].
    fn translate_std(&self, tape: Tape<S>) -> Tape<S> {
        self.execute(Configuration::new_std(tape)).into_tape()
    }

    /// Translates and returns a mutated [`Tape`] using [`TuringMachine::execute`]
    /// method in normal begining [`Configuration::new_nrm`].
    fn translate_nrm(&self, tape: Tape<S>) -> Tape<S> {
        self.execute(Configuration::new_nrm(tape)).into_tape()
    }
}
