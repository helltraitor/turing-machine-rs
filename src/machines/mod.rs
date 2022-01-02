//! Provides [`Classic`] [`crate::TuringMachine`] realization and [`Debugger`]
//! [`crate::TuringMachine`] for debugging [`crate::TuringMachine`] implementations.
//!
//! # Warning
//! [`Classic`] and [`Debugger`] can panic!
//!
//! [`Debugger`] could panic only if source code is broken - this is a bug.
//! So you can open issue on [GitHub](https://github.com/Helltraitor/turing-machine-rs).
//!
//! # Examples
//! ## Execute from [`Classic`] tests
//! ```rust
//! use turing_machine_rs::instruction::Direction;
//! use turing_machine_rs::machines::Classic;
//! use turing_machine_rs::program::{ExtendBy, Program};
//! use turing_machine_rs::state::{Configuration, Tape};
//! use turing_machine_rs::TuringMachine;
//!
//! fn main() {
//!     let mut program = Program::new(vec![' ', '0', '1'], 2);
//!     program.extend_by([
//!         (1, ' ', 2, ' ', Direction::Right),
//!         (1, '0', 1, '1', Direction::Left),
//!         (1, '1', 1, '0', Direction::Left),
//!         (2, ' ', 0, ' ', Direction::Left),
//!         (2, '0', 2, '0', Direction::Right),
//!         (2, '1', 2, '1', Direction::Right),
//!     ]);
//!     let machine = Classic::new(program, ' ');
//!
//!     let conf = Configuration::new(Tape::from("001100"), 5, 0);
//!     let result = machine.execute(conf.clone());
//!
//!     let expected = conf;
//!
//!     assert_eq!(expected, result);
//!
//!     let conf = Configuration::new_std(Tape::from("001100"));
//!     let result = machine.execute(conf);
//!
//!     let expected = Configuration::new(Tape::from(" 110011 "), 6, 0);
//!
//!     assert_eq!(expected, result);
//! }
//! ```

mod classic;
mod debugger;

pub use classic::Classic;
pub use debugger::Debugger;
