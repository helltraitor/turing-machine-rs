extern crate turing_machine_rs;

use turing_machine_rs::instruction::Direction;
use turing_machine_rs::machines::Classic;
use turing_machine_rs::program::{ExtendBy, Program};
use turing_machine_rs::state::Tape;
use turing_machine_rs::TuringMachine;

fn main() {
    let mut program = Program::new(vec![' ', '0', '1', '+'], 8);
    program.extend_by([
        // Sub 1, also init zero check
        (1, ' ', 0, ' ', Direction::Center),
        (1, '0', 1, '0', Direction::Left),
        (1, '1', 2, '0', Direction::Right),
        (1, '+', 6, '+', Direction::Right),
        // Subs part
        (2, ' ', 3, ' ', Direction::Left),
        (2, '0', 2, '1', Direction::Right),
        // 2, '1' -> Impl
        // 2, '+' -> Err
        //
        // Find + on left
        // 3, ' ' -> Err
        (3, '0', 3, '0', Direction::Left),
        (3, '1', 3, '1', Direction::Left),
        (3, '+', 4, '+', Direction::Left),
        // Add 1
        (4, ' ', 5, '1', Direction::Right),
        (4, '0', 5, '1', Direction::Right),
        (4, '1', 4, '0', Direction::Left),
        // 4, '+' -> Err
        //
        // Find + on rigth
        // 5, ' ' -> Imp
        (5, '0', 5, '0', Direction::Right),
        (5, '1', 5, '1', Direction::Right),
        (5, '+', 6, '+', Direction::Right),
        // Zero check
        (6, ' ', 8, ' ', Direction::Left),
        (6, '0', 6, '0', Direction::Right),
        (6, '1', 7, '1', Direction::Right),
        // 6, '+' -> Err
        //
        // Find last num
        (7, ' ', 1, ' ', Direction::Left),
        (7, '0', 7, '0', Direction::Right),
        (7, '1', 7, '1', Direction::Right),
        // 7, '+' -> Err
        //
        // Clear + and after
        // 8, ' ' - Imp
        (8, '0', 8, ' ', Direction::Left),
        // 8, '1' - Imp
        (8, '+', 0, ' ', Direction::Right),
    ]);
    let machine = Classic::new(program, ' ').unwrap();

    // Change and try to run this example!
    let lhs = "10101";
    let rhs = "111";
    // --------------

    let mut expr = String::new();
    expr.push_str(lhs);
    expr.push('+');
    expr.push_str(rhs);
    let tape = Tape::from(expr);

    let res = machine.translate_std(tape).unwrap();
    println!("{} + {} = {}", lhs, rhs, String::from_iter(res.as_vec()));
}
