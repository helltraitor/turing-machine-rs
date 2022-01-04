use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use turing_machine_rs::instruction::Move;
use turing_machine_rs::machines::{Classic, Debugger};
use turing_machine_rs::program::{Extend, Program};
use turing_machine_rs::state::{Configuration, Tape};
use turing_machine_rs::TuringMachine;

#[cfg(test)]
mod copy {
    use super::*;

    fn new_custom_machine() -> Classic<char> {
        let mut program = Program::new(vec![' '], 1);
        program.extend([(1, ' ', 1, ' ', Move::Right)]);

        Classic::new(program, ' ').unwrap()
    }

    #[test]
    fn success_creation() {
        let machine = new_custom_machine();
        let _ = Debugger::new(machine);
    }

    #[test]
    fn set_handlers() {
        let machine = new_custom_machine();
        let mut debugger = Debugger::new(machine);

        let conf = Configuration::new_nrm(Tape::from("   ")).unwrap();
        let buffer = Rc::new(RefCell::new(String::new()));

        let c_buffer = buffer.clone();
        debugger.set_c_handler(move |_| {
            let mut buffer = c_buffer.borrow_mut();
            buffer.push('c');
        });
        let conf = debugger.execute_once(conf).unwrap();

        let i_buffer = buffer.clone();
        debugger.set_i_handler(move |_, _| {
            let mut buffer = i_buffer.borrow_mut();
            buffer.push('i');
        });
        debugger.execute_once(conf).unwrap();

        assert_eq!(String::from("cci"), buffer.deref().borrow().as_ref());
    }
}

#[cfg(test)]
mod clone {
    use super::*;

    fn new_custom_machine() -> Classic<Box<char>> {
        let mut program = Program::new(vec![Box::new(' ')], 1);
        program.extend([(1, Box::new(' '), 1, Box::new(' '), Move::Right)]);

        Classic::new(program, Box::new(' ')).unwrap()
    }

    #[test]
    fn creation() {
        let machine = new_custom_machine();
        let _ = Debugger::new(machine);
    }

    #[test]
    fn set_handlers() {
        let machine = new_custom_machine();
        let mut debugger = Debugger::new(machine);

        let conf = Configuration::new_nrm(Tape::new("   ".chars().map(|ch| Box::new(ch)))).unwrap();
        let buffer = Rc::new(RefCell::new(String::new()));

        let c_buffer = buffer.clone();
        debugger.set_c_handler(move |_| {
            let mut buffer = c_buffer.borrow_mut();
            buffer.push('c');
        });
        let conf = debugger.execute_once(conf).unwrap();

        let i_buffer = buffer.clone();
        debugger.set_i_handler(move |_, _| {
            let mut buffer = i_buffer.borrow_mut();
            buffer.push('i');
        });
        debugger.execute_once(conf).unwrap();

        assert_eq!(String::from("cci"), buffer.deref().borrow().as_ref());
    }
}

#[cfg(test)]
mod copy_turing_machine {
    use super::*;

    fn new_zerofy_machine() -> Classic<char> {
        let mut program = Program::new(vec!['0', '1'], 4);
        program.extend([
            (1, '0', 2, '0', Move::Right),
            (2, '0', 3, '0', Move::Left),
            (2, '1', 2, '1', Move::Right),
            (3, '0', 0, '0', Move::None),
            (3, '1', 4, '0', Move::None),
            (4, '0', 3, '0', Move::Left),
        ]);
        Classic::new(program, '0').unwrap()
    }

    #[test]
    fn execute() {
        let machine = new_zerofy_machine();
        let debugger = Debugger::new(machine);

        let conf = Configuration::new_nrm(Tape::from("0110")).unwrap();
        let result = debugger.execute(conf).unwrap();

        let mut expected = Configuration::new_nrm(Tape::from("0000")).unwrap();
        expected.state = 0;

        assert_eq!(expected, result);
    }

    #[test]
    fn execute_once() {
        let machine = new_zerofy_machine();
        let debugger = Debugger::new(machine);

        let conf = Configuration::new_nrm(Tape::from("0110")).unwrap();
        let result = debugger
            .execute_once(debugger.execute_once(conf).unwrap())
            .unwrap();

        let expected = Configuration::new(Tape::from("0110"), 2, 2).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn execute_until() {
        let machine = new_zerofy_machine();
        let debugger = Debugger::new(machine);

        let conf = Configuration::new_nrm(Tape::from("0110")).unwrap();

        let result = debugger
            .execute_until(conf, |conf| conf.state == 3)
            .unwrap();

        assert_eq!(
            Configuration::new(Tape::from("0110"), 2, 3).unwrap(),
            result
        );
    }

    #[test]
    fn translate_std() {
        let mut program = Program::new(vec!['0', '1'], 3);
        program.extend([
            (1, '0', 2, '0', Move::Right),
            (1, '1', 1, '1', Move::Left),
            (2, '0', 3, '1', Move::Left),
            (2, '1', 2, '1', Move::Right),
            (3, '0', 0, '0', Move::None),
            (3, '1', 3, '0', Move::Left),
        ]);
        let machine = Classic::new(program, '0').unwrap();
        let debugger = Debugger::new(machine);

        let result = debugger.translate_std(Tape::from("010")).unwrap();

        assert_eq!(result, Tape::from("0101"));
    }

    #[test]
    fn translate_nrm() {
        let mut program = Program::new(vec!['0', '1'], 3);
        program.extend([
            (1, '0', 2, '0', Move::Right),
            (1, '1', 1, '1', Move::Left),
            (2, '0', 3, '1', Move::Left),
            (2, '1', 2, '1', Move::Right),
            (3, '0', 0, '0', Move::None),
            (3, '1', 3, '0', Move::Left),
        ]);
        let machine = Classic::new(program, '0').unwrap();
        let debugger = Debugger::new(machine);

        let result = debugger.translate_nrm(Tape::from("010")).unwrap();

        let expected = Tape::from("001");

        assert_eq!(expected, result);
    }
}

#[cfg(test)]
mod clone_turing_machine {
    use super::*;

    fn new_zerofy_machine() -> Classic<Box<char>> {
        let mut program = Program::new(vec![Box::new('0'), Box::new('1')], 4);
        program.extend([
            (1, Box::new('0'), 2, Box::new('0'), Move::Right),
            (2, Box::new('0'), 3, Box::new('0'), Move::Left),
            (2, Box::new('1'), 2, Box::new('1'), Move::Right),
            (3, Box::new('0'), 0, Box::new('0'), Move::None),
            (3, Box::new('1'), 4, Box::new('0'), Move::None),
            (4, Box::new('0'), 3, Box::new('0'), Move::Left),
        ]);
        Classic::new(program, Box::new('0')).unwrap()
    }

    #[test]
    fn execute() {
        let machine = new_zerofy_machine();
        let debugger = Debugger::new(machine);

        let conf =
            Configuration::new_nrm(Tape::new("0110".chars().map(|ch| Box::new(ch)))).unwrap();
        let result = debugger.execute(conf).unwrap();

        let mut expected =
            Configuration::new_nrm(Tape::new("0000".chars().map(|ch| Box::new(ch)))).unwrap();
        expected.state = 0;

        assert_eq!(expected, result);
    }

    #[test]
    fn execute_once() {
        let machine = new_zerofy_machine();
        let debugger = Debugger::new(machine);

        let conf =
            Configuration::new_nrm(Tape::new("0110".chars().map(|ch| Box::new(ch)))).unwrap();
        let result = debugger
            .execute_once(debugger.execute_once(conf).unwrap())
            .unwrap();

        let expected =
            Configuration::new(Tape::new("0110".chars().map(|ch| Box::new(ch))), 2, 2).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn execute_until() {
        let machine = new_zerofy_machine();
        let debugger = Debugger::new(machine);

        let conf =
            Configuration::new_nrm(Tape::new("0110".chars().map(|ch| Box::new(ch)))).unwrap();

        let result = debugger
            .execute_until(conf, |conf| conf.state == 3)
            .unwrap();

        assert_eq!(
            Configuration::new(Tape::new("0110".chars().map(|ch| Box::new(ch))), 2, 3).unwrap(),
            result
        );
    }

    #[test]
    fn translate_std() {
        let mut program = Program::new(vec![Box::new('0'), Box::new('1')], 3);
        program.extend([
            (1, Box::new('0'), 2, Box::new('0'), Move::Right),
            (1, Box::new('1'), 1, Box::new('1'), Move::Left),
            (2, Box::new('0'), 3, Box::new('1'), Move::Left),
            (2, Box::new('1'), 2, Box::new('1'), Move::Right),
            (3, Box::new('0'), 0, Box::new('0'), Move::None),
            (3, Box::new('1'), 3, Box::new('0'), Move::Left),
        ]);
        let machine = Classic::new(program, Box::new('0')).unwrap();
        let debugger = Debugger::new(machine);

        let expected = debugger
            .translate_std(Tape::new("010".chars().map(|ch| Box::new(ch))))
            .unwrap();

        assert_eq!(expected, Tape::new("0101".chars().map(|ch| Box::new(ch))));
    }

    #[test]
    fn translate_nrm() {
        let mut program = Program::new(vec![Box::new('0'), Box::new('1')], 3);
        program.extend([
            (1, Box::new('0'), 2, Box::new('0'), Move::Right),
            (1, Box::new('1'), 1, Box::new('1'), Move::Left),
            (2, Box::new('0'), 3, Box::new('1'), Move::Left),
            (2, Box::new('1'), 2, Box::new('1'), Move::Right),
            (3, Box::new('0'), 0, Box::new('0'), Move::None),
            (3, Box::new('1'), 3, Box::new('0'), Move::Left),
        ]);
        let machine = Classic::new(program, Box::new('0')).unwrap();
        let debugger = Debugger::new(machine);

        let result = debugger
            .translate_nrm(Tape::new("010".chars().map(|ch| Box::new(ch))))
            .unwrap();

        let expected = Tape::new("001".chars().map(|ch| Box::new(ch)));

        assert_eq!(expected, result);
    }
}
