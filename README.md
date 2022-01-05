# Turing Machine RS
A library for implementing any Turing machine with minimal limitations for the Rust programming language. It is:

* **Low-cost**: Turing Machine RS designed to simulate execution. That's why it cannot be simple, flexible and zero-cost at the same time.

* **Flexible**: Turing Machine RS works with not the specific types nor even copy-only types! Instead, the library supports any struct or object that implements `Clone + Debug + Display + Eq + PartialEq` trait.

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Rust 2021][actions-2021-badge]][actions-2021-url]
[![Rust Stable][actions-stable-badge]][actions-stable-url]
[![Rust Nightly][actions-nightly-badge]][actions-nightly-url]

[crates-badge]: https://img.shields.io/crates/v/turing-machine-rs
[crates-url]: https://crates.io/crates/turing-machine-rs
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/Helltraitor/turing-machine-rs/blob/main/LICENSE
[actions-2021-badge]: https://github.com/Helltraitor/turing-machine-rs/actions/workflows/rust_2021.yml/badge.svg
[actions-2021-url]: https://github.com/Helltraitor/turing-machine-rs/actions/workflows/rust_2021.yml
[actions-stable-badge]: https://github.com/Helltraitor/turing-machine-rs/actions/workflows/rust_stable.yml/badge.svg
[actions-stable-url]: https://github.com/Helltraitor/turing-machine-rs/actions/workflows/rust_stable.yml
[actions-nightly-badge]: https://github.com/Helltraitor/turing-machine-rs/actions/workflows/rust_nightly.yml/badge.svg
[actions-nightly-url]: https://github.com/Helltraitor/turing-machine-rs/actions/workflows/rust_nightly.yml

[Docs](https://docs.rs/turing-machine-rs)

## Overview
Turing Machine RS includes a "Classic" realization for Turing Machine (a minimal version for simulation) and a "Debugger" Turing Machine that works with any type that implements the Turing Machine Trait.

## Example
This is a simple example of a Turing Machine that replaces `nice` by `test` and `test` by `nice` words.

```rust
extern crate turing_machine_rs;

use turing_machine_rs::instruction::{Move, State};
use turing_machine_rs::machines::Classic;
use turing_machine_rs::program::{Extend, Program};
use turing_machine_rs::state::Tape;
use turing_machine_rs::TuringMachine;

// For more comfortable coding, use Result<(), String>:
// `?` postfix symbol is better then `.unwrap()` postfix method call.
fn main() -> Result<(), String> {
    let alphabet = vec!['t', 'e', 's', 'n', 'i', 'c', 'e', '_'];
    let mut program = Program::new(alphabet, State(4));
    program.extend([
        (1, 't', 2, 'n', Move::Right),
        (2, 'e', 3, 'i', Move::Right),
        (3, 's', 4, 'c', Move::Right),
        (4, 't', 0, 'e', Move::None),
        // Revers
        (1, 'n', 2, 't', Move::Right),
        (2, 'i', 3, 'e', Move::Right),
        (3, 'c', 4, 's', Move::Right),
        (4, 'e', 0, 't', Move::None),
    ])?;
    let machine = Classic::new(program, '_')?;

    let test = Tape::from("test");
    let nice = machine.translate_nrm(test.clone())?;
    println!(
        "{} {}!",
        String::from_iter(nice.as_vec()),
        String::from_iter(test.as_vec())
    );
    Ok(())
}
```

But this library is not just for the simplest types: you can even use other Turing machines as symbols! More examples can be found [here][examples].

## Getting Help
First, read [examples][examples] or [docs][docs]. If examples can't provide answers for you, then you can try to read docs, and after all of that, you can contact me: <helltraitor@hotmail.com>

### Major links:
* [Examples][examples]
* [Docs][docs]
* [Repository][repo]

[examples]: https://github.com/Helltraitor/turing-machine-rs/tree/main/examples
[docs]: https://docs.rs/turing-machine-rs
[repo]: https://github.com/Helltraitor/turing-machine-rs

## Contributing
If you want to improve this crate, just open an issue (you can use [example][issue-example] as a template). The issue must contain these headings: `Problem` or `Enhancement`, `Motivation` (reasons to solve the problem or implement the enhancement). It would be very useful to add `Useful sources` for documentation and examples.

[See example.][issue-example]

[issue-example]: https://github.com/Helltraitor/turing-machine-rs/issues/2

## Supported Rust Versions
Turing Machine RS was created in the latest stable version (2021, 1.57) but this library also supports 1.56 (at least) and above.

## License
This project is licensed under the [MIT license].

[MIT license]: https://github.com/Helltraitor/turing-machine-rs/blob/main/LICENSE

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in Turing Machine RS by you, shall be licensed as MIT, without any additional terms or conditions.
