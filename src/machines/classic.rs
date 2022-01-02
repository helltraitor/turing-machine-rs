use std::fmt::{Display, Error, Formatter};

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
    /// # Panics
    /// Panics when default symbol is not in alphabet.
    ///
    /// # Example
    /// ```rust
    /// use turing_machine_rs::TuringMachine;
    /// use turing_machine_rs::machines::Classic;
    /// use turing_machine_rs::program::Program;
    ///
    /// fn main() {
    ///     let program = Program::new(vec![' '], 1);
    ///     // This is fine
    ///     let _ = Classic::new(program, ' ');
    ///     // This will panic!
    ///     // let _ = Classic::new(program, '!');
    /// }
    /// ```
    pub fn new(program: Program<S>, default: S) -> Self {
        assert!(
            program.alphabet().contains(&default),
            "new error: default symbol {} not in alphabet {:?}",
            default,
            program.alphabet()
        );
        Classic { program, default }
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
        let inst = self.program.get(&head).unwrap_or_else(|| {
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
            let inst = self.program.get(&head).unwrap_or_else(|| {
                panic!(
                    "uncovered case: have no tail for head ({}) in program",
                    head
                )
            });
            conf.state = inst.tail.state;
            conf.set_symbol(inst.tail.symbol.clone());
            conf.shift(inst.tail.direction, self.default.clone());
        }
        conf
    }
}

impl<S: Symbol> With<Classic<S>> for Classic<S> {
    type Output = Option<Classic<S>>;

    /// Makes superposition with two or more [`Classic`] machines by chain.
    /// This method accept only [`Classic`] struct and can be used only for
    /// another [`Classic`] machine.
    ///
    /// # Example
    /// ```rust, ignore
    /// use turing_machine_rs::{TuringMachine, With};
    /// use turing_machine_rs::machines::Classic;
    ///
    /// // Machines implementations are hidden (takes to much space)
    /// // These machines implementations could be seing in `machines_classic.rs`
    ///
    /// fn main() {
    ///     let zerofy = new_zerofy_machine();
    ///     let left_shift = new_left_shift_machine();
    ///     let right_shift = new_right_shift_machine();
    ///     let trans = new_trans_machine();
    ///
    ///     let choose_machine = right_shift
    ///         .with(&trans)
    ///         .with(&right_shift)
    ///         .with(&zerofy)
    ///         .with(&left_shift)
    ///         .with(&zerofy)
    ///         .with(&left_shift);
    /// }
    /// ```
    fn with(&self, other: &Classic<S>) -> Self::Output {
        if self.program.alphabet() != other.program.alphabet() {
            return None;
        }
        if self.default != other.default {
            return None;
        }
        let mut program = self.program.clone();
        program.extend(&other.program);

        Some(Classic::new(program, self.default.clone()))
    }
}

impl<S: Symbol> With<Classic<S>> for Option<Classic<S>> {
    type Output = Option<Classic<S>>;

    /// Makes superposition with two or more [`Classic`] machines by chain.
    /// This method accept only [`Classic`] struct and can be used only for
    /// [`Option<Classic>`] machine.
    fn with(&self, other: &Classic<S>) -> Self::Output {
        match self {
            Some(machine) => machine.with(other),
            None => None,
        }
    }
}

impl<S: Symbol> Display for Classic<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        use std::any::type_name;

        write!(
            f,
            "Classic<{}> {{ program: {} }}",
            type_name::<S>(),
            self.program
        )
    }
}
