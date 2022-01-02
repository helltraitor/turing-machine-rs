#![warn(missing_docs)]

//! # Turing Machine RS
//! A library for implementing any turing machines with minimal limitations
//! for Rust programming language. It is:
//! * **Low-cost**: Turing Machine RS designed to simulate execution that's
//!     why it cannot be simple, flexible and zero-cost in the same time.
//! * **Flexible**: Turing Machine RS works with not the specific types nor even copy-only types! Instead of the library supports any struct or object with implements `Clone + Debug + Display + Eq + PartialEq` trait.
//!
//! Use `cargo doc --open` or proceed to repository on
//! [Github](https://github.com/Helltraitor/turing-machine-rs).

pub mod core;
pub mod instruction;
pub mod machines;
pub mod program;
pub mod state;

pub use crate::core::Symbol;
pub use crate::core::TuringMachine;
pub use crate::core::With;
