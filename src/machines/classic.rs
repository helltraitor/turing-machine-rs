use std::fmt::{Display, Error, Formatter};

use crate::instruction::Head;
use crate::program::Program;
use crate::state::Configuration;
use crate::{Symbol, TuringMachine, With};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Classic<S: Symbol> {
    default: S,
    program: Program<S>,
}

impl<S: Symbol> Classic<S> {
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
