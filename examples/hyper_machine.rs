extern crate turing_machine_rs;

use turing_machine_rs::instruction::{Move, State};
use turing_machine_rs::machines::Classic;
use turing_machine_rs::program::{Extend, Program};
use turing_machine_rs::state::{Configuration, Tape};
use turing_machine_rs::TuringMachine;

// For more comfortable coding, use Result<(), String>:
// `?` postfix symbol is better then `.unwrap()` postfix method call.
fn main() -> Result<(), String> {
    use nrm_machines::*;

    let stand = new_stand_machine();
    let zerofy = new_zerofy_machine();
    let l_shift = new_left_shift_machine();
    let r_shift = new_right_shift_machine();
    let trans = new_trans_machine();

    let mut program = Program::new(
        vec![
            stand.clone(),
            zerofy.clone(),
            l_shift.clone(),
            r_shift.clone(),
            trans.clone(),
        ],
        State(9),
    );
    // This is simplest implementation of `change choose second to choose third` machine
    program.extend([
        // Find l_shift
        (1, r_shift.clone(), 1, r_shift.clone(), Move::Right),
        (1, trans.clone(), 1, trans.clone(), Move::Right),
        (1, zerofy.clone(), 1, zerofy.clone(), Move::Right),
        (1, l_shift.clone(), 2, stand.clone(), Move::Left),
        // Clear until r_shift
        (2, zerofy.clone(), 2, stand.clone(), Move::Left),
        (2, trans.clone(), 2, stand.clone(), Move::Left),
        (2, r_shift.clone(), 3, r_shift.clone(), Move::Right),
        //
        // Set second r_shift
        (3, stand.clone(), 4, r_shift.clone(), Move::Right),
        // Set first trans
        (4, stand.clone(), 5, trans.clone(), Move::Right),
        // Set first zerofy
        (5, stand.clone(), 6, zerofy.clone(), Move::Right),
        // Set first l_shift
        (6, stand.clone(), 7, l_shift.clone(), Move::Right),
        // Set second trans
        (7, stand.clone(), 8, trans.clone(), Move::Right),
        // Set second zerofy
        (8, stand.clone(), 9, zerofy.clone(), Move::Right),
        // Set second l_shift and stop execution
        (9, stand.clone(), 0, l_shift.clone(), Move::None),
    ])?;

    let hyper_machine = Classic::new(program, stand.clone())?;
    let choose_second = Tape::new([
        r_shift.clone(),
        trans.clone(),
        zerofy.clone(),
        l_shift.clone(),
    ]);
    let result_choose_third = hyper_machine.translate_nrm(choose_second)?;

    let expected_choose_third = Tape::new([
        r_shift.clone(),
        r_shift.clone(),
        trans.clone(),
        zerofy.clone(),
        l_shift.clone(),
        trans.clone(),
        zerofy.clone(),
        l_shift.clone(),
    ]);

    assert_eq!(expected_choose_third, result_choose_third);
    println!("If you're reading this, hyper machine successful transform choose second machine");

    let tape = Tape::from("0101101110");
    let mut conf = Configuration::new_nrm(tape.clone())?;
    for machine in result_choose_third.as_vec() {
        conf = machine.execute(conf).unwrap();
        conf.state = State(1)
    }
    println!(
        "Choose third machine translate {} into {}",
        String::from_iter(tape.as_vec()),
        String::from_iter(conf.tape().as_vec())
    );

    Ok(())
}

// This module just contains several nrm machines
mod nrm_machines {
    use super::*;

    pub fn new_stand_machine() -> Classic<char> {
        let mut program = Program::new(vec!['0', '1'], State(1));
        program
            .extend([(1, '0', 0, '0', Move::None), (1, '1', 0, '1', Move::None)])
            .unwrap();
        Classic::new(program, '0').unwrap()
    }

    pub fn new_zerofy_machine() -> Classic<char> {
        let mut program = Program::new(vec!['0', '1'], State(4));
        program
            .extend([
                (1, '0', 2, '0', Move::Right),
                (2, '0', 3, '0', Move::Left),
                (2, '1', 2, '1', Move::Right),
                (3, '0', 0, '0', Move::None),
                (3, '1', 4, '0', Move::None),
                (4, '0', 3, '0', Move::Left),
            ])
            .unwrap();
        Classic::new(program, '0').unwrap()
    }

    pub fn new_left_shift_machine() -> Classic<char> {
        let mut program = Program::new(vec!['0', '1'], State(2));
        program
            .extend([
                (1, '0', 2, '0', Move::Left),
                (2, '0', 0, '0', Move::None),
                (2, '1', 2, '1', Move::Left),
            ])
            .unwrap();
        Classic::new(program, '0').unwrap()
    }

    pub fn new_right_shift_machine() -> Classic<char> {
        let mut program = Program::new(vec!['0', '1'], State(2));
        program
            .extend([
                (1, '0', 2, '0', Move::Right),
                (2, '0', 0, '0', Move::None),
                (2, '1', 2, '1', Move::Right),
            ])
            .unwrap();
        Classic::new(program, '0').unwrap()
    }

    pub fn new_trans_machine() -> Classic<char> {
        let mut program = Program::new(vec!['0', '1'], State(19));
        program
            .extend([
                (1, '0', 2, '0', Move::Right),
                (2, '0', 3, '0', Move::None),
                (2, '1', 2, '1', Move::Right),
                (3, '0', 4, '0', Move::Left),
                (4, '0', 7, '0', Move::None),
                (4, '1', 5, '0', Move::None),
                (5, '0', 6, '0', Move::Left),
                (6, '0', 7, '1', Move::None),
                (6, '1', 6, '1', Move::Left),
                (7, '0', 16, '1', Move::None),
                (7, '1', 8, '1', Move::Left),
                (8, '0', 18, '0', Move::Right),
                (8, '1', 9, '0', Move::None),
                (9, '0', 10, '0', Move::Right),
                (10, '0', 11, '1', Move::None),
                (10, '1', 10, '1', Move::Right),
                (11, '1', 12, '1', Move::Left),
                (12, '1', 13, '0', Move::None),
                (13, '0', 14, '0', Move::Left),
                (14, '0', 15, '1', Move::None),
                (14, '1', 14, '1', Move::Left),
                (15, '0', 7, '0', Move::None),
                (15, '1', 7, '1', Move::None),
                (16, '1', 17, '1', Move::Left),
                (17, '0', 19, '0', Move::Right),
                (17, '1', 15, '0', Move::None),
                (18, '0', 0, '0', Move::None),
                (18, '1', 18, '1', Move::Right),
                (19, '1', 0, '0', Move::None),
            ])
            .unwrap();
        Classic::new(program, '0').unwrap()
    }
}
