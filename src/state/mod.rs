//! Provides [`Configuration`] and it's component [`Tape`] realizations
//! which represents Turing machine state.
//!
//! This module provides [`Configuration`] and [`Tape`] which are used
//! by Turing machines and can be used with any type that implements
//! the [`crate::Symbol`] trait.
//!
//! [`Configuration`] is used by the Turing machine directly. And [`Tape`] is
//! the component of [`Configuration`] with the ability to use it
//! in [`Configuration`]s default variants (normal or standard)
//! for use in [`crate::TuringMachine::translate_nrm`]
//! and [`crate::TuringMachine::translate_std`] methods.
//!
//! # Warning
//! [`Configuration`] could panic only if source code is broken - this
//! would be a bug.
//!
//! So you could open an issue on [GitHub](https://github.com/Helltraitor/turing-machine-rs).
//!
//! [`Tape`] may panic in situations similar to those encountered by [`Vec`]
//! (because the [`Tape`] type is based on [`Vec`]). It should be used only
//! for creation and translation by a Turing machine.
//!
//! Examples
//! ```rust
//! extern crate turing_machine_rs;
//!
//! use turing_machine_rs::instruction::{Move, State};
//! use turing_machine_rs::machines::Classic;
//! use turing_machine_rs::program::{Extend, Program};
//! use turing_machine_rs::state::Tape;
//! use turing_machine_rs::TuringMachine;
//!
//! // For more comfortable coding, use Result<(), String>:
//! // `?` postfix symbol is better then `.unwrap()` postfix method call.
//! fn main() -> Result<(), String> {
//!     let alphabet = vec!['t', 'e', 's', 'n', 'i', 'c', 'e', '_'];
//!     let mut program = Program::new(alphabet, State(4));
//!     program.extend([
//!         (1, 't', 2, 'n', Move::Right),
//!         (2, 'e', 3, 'i', Move::Right),
//!         (3, 's', 4, 'c', Move::Right),
//!         (4, 't', 0, 'e', Move::None),
//!         // Revers
//!         (1, 'n', 2, 't', Move::Right),
//!         (2, 'i', 3, 'e', Move::Right),
//!         (3, 'c', 4, 's', Move::Right),
//!         (4, 'e', 0, 't', Move::None),
//!     ])?;
//!     let machine = Classic::new(program, '_')?;
//!
//!     // Use of `Tape` without `Configuration`
//!     let test = Tape::from("test");
//!     let nice = machine.translate_nrm(test.clone())?;
//!     println!(
//!         "{} {}!",
//!         String::from_iter(nice.as_vec()),
//!         String::from_iter(test.as_vec())
//!     );
//!
//!     Ok(())
//! }
//! ```
mod configuration;
mod tape;

pub use configuration::Configuration;
pub use tape::Tape;
