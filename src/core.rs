use std::fmt::{Debug, Display};

use crate::state::{Configuration, Tape};

pub trait With<T> {
    type Output;

    fn with(&self, other: &T) -> Self::Output;
}

pub trait Symbol: Clone + Debug + Display + Eq + PartialEq {}

impl<T> Symbol for T where T: Clone + Debug + Display + Eq + PartialEq {}

pub trait TuringMachine<S: Symbol> {
    fn execute(&self, conf: Configuration<S>) -> Configuration<S> {
        self.execute_until(conf, |conf| conf.state == 0)
    }

    fn execute_once(&self, conf: Configuration<S>) -> Configuration<S>;

    fn execute_until(
        &self,
        conf: Configuration<S>,
        until: impl Fn(&Configuration<S>) -> bool,
    ) -> Configuration<S>;

    fn translate_std(&self, tape: Tape<S>) -> Tape<S> {
        self.execute(Configuration::new_std(tape)).into_tape()
    }

    fn translate_nrm(&self, tape: Tape<S>) -> Tape<S> {
        self.execute(Configuration::new_nrm(tape)).into_tape()
    }
}
