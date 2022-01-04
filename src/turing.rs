use crate::instruction::State;
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
/// ```rust
/// extern crate turing_machine_rs;
///
/// use turing_machine_rs::instruction::{Move, State};
/// use turing_machine_rs::machines::Classic;
/// use turing_machine_rs::program::{Extend, Program};
/// use turing_machine_rs::state::Tape;
/// use turing_machine_rs::TuringMachine;
///
/// fn main() -> Result<(), String> {
///    let alphabet = vec!['t', 'e', 's', 'n', 'i', 'c', 'e', '_'];
///    let mut program = Program::new(alphabet, State(4));
///     // Trait for more comfortable coding
///     program.extend([
///         // Instruction consists of Head and Tail parts
///         // Head state, Head symbol, Tail state, Tail symbol, Tail Move
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
pub trait TuringMachine<S: Symbol> {
    /// Executes the [`crate::program::Program`] and returns a mutated [`Configuration`]
    /// using the [`TuringMachine::execute_until`] method with the `conf.state == 0`
    /// predicate. This is the most commonly used method for [`Program`] execution.
    fn execute(&self, conf: Configuration<S>) -> Result<Configuration<S>, String> {
        self.execute_until(conf, |conf| conf.state == State(0))
    }

    /// A Turing machine must have the ability to execute [`crate::program::Program`]
    /// and change the [`Configuration`] once. This is important for machines,
    /// and its realization can vary depending on machine type.
    fn execute_once(&self, conf: Configuration<S>) -> Result<Configuration<S>, String>;

    /// Executes program untill stop predicate equals to `false` and returns
    /// a mutated [`Configuration`].
    ///
    /// # Examples
    /// ```rust
    /// use turing_machine_rs::instruction::{Move, State};
    /// use turing_machine_rs::machines::Classic;
    /// use turing_machine_rs::program::{Extend, Program};
    /// use turing_machine_rs::state::{Configuration, Tape};
    /// use turing_machine_rs::TuringMachine;
    ///
    /// fn main() -> Result<(), String> {
    ///     let mut program = Program::new(vec!['0', '1'], State(3));
    ///     program.extend([
    ///         (1, '0', 2, '0', Move::Right),
    ///         (1, '1', 1, '1', Move::Left),
    ///         (2, '0', 3, '1', Move::Left),
    ///         (2, '1', 2, '1', Move::Right),
    ///         (3, '0', 0, '0', Move::None),
    ///         (3, '1', 3, '0', Move::Left),
    ///     ])?;
    ///     let machine = Classic::new(program, '0')?;
    ///
    ///     let conf = Configuration::new_std(Tape::from("010"))?;
    ///     let result = machine.execute_until(conf, |conf| conf.state == State(3))?;
    ///
    ///     let expected = Configuration::new(Tape::from("0101"), 2, State(3))?;
    ///     assert_eq!(expected, result);
    ///
    ///     Ok(())
    /// }
    /// ```
    fn execute_until(
        &self,
        conf: Configuration<S>,
        until: impl Fn(&Configuration<S>) -> bool,
    ) -> Result<Configuration<S>, String>;

    /// Translates and returns a mutated [`Tape`] using the [`TuringMachine::execute`]
    /// method as the [`Configuration::new_std`].
    fn translate_std(&self, tape: Tape<S>) -> Result<Tape<S>, String> {
        let conf = Configuration::new_std(tape)?;
        let exec = self.execute(conf)?;
        Ok(exec.into_tape())
    }

    /// Translates and returns a mutated [`Tape`] using the [`TuringMachine::execute`]
    /// method as the [`Configuration::new_nrm`].
    fn translate_nrm(&self, tape: Tape<S>) -> Result<Tape<S>, String> {
        let conf = Configuration::new_nrm(tape)?;
        let exec = self.execute(conf)?;
        Ok(exec.into_tape())
    }
}
