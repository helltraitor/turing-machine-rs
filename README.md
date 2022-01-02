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
[actions-nightly-badge]: https://github.com/Helltraitor/turing-machine-rs/actions/workflows/rust_stable.yml/badge.svg
[actions-nightly-url]: https://github.com/Helltraitor/turing-machine-rs/actions/workflows/rust_stable.yml

[API Docs](https://docs.rs/turing-machine-rs)

## Overview

Turing Machine RS includes a "Classic" realization for Turing Machine (a minimal version for simulation) and a "Debugger" Turing machine that works with any type that implements the Turing Machine Trait.

## Example

This is a simple example of a Turing Machine that replaces nice by test and test by nice chars.

```rust
extern crate turing_machine_rs;

use turing_machine_rs::instruction::Direction;
use turing_machine_rs::machines::Classic;
use turing_machine_rs::program::{ExtendBy, Program};
use turing_machine_rs::state::Tape;
use turing_machine_rs::TuringMachine;

fn main() {
    let mut program = Program::new(vec!['t', 'e', 's', 'n', 'i', 'c', 'e', '_'], 4);
    // Trait for more comfortable coding
    program.extend_by([
        // Instruction consists of Head and Tail parts
        // Head state, Head symbol, Tail state, Tail symbol, Tail Direction
        (1, 't', 2, 'n', Direction::Right),
        (2, 'e', 3, 'i', Direction::Right),
        (3, 's', 4, 'c', Direction::Right),
        (4, 't', 0, 'e', Direction::Center),
        // Revers
        (1, 'n', 2, 't', Direction::Right),
        (2, 'i', 3, 'e', Direction::Right),
        (3, 'c', 4, 's', Direction::Right),
        (4, 'e', 0, 't', Direction::Center),
    ]);
    let machine = Classic::new(program, '_');

    let test = Tape::from("test");
    let nice = machine.translate_nrm(test.clone());
    println!(
        "{} {}!",
        String::from_iter(nice.as_vec()),
        String::from_iter(test.as_vec())
    );
}
```

But this library is not just for the simplest types: you can even use other Turing machines as symbols! More examples can be found [here][examples].

To see a list of the available features flags that can be enabled, check [docs][feature-flag-docs].

## Getting Help

First, read [examples][examples] or [api][api-documentation]. If examples can't provide answers for you, then you can try to read docs, and after all of that, you can contact me: <helltraitor@hotmail.com>

### Major links:
* [Examples][examples]
* [API documentation][api-documentation]

[examples]: https://github.com/Helltraitor/turing-machine-rs/tree/main/examples
[api-documentation]: https://docs.rs/turing-machine-rs

## Contributing

If you want to improve this crate, just open an issue and describe the problem (shortly if you can), further possible problems or reasons to improve (widely if needed), how to solve this problem, and docs for this or similar cases (optional).
[See example.](FILL_ME)

## Supported Rust Versions

Turing Machine RS was created in the latest stable version (2021, 1.57) but this library also supports 1.56 (at least) and above.

## License

This project is licensed under the [MIT license].

[MIT license]: https://github.com/Helltraitor/turing-machine-rs/blob/main/LICENSE

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Turing Machine RS by you, shall be licensed as MIT, without any additional
terms or conditions.