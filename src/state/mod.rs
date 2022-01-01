//! State implementation for Turing Machine RS
//!
//! This module provides [`Tape`] and [`Configuration`] which are using
//! by Turing machines and can be used with any types which implements
//! [`crate::Symbol`] trait.
//!
//! Main component is [`Configuration`] which is used by Turing machine
//! directly.
//!
//! [`Configuration`] can be used without type annotation but it depends
//! to [`Tape`] which needs in type annotation when it's used without
//! Turing machine or other code which could help compiler to determine
//! type.
//!
//! [`Tape`] supports `str-as-copy` and `string-as-copy`. When these features
//! are enabled, type annotations are exceed and any [`Tape::from`]`(&str or String)`
//! will be interpreted as [`Tape<char>`].
//!
//! # Warning
//! [`Configuration`] and [`Tape`] can panic!
//!
//! # Examples
//! No features, no machines.
//! ```rust
//! use turing_machine_rs::state::{Configuration, Tape};
//!
//! fn main() {
//!     let _: Configuration<char> = Configuration::new_std(Tape::from("test"));
//!     // BOTH ALLOWED
//!     let _: Configuration<Box<char>> = Configuration::new_std(Tape::from("test"));
//! }
//! ```
//!
//! `str-as-copy` feature, no machines.
//!
//! Note: this code will not running when doc tests because it's impossible to
//! activate feature for code shipped.
//! ```rust, ignore
//! use turing_machine_rs::state::{Configuration, Tape};
//!
//! fn main() {
//!     let _ = Configuration::new_std(Tape::from("test"));
//!     // Caused mistype error
//!     // let _: Configuration<Box<char>> = Configuration::new_std(Tape::from("test"));
//! }
//! ```

mod configuration;
mod tape;

pub use configuration::Configuration;
pub use tape::Tape;
