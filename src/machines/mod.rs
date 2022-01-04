//! Provides two Turing machines: [`Classic`] and [`Debugger`].
//! - [`Classic`] is a Turing machine general realization. If you need to only
//!     execute a program for a configuration, then you could use this machine.
//! - [`Debugger`] is another Turing machine that is created by using an existing
//!     machine. Provides an ability to set [`crate::state::Configuration`] and
//!     ([`crate::instruction::Head`], [`crate::instruction::Tail`]) handlers
//!     (e.g. you can print output in the string buffer).
//!
//! [`crate::TuringMachine`] for debugging [`crate::TuringMachine`] implementations.
//!
//! # Warning
//! [`Debugger`] could panic only if source code is broken - this would be a bug.
//!
//! So you could open an issue on [GitHub](https://github.com/Helltraitor/turing-machine-rs).

mod classic;
mod debugger;

pub use classic::Classic;
pub use debugger::Debugger;
