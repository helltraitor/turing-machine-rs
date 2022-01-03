use std::fmt;

use crate::instruction::Head;
use crate::program::Program;
use crate::state::Configuration;
use crate::{Symbol, TuringMachine, With};

/// [`Classic`] is an example of [`TuringMachine`] which can be freely used
/// for program execution.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Classic<S: Symbol> {
    default: S,
    program: Program<S>,
}

impl<S: Symbol> Classic<S> {
    /// Constructs a new [`Classic<Symbol>`] Turing machine from program
    /// [`Program`] and default symbol [`Symbol`].
    ///
    /// Returns [`Ok(Classic<Symbol>)`] when default symbol is in program
    /// alphabet otherwise [`Err(String)`] with diagnostic information.
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
    /// # Panics
    /// Panics when program doesn't contains [`crate::instruction::Instruction`]
    /// with [`Head`] for this index and symbol.
    fn execute_once(&self, mut conf: Configuration<S>) -> Configuration<S> {
        let head = Head::new(conf.state, conf.get_symbol().clone());
        let inst = self.program.get(&head).unwrap().unwrap_or_else(|| {
            panic!(
                "uncovered case: have no tail for head ({}) in program",
                head
            )
        });
        conf.state = inst.tail.state;
        conf.set_symbol(inst.tail.symbol.clone());
        conf.shift(inst.tail.direction, self.default.clone());
        conf
    }
    /// Executes [`Configuration`] until predicate is `false` by mutation.
    ///
    /// # Panics
    /// Panics when program doesn't contains [`crate::instruction::Instruction`]
    /// with [`Head`] for this index and symbol.
    fn execute_until(
        &self,
        mut conf: Configuration<S>,
        until: impl Fn(&Configuration<S>) -> bool,
    ) -> Configuration<S> {
        while !until(&conf) {
            let head = Head::new(conf.state, conf.get_symbol().clone());
            let inst = self.program.get(&head).unwrap().expect(
                format!(
                    "uncovered case: have no tail for head ({}) in program",
                    head
                )
                .as_str(),
            );
            conf.state = inst.tail.state;
            conf.set_symbol(inst.tail.symbol.clone());
            conf.shift(inst.tail.direction, self.default.clone());
        }
        conf
    }
}

impl<S: Symbol> With<Classic<S>> for Classic<S> {
    type Output = Result<Classic<S>, String>;

    /// Makes superposition with two or more [`Classic`] machines by chain.
    /// This method accept only [`Classic`] struct and can be used only for
    /// another [`Classic`] machine.
    ///
    /// Returns a new [`Ok(Classic)`] on success and [`Err(String)`]
    /// with diagnostic information when machines have different alphabets
    /// or default symbols.
    fn with(&self, other: &Classic<S>) -> Self::Output {
        if self.program.alphabet() != other.program.alphabet() {
            return Err(format!(
                "with error: classic machines have different alphabets: {:?} and {:?}",
                self.program.alphabet(),
                other.program.alphabet()
            ));
        }
        if self.default != other.default {
            return Err(format!(
                "with error: classic machines have different default symbols: {} and {}",
                self.default, other.default,
            ));
        }
        match self.program.with(&other.program) {
            Ok(program) => Classic::new(program, self.default.clone()),
            Err(msg) => Err(msg),
        }
    }
}

impl<S: Symbol> With<Classic<S>> for Result<Classic<S>, String> {
    type Output = Result<Classic<S>, String>;

    /// Makes superposition with two or more [`Classic`] machines by chain.
    /// This method accept only [`Classic`] struct and can be used only for
    /// [`Result<Classic, &'static str>`] machine.
    ///
    /// Returns a new [`Ok(Classic)`] when `self` is [`Result::Ok`] on success
    /// and [`Err(String)`] when `self` is [`Result::Ok`] but machines have
    /// different alphabets or default symbols. Returns a copy of [`Err(String)`]
    /// when `self` is [`Result::Err`]
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
