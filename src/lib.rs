#![warn(missing_docs)]

//! # Turing Machine RS
//! A library for implementing any Turing machine with minimal limitations
//! for the Rust programming language. It is:
//! * **Low-cost**: Turing Machine RS designed to simulate execution.
//!     That's why it cannot be simple, flexible and zero-cost at the same time.
//! * **Flexible**: Turing Machine RS works with not the specific types nor even
//!     copy-only types! Instead, the library supports any struct or object that
//!     implements `Clone + Debug + Display + Eq + PartialEq` trait.
//!
//! For futher details use `cargo doc --open` (or online docs) or proceed
//! to the repository on [Github](https://github.com/Helltraitor/turing-machine-rs).

mod core;
pub mod instruction;
pub mod machines;
pub mod program;
pub mod state;
mod turing;

pub use crate::core::Symbol;
pub use crate::core::With;
pub use crate::turing::TuringMachine;
