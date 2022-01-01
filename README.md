# Turing Machine RS

A library for implementing any turing machines with minimal limitations for Rust programming language. It is:

* **Low-cost**: Turing Machine RS designed to simulate execution that's why it cannot be simple, flexible and zero-cost in the same time.

* **Flexible**: Turing Machine RS works with not the specific types nor even copy-only types! Instead of the library supports any struct or object with implements `Clone + Debug + Display + Eq + PartialEq` trait.

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/FILL_SHIELDS_BADGE
[crates-url]: https://crates.io/crates/FILL_CRATES_URL
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/FILL_LICENSE_PATH
[actions-badge]: https://github.com/FILL_ATCTION_BADGE
[actions-url]: https://github.com/FILL_ACTION_URL

[Website](FILL_ME) |
[Guides](FILL_GITHUB_IO_TUTORIAL) |
[API Docs](https://docs.rs/FILL_ME)

## Overview

Turing Machine RS provides realization for classic turing machine named "Classic" (minimal version for simulation) and debugger for turing machines "Debugger" which works with any machines implemented Turing Machine Trait.

## Example

This is a simple example for a Turing Machine which replace nice by test and test by nice chars.

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

But this library is not for only simplest types: you can even use other turing machines as symbols! More examples can be found [here][examples].

To see a list of the available features flags that can be enabled, check [docs][feature-flag-docs].

## Getting Help

First, read [tutorial] or [examples]. If tutorial can't provides answers for you then you can try to read docs and after all of it you can contact with me: <helltraitor@hotmail.com>

### Major links:
* [Tutorial][tutorial]
* [Examples][examples]
* [API documentation][api-documentation]
* [Feature flag doc][feature-flag-docs]

[tutorial]: FILL_ME
[examples]: https://github.com/Helltraitor/turing-machine-rs/tree/main/examples
[api-documentation]: https://docs.rs/FILL_ME
[feature-flag-docs]: https://docs.rs/FILL_ME

## Contributing

If you want to improve this crate, just open an issue and describe the problem (shortly if you can), further possible problems or reasons to improve (widely if needs), how to solve this problems and docs for this or similar cases (optional).
[See example.](FILL_ME)

## Supported Rust Versions

Turing Machine RS was created in lastest stable version (2021, 1.57) but this library also support FILL_ME and above.

## License

This project is licensed under the [MIT license].

[MIT license]: https://github.com/FILL_ME

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Turing Machine RS by you, shall be licensed as MIT, without any additional
terms or conditions.