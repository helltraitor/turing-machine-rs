//! Provides [`Classic`] [`crate::TuringMachine`] realization and [`Debugger`]
//! [`crate::TuringMachine`] for debugging [`crate::TuringMachine`] implementations.
//!
//! # Warning
//! [`Debugger`] can panic!
//!
//! [`Debugger`] could panic only if source code is broken - this is a bug.
//! So you can open issue on [GitHub](https://github.com/Helltraitor/turing-machine-rs).

mod classic;
mod debugger;

pub use classic::Classic;
pub use debugger::Debugger;
