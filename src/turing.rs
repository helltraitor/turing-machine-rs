use crate::state::{Configuration, Tape};
use crate::Symbol;

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
///     let machine = Classic::new(program, '_').unwrap();
///
///     let test = Tape::from("test");
///     let nice = machine.translate_nrm(test.clone()).unwrap();
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
    fn execute(&self, conf: Configuration<S>) -> Result<Configuration<S>, String> {
        self.execute_until(conf, |conf| conf.state == 0)
    }

    /// Turing machine msut have ability to execute program and change
    /// [`Configuration`] once. This is important for machine and it
    /// realization can variates depends to machine type.
    fn execute_once(&self, conf: Configuration<S>) -> Result<Configuration<S>, String>;

    #[allow(clippy::needless_doctest_main)]
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
    ///     let machine = Classic::new(program, '0').unwrap();
    ///
    ///     let conf = Configuration::new_std(Tape::from("010")).unwrap();
    ///     let result = machine.execute_until(conf, |conf| conf.state == 3).unwrap();
    ///
    ///     let expected = Configuration::new(Tape::from("0101"), 2, 3).unwrap();
    ///     assert_eq!(expected, result);
    /// }
    /// ```
    fn execute_until(
        &self,
        conf: Configuration<S>,
        until: impl Fn(&Configuration<S>) -> bool,
    ) -> Result<Configuration<S>, String>;

    /// Translates and returns a mutated [`Tape`] using [`TuringMachine::execute`]
    /// method in standart begining [`Configuration::new_nrm`].
    fn translate_std(&self, tape: Tape<S>) -> Result<Tape<S>, String> {
        let conf = Configuration::new_std(tape)?;
        let exec = self.execute(conf)?;
        Ok(exec.into_tape())
    }

    /// Translates and returns a mutated [`Tape`] using [`TuringMachine::execute`]
    /// method in normal begining [`Configuration::new_nrm`].
    fn translate_nrm(&self, tape: Tape<S>) -> Result<Tape<S>, String> {
        let conf = Configuration::new_nrm(tape)?;
        let exec = self.execute(conf)?;
        Ok(exec.into_tape())
    }
}
