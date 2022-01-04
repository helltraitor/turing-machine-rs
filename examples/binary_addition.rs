extern crate turing_machine_rs;

use turing_machine_rs::instruction::Move;
use turing_machine_rs::machines::Classic;
use turing_machine_rs::program::{Extend, Program};
use turing_machine_rs::state::Tape;
use turing_machine_rs::TuringMachine;

// For more comfortable coding, use Result<(), String>:
// `?` postfix symbol is better then `.unwrap()` postfix method call.
fn main() -> Result<(), String> {
    let mut program = Program::new(vec![' ', '0', '1', '+'], 8);
    program.extend([
        // Sub 1, also init zero check
        (1, ' ', 0, ' ', Move::None),
        (1, '0', 1, '0', Move::Left),
        (1, '1', 2, '0', Move::Right),
        (1, '+', 6, '+', Move::Right),
        // Subs part
        (2, ' ', 3, ' ', Move::Left),
        (2, '0', 2, '1', Move::Right),
        // 2, '1' -> Impl
        // 2, '+' -> Err
        //
        // Find + on left
        // 3, ' ' -> Err
        (3, '0', 3, '0', Move::Left),
        (3, '1', 3, '1', Move::Left),
        (3, '+', 4, '+', Move::Left),
        // Add 1
        (4, ' ', 5, '1', Move::Right),
        (4, '0', 5, '1', Move::Right),
        (4, '1', 4, '0', Move::Left),
        // 4, '+' -> Err
        //
        // Find + on rigth
        // 5, ' ' -> Imp
        (5, '0', 5, '0', Move::Right),
        (5, '1', 5, '1', Move::Right),
        (5, '+', 6, '+', Move::Right),
        // Zero check
        (6, ' ', 8, ' ', Move::Left),
        (6, '0', 6, '0', Move::Right),
        (6, '1', 7, '1', Move::Right),
        // 6, '+' -> Err
        //
        // Find last num
        (7, ' ', 1, ' ', Move::Left),
        (7, '0', 7, '0', Move::Right),
        (7, '1', 7, '1', Move::Right),
        // 7, '+' -> Err
        //
        // Clear + and after
        // 8, ' ' - Imp
        (8, '0', 8, ' ', Move::Left),
        // 8, '1' - Imp
        (8, '+', 0, ' ', Move::Right),
    ])?;
    let machine = Classic::new(program, ' ')?;

    // Change and try to run this example!
    let lhs = "10101";
    let rhs = "111";
    // -----------------------------------
    let tape = Tape::from(format!("{}+{}", lhs, rhs));

    let res = machine.translate_std(tape)?;
    println!("{} + {} = {}", lhs, rhs, String::from_iter(res.as_vec()));

    Ok(())
}
