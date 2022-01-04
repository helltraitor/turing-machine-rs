use std::fmt;

use crate::instruction::Head;
use crate::program::Program;
use crate::state::Configuration;
use crate::{Symbol, TuringMachine, With};

/// [`Classic`] is a common [`TuringMachine`] realization that can be used
/// freely for program execution.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Classic<S: Symbol> {
    default: S,
    program: Program<S>,
}

impl<S: Symbol> Classic<S> {
    /// Constructs a new [`Classic`] Turing machine from the program
    /// [`Program`] and the default symbol [`Symbol`].
    ///
    /// Returns [`Ok(Classic)`] when the default symbol is in the program
    /// alphabet otherwise [`Err(String)`] with diagnostic information.
    ///
    /// # Examples
    /// Trying to return the new machine with a mismatched default symbol:
    /// ```rust
    /// use turing_machine_rs::instruction::State;
    /// use turing_machine_rs::machines::Classic;
    /// use turing_machine_rs::program::Program;
    ///
    /// let program = Program::new(vec!['0', '1'], State(1));
    /// let machine = Classic::new(program, '!');
    ///
    /// assert!(machine.is_err());
    /// ```
    ///
    /// Successful creation:
    /// ```rust
    /// use turing_machine_rs::instruction::State;
    /// use turing_machine_rs::machines::Classic;
    /// use turing_machine_rs::program::Program;
    ///
    /// let program = Program::new(vec!['0', '1'], State(1));
    /// let machine = Classic::new(program, '0');
    ///
    /// assert!(machine.is_ok());
    /// ```
    pub fn new(program: Program<S>, default: S) -> Result<Self, String> {
        match program.alphabet().contains(&default) {
            true => Ok(Classic { program, default }),
            false => Err(format!(
                "new error: default symbol {} is not in alphabet {:?}",
                default,
                program.alphabet()
            )),
        }
    }
}

impl<S: Symbol> TuringMachine<S> for Classic<S> {
    /// Executes [`Configuration`] once by mutation.
    ///
    /// Returns [`Ok(Configuration)`] when an [`crate::instruction::Instruction`]
    /// exists for the current [`Configuration`] symbol and state.
    /// And otherwise returns [`Err(String)`] with diagnostic information.
    fn execute_once(&self, mut conf: Configuration<S>) -> Result<Configuration<S>, String> {
        let head = Head::new(conf.state, conf.get_symbol().clone());
        let inst = match self.program.get(&head)? {
            Some(inst) => inst,
            None => {
                return Err(format!(
                    "uncovered case: have no tail for head ({}) in program",
                    head
                ))
            }
        };
        conf.state = inst.tail.state;
        conf.set_symbol(inst.tail.symbol.clone());
        conf.shift(inst.tail.movement, self.default.clone());
        Ok(conf)
    }

    /// Executes [`Configuration`] until predicate is `false` by mutation.
    ///
    /// Returns [`Ok(Configuration)`] when an [`crate::instruction::Instruction`]
    /// exists for the current [`Configuration`] symbol and state.
    /// And otherwise returns [`Err(String)`] with diagnostic information.
    fn execute_until(
        &self,
        mut conf: Configuration<S>,
        until: impl Fn(&Configuration<S>) -> bool,
    ) -> Result<Configuration<S>, String> {
        while !until(&conf) {
            let head = Head::new(conf.state, conf.get_symbol().clone());
            let inst = match self.program.get(&head)? {
                Some(inst) => inst,
                None => {
                    return Err(format!(
                        "uncovered case: have no tail for head ({}) in program",
                        head
                    ))
                }
            };
            conf.state = inst.tail.state;
            conf.set_symbol(inst.tail.symbol.clone());
            conf.shift(inst.tail.movement, self.default.clone());
        }
        Ok(conf)
    }
}

impl<S: Symbol> With<Classic<S>> for Classic<S> {
    type Output = Result<Classic<S>, String>;

    /// Makes superposition with two or more [`Classic`] machines by chain.
    /// This method accept only [`Classic`] struct and can be used only for
    /// another [`Classic`] machine.
    ///
    /// Returns a new [`Ok(Classic)`] when machines can be concatenated
    /// and [`Err(String)`] with diagnostic information when machines
    /// have different alphabets or default symbols.
    fn with(&self, other: &Classic<S>) -> Self::Output {
        if self.default != other.default {
            return Err(format!(
                "with error: classic machines have different default symbols: {} and {}",
                self.default, other.default,
            ));
        }
        // `Program::with` implementation guarantees that program can
        // be concatenated only with the same alphabet
        let program = self.program.with(&other.program)?;
        Classic::new(program, self.default.clone())
    }
}

impl<S: Symbol> With<Classic<S>> for Result<Classic<S>, String> {
    type Output = Result<Classic<S>, String>;

    /// Makes superposition with two or more [`Classic`] machines by chain.
    /// This method accept only [`Classic`] struct and can be used only for
    /// [`Result<Classic, String>`].
    ///
    /// Returns a new [`Ok(Classic)`] when `self` is [`Result::Ok`] and machines
    /// can be concatenated and [`Err(String)`] when `self` is [`Result::Ok`]
    /// but machines have different alphabets or default symbols.
    ///
    /// And Returns a copy of [`Err(String)`] when `self` is [`Result::Err`].
    fn with(&self, other: &Classic<S>) -> Self::Output {
        match self {
            Ok(machine) => machine.with(other),
            Err(msg) => Err(msg.clone()),
        }
    }
}

impl<S: Symbol> fmt::Display for Classic<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use std::any::type_name;

        write!(
            f,
            "Classic<{}> {{ program: {} }}",
            type_name::<S>(),
            self.program
        )
    }
}
