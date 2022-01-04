extern crate turing_machine_rs;

use turing_machine_rs::instruction::Direction;
use turing_machine_rs::machines::Classic;
use turing_machine_rs::program::{Extend, Program};
use turing_machine_rs::state::Tape;
use turing_machine_rs::TuringMachine;

fn main() {
    let mut program = Program::new(vec!['t', 'e', 's', 'n', 'i', 'c', 'e', '_'], 4);
    program.extend([
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
    let machine = Classic::new(program, '_').unwrap();

    let test = Tape::from("test");
    let nice = machine.translate_nrm(test.clone()).unwrap();
    println!(
        "{} {}!",
        String::from_iter(nice.as_vec()),
        String::from_iter(test.as_vec())
    );
}
