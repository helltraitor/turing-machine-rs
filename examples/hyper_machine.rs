extern crate turing_machine_rs;

use turing_machine_rs::instruction::Direction;
use turing_machine_rs::machines::Classic;
use turing_machine_rs::program::{Extend, Program};
use turing_machine_rs::state::{Configuration, Tape};
use turing_machine_rs::TuringMachine;

fn main() {
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
        9,
    );
    // This is simplest implementation of `change choose second to choose third` machine
    program.extend([
        // Find l_shift
        (1, r_shift.clone(), 1, r_shift.clone(), Direction::Right),
        (1, trans.clone(), 1, trans.clone(), Direction::Right),
        (1, zerofy.clone(), 1, zerofy.clone(), Direction::Right),
        (1, l_shift.clone(), 2, stand.clone(), Direction::Left),
        // Clear until r_shift
        (2, zerofy.clone(), 2, stand.clone(), Direction::Left),
        (2, trans.clone(), 2, stand.clone(), Direction::Left),
        (2, r_shift.clone(), 3, r_shift.clone(), Direction::Right),
        //
        // Set second r_shift
        (3, stand.clone(), 4, r_shift.clone(), Direction::Right),
        // Set first trans
        (4, stand.clone(), 5, trans.clone(), Direction::Right),
        // Set first zerofy
        (5, stand.clone(), 6, zerofy.clone(), Direction::Right),
        // Set first l_shift
        (6, stand.clone(), 7, l_shift.clone(), Direction::Right),
        // Set second trans
        (7, stand.clone(), 8, trans.clone(), Direction::Right),
        // Set second zerofy
        (8, stand.clone(), 9, zerofy.clone(), Direction::Right),
        // Set second l_shift and stop execution
        (9, stand.clone(), 0, l_shift.clone(), Direction::Center),
    ]);

    let hyper_machine = Classic::new(program, stand.clone()).unwrap();
    let choose_second = Tape::new(vec![
        r_shift.clone(),
        trans.clone(),
        zerofy.clone(),
        l_shift.clone(),
    ]);
    let result_choose_third = hyper_machine.translate_nrm(choose_second).unwrap();

    let expected_choose_third = Tape::new(vec![
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
    let mut conf = Configuration::new_nrm(tape.clone()).unwrap();
    for machine in result_choose_third.as_vec() {
        conf = machine.execute(conf).unwrap();
        conf.state = 1
    }
    println!(
        "Choose third machine translate {} into {}",
        String::from_iter(tape.as_vec()),
        String::from_iter(conf.tape().as_vec())
    );
}

// This module just contains several nrm machines
mod nrm_machines {
    use super::*;

    pub fn new_stand_machine() -> Classic<char> {
        let mut program = Program::new(vec!['0', '1'], 1);
        program.extend([
            (1, '0', 0, '0', Direction::Center),
            (1, '1', 0, '1', Direction::Center),
        ]);
        Classic::new(program, '0').unwrap()
    }

    pub fn new_zerofy_machine() -> Classic<char> {
        let mut program = Program::new(vec!['0', '1'], 4);
        program.extend([
            (1, '0', 2, '0', Direction::Right),
            (2, '0', 3, '0', Direction::Left),
            (2, '1', 2, '1', Direction::Right),
            (3, '0', 0, '0', Direction::Center),
            (3, '1', 4, '0', Direction::Center),
            (4, '0', 3, '0', Direction::Left),
        ]);
        Classic::new(program, '0').unwrap()
    }

    pub fn new_left_shift_machine() -> Classic<char> {
        let mut program = Program::new(vec!['0', '1'], 2);
        program.extend([
            (1, '0', 2, '0', Direction::Left),
            (2, '0', 0, '0', Direction::Center),
            (2, '1', 2, '1', Direction::Left),
        ]);
        Classic::new(program, '0').unwrap()
    }

    pub fn new_right_shift_machine() -> Classic<char> {
        let mut program = Program::new(vec!['0', '1'], 2);
        program.extend([
            (1, '0', 2, '0', Direction::Right),
            (2, '0', 0, '0', Direction::Center),
            (2, '1', 2, '1', Direction::Right),
        ]);
        Classic::new(program, '0').unwrap()
    }

    pub fn new_trans_machine() -> Classic<char> {
        let mut program = Program::new(vec!['0', '1'], 19);
        program.extend([
            (1, '0', 2, '0', Direction::Right),
            (2, '0', 3, '0', Direction::Center),
            (2, '1', 2, '1', Direction::Right),
            (3, '0', 4, '0', Direction::Left),
            (4, '0', 7, '0', Direction::Center),
            (4, '1', 5, '0', Direction::Center),
            (5, '0', 6, '0', Direction::Left),
            (6, '0', 7, '1', Direction::Center),
            (6, '1', 6, '1', Direction::Left),
            (7, '0', 16, '1', Direction::Center),
            (7, '1', 8, '1', Direction::Left),
            (8, '0', 18, '0', Direction::Right),
            (8, '1', 9, '0', Direction::Center),
            (9, '0', 10, '0', Direction::Right),
            (10, '0', 11, '1', Direction::Center),
            (10, '1', 10, '1', Direction::Right),
            (11, '1', 12, '1', Direction::Left),
            (12, '1', 13, '0', Direction::Center),
            (13, '0', 14, '0', Direction::Left),
            (14, '0', 15, '1', Direction::Center),
            (14, '1', 14, '1', Direction::Left),
            (15, '0', 7, '0', Direction::Center),
            (15, '1', 7, '1', Direction::Center),
            (16, '1', 17, '1', Direction::Left),
            (17, '0', 19, '0', Direction::Right),
            (17, '1', 15, '0', Direction::Center),
            (18, '0', 0, '0', Direction::Center),
            (18, '1', 18, '1', Direction::Right),
            (19, '1', 0, '0', Direction::Center),
        ]);
        Classic::new(program, '0').unwrap()
    }
}
