use turing_machine_rs::instruction::{Head, Instruction, Move, State, Tail};
use turing_machine_rs::machines::Classic;
use turing_machine_rs::program::{Extend, Program};
use turing_machine_rs::state::{Configuration, Tape};
use turing_machine_rs::{TuringMachine, With};

#[cfg(test)]
mod copy {
    use super::*;

    #[test]
    fn success_creation() {
        let program = Program::new(vec![' '], State(1));
        let _ = Classic::new(program, ' ').unwrap();
    }

    #[test]
    #[should_panic]
    fn fail_creation() {
        let program = Program::new(vec![' '], State(1));
        let _ = Classic::new(program, '_').unwrap();
    }
}

#[cfg(test)]
mod clone {
    use super::*;

    #[test]
    fn success_creation() {
        let program = Program::new(vec![Box::new(' ')], State(1));
        let _ = Classic::new(program, Box::new(' ')).unwrap();
    }

    #[test]
    #[should_panic]
    fn fail_creation() {
        let program = Program::new(vec![Box::new(' ')], State(1));
        let _ = Classic::new(program, Box::new('_')).unwrap();
    }
}

#[cfg(test)]
mod copy_turing_machine {
    use super::*;

    fn new_fail_machine() -> Classic<char> {
        let mut program = Program::new(vec!['0', '1'], State(3));
        program.extend([
            (1, '0', 2, '0', Move::Right),
            (1, '1', 1, '1', Move::Left),
            (3, '0', 0, '0', Move::None),
            (3, '1', 3, '0', Move::Left),
        ]);

        Classic::new(program, '0').unwrap()
    }

    fn new_success_machine() -> Classic<char> {
        let mut program = Program::new(vec!['0', '1'], State(3));
        program.extend([
            (1, '0', 2, '0', Move::Right),
            (1, '1', 1, '1', Move::Left),
            (2, '0', 3, '1', Move::Left),
            (2, '1', 2, '1', Move::Right),
            (3, '0', 0, '0', Move::None),
            (3, '1', 3, '0', Move::Left),
        ]);

        Classic::new(program, '0').unwrap()
    }

    #[test]
    fn execute() {
        let mut program = Program::new(vec![' ', '0', '1'], State(2));
        program.extend([
            (1, ' ', 2, ' ', Move::Right),
            (1, '0', 1, '1', Move::Left),
            (1, '1', 1, '0', Move::Left),
            (2, ' ', 0, ' ', Move::Left),
            (2, '0', 2, '0', Move::Right),
            (2, '1', 2, '1', Move::Right),
        ]);
        let machine = Classic::new(program, ' ').unwrap();

        let conf = Configuration::new(Tape::from("001100"), 5, State(0)).unwrap();
        let result = machine.execute(conf.clone()).unwrap();

        let expected = conf;

        assert_eq!(expected, result);

        let conf = Configuration::new_std(Tape::from("001100")).unwrap();
        let result = machine.execute(conf).unwrap();

        let expected = Configuration::new(Tape::from(" 110011 "), 6, State(0)).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn success_execute_once() {
        let mut program = Program::new(vec![' ', '1'], State(1));
        let _ = program.insert(Instruction::new(
            Head::new(State(1), '1'),
            Tail::new(State(1), '1', Move::Right),
        ));

        let machine = Classic::new(program, ' ').unwrap();

        let conf = Configuration::new_std(Tape::from("1")).unwrap();
        let result = machine.execute_once(conf).unwrap();

        let expected = Configuration::new(Tape::from("1 "), 1, State(1)).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    #[should_panic]
    fn fail_execute_once() {
        let mut program = Program::new(vec![' ', '1'], State(1));
        let _ = program.insert(Instruction::new(
            Head::new(State(1), '1'),
            Tail::new(State(1), '1', Move::Right),
        ));

        let machine = Classic::new(program, ' ').unwrap();

        let conf = Configuration::new_std(Tape::from(" ")).unwrap();
        machine.execute_once(conf).unwrap();
    }

    #[test]
    fn success_execute_until() {
        let machine = new_success_machine();

        let conf = Configuration::new_std(Tape::from("010")).unwrap();
        let result = machine
            .execute_until(conf, |conf| conf.state == State(3))
            .unwrap();

        let expected = Configuration::new(Tape::from("0101"), 2, State(3)).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    #[should_panic]
    fn fail_execute_until() {
        let machine = new_fail_machine();

        let conf = Configuration::new_std(Tape::from("010")).unwrap();
        let _ = machine
            .execute_until(conf, |conf| conf.state == State(3))
            .unwrap();
    }

    #[test]
    fn translate_std() {
        let machine = new_success_machine();

        let result = machine.translate_std(Tape::from("010")).unwrap();
        let expected = Tape::from("0101");
        assert_eq!(expected, result);
    }

    #[test]
    fn translate_nrm() {
        let machine = new_success_machine();

        let result = machine.translate_nrm(Tape::from("010")).unwrap();
        let expected = Tape::from("001");
        assert_eq!(expected, result);
    }
}

#[cfg(test)]
mod clone_turing_machine {
    use super::*;

    fn new_fail_machine() -> Classic<Box<char>> {
        let mut program = Program::new(vec![Box::new('0'), Box::new('1')], State(3));
        program.extend([
            (1, Box::new('0'), 2, Box::new('0'), Move::Right),
            (1, Box::new('1'), 1, Box::new('1'), Move::Left),
            (3, Box::new('0'), 0, Box::new('0'), Move::None),
            (3, Box::new('1'), 3, Box::new('0'), Move::Left),
        ]);

        Classic::new(program, Box::new('0')).unwrap()
    }

    fn new_success_machine() -> Classic<Box<char>> {
        let mut program = Program::new(vec![Box::new('0'), Box::new('1')], State(3));
        program.extend([
            (1, Box::new('0'), 2, Box::new('0'), Move::Right),
            (1, Box::new('1'), 1, Box::new('1'), Move::Left),
            (2, Box::new('0'), 3, Box::new('1'), Move::Left),
            (2, Box::new('1'), 2, Box::new('1'), Move::Right),
            (3, Box::new('0'), 0, Box::new('0'), Move::None),
            (3, Box::new('1'), 3, Box::new('0'), Move::Left),
        ]);

        Classic::new(program, Box::new('0')).unwrap()
    }

    #[test]
    fn execute() {
        let mut program = Program::new(vec![Box::new(' '), Box::new('0'), Box::new('1')], State(2));
        program.extend([
            (1, Box::new(' '), 2, Box::new(' '), Move::Right),
            (1, Box::new('0'), 1, Box::new('1'), Move::Left),
            (1, Box::new('1'), 1, Box::new('0'), Move::Left),
            (2, Box::new(' '), 0, Box::new(' '), Move::Left),
            (2, Box::new('0'), 2, Box::new('0'), Move::Right),
            (2, Box::new('1'), 2, Box::new('1'), Move::Right),
        ]);
        let machine = Classic::new(program, Box::new(' ')).unwrap();

        let conf = Configuration::new(
            Tape::new("001100".chars().map(|ch| Box::new(ch))),
            5,
            State(0),
        )
        .unwrap();
        let result = machine.execute(conf.clone()).unwrap();

        let expected = conf;

        assert_eq!(expected, result);

        let conf =
            Configuration::new_std(Tape::new("001100".chars().map(|ch| Box::new(ch)))).unwrap();
        let result = machine.execute(conf).unwrap();

        let expected = Configuration::new(
            Tape::new(" 110011 ".chars().map(|ch| Box::new(ch))),
            6,
            State(0),
        )
        .unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn success_execute_once() {
        let mut program = Program::new(vec![Box::new(' '), Box::new('1')], State(1));
        let _ = program.insert(Instruction::new(
            Head::new(State(1), Box::new('1')),
            Tail::new(State(1), Box::new('1'), Move::Right),
        ));

        let machine = Classic::new(program, Box::new(' ')).unwrap();

        let conf = Configuration::new_std(Tape::new([Box::new('1')])).unwrap();
        let result = machine.execute_once(conf).unwrap();

        let expected =
            Configuration::new(Tape::new([Box::new('1'), Box::new(' ')]), 1, State(1)).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    #[should_panic]
    fn fail_execute_once() {
        let mut program = Program::new(vec![Box::new(' '), Box::new('1')], State(1));
        let _ = program.insert(Instruction::new(
            Head::new(State(1), Box::new('1')),
            Tail::new(State(1), Box::new('1'), Move::Right),
        ));

        let machine = Classic::new(program, Box::new(' ')).unwrap();

        let conf = Configuration::new_std(Tape::new([Box::new(' ')])).unwrap();
        machine.execute_once(conf).unwrap();
    }

    #[test]
    fn success_execute_until() {
        let machine = new_success_machine();

        let conf = Configuration::new_std(Tape::new("010".chars().map(|ch| Box::new(ch)))).unwrap();
        let result = machine
            .execute_until(conf, |conf| conf.state == State(3))
            .unwrap();

        let expected = Configuration::new(
            Tape::new("0101".chars().map(|ch| Box::new(ch))),
            2,
            State(3),
        )
        .unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    #[should_panic]
    fn fail_execute_until() {
        let machine = new_fail_machine();

        let conf = Configuration::new_std(Tape::new("010".chars().map(|ch| Box::new(ch)))).unwrap();
        let _ = machine
            .execute_until(conf, |conf| conf.state == State(3))
            .unwrap();
    }

    #[test]
    fn translate_std() {
        let machine = new_success_machine();

        let result = machine
            .translate_std(Tape::new("010".chars().map(|ch| Box::new(ch))))
            .unwrap();
        let expected = Tape::new("0101".chars().map(|ch| Box::new(ch)));
        assert_eq!(expected, result);
    }

    #[test]
    fn translate_nrm() {
        let machine = new_success_machine();

        let result = machine
            .translate_nrm(Tape::new("010".chars().map(|ch| Box::new(ch))))
            .unwrap();
        let expected = Tape::new("001".chars().map(|ch| Box::new(ch)));
        assert_eq!(expected, result);
    }
}

#[cfg(test)]
mod copy_with_for_classic {
    use super::*;

    fn new_zerofy_machine() -> Classic<char> {
        let mut program = Program::new(vec!['0', '1'], State(4));
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

    fn new_left_shift_machine() -> Classic<char> {
        let mut program = Program::new(vec!['0', '1'], State(2));
        program.extend([
            (1, '0', 2, '0', Move::Left),
            (2, '0', 0, '0', Move::None),
            (2, '1', 2, '1', Move::Left),
        ]);
        Classic::new(program, '0').unwrap()
    }

    fn new_right_shift_machine() -> Classic<char> {
        let mut program = Program::new(vec!['0', '1'], State(2));
        program.extend([
            (1, '0', 2, '0', Move::Right),
            (2, '0', 0, '0', Move::None),
            (2, '1', 2, '1', Move::Right),
        ]);
        Classic::new(program, '0').unwrap()
    }

    fn new_trans_machine() -> Classic<char> {
        let mut program = Program::new(vec!['0', '1'], State(19));
        program.extend([
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
        ]);
        Classic::new(program, '0').unwrap()
    }

    #[test]
    fn self_with_other_success() {
        let zerofy = new_zerofy_machine();
        let left_shift = new_left_shift_machine();
        let right_shift = new_right_shift_machine();
        let trans = new_trans_machine();

        // This test checks only object with object
        // Thats why it needs to unpack value and try again

        // choose the second from three
        let choose_machine = right_shift.with(&trans).unwrap();
        let choose_machine = choose_machine.with(&right_shift).unwrap();
        let choose_machine = choose_machine.with(&zerofy).unwrap();
        let choose_machine = choose_machine.with(&left_shift).unwrap();
        let choose_machine = choose_machine.with(&zerofy).unwrap();
        let choose_machine = choose_machine.with(&left_shift).unwrap();

        let tape = Tape::from("0101101110");
        let result = choose_machine.translate_nrm(tape).unwrap();

        let expected = Tape::from("0110000000");
        assert_eq!(expected, result);

        let tape = Tape::from("010111010");
        let result = choose_machine.translate_nrm(tape).unwrap();

        let expected = Tape::from("011100000");

        assert_eq!(expected, result);
    }

    #[test]
    #[should_panic]
    fn self_with_other_fail() {
        let zero_one = Classic::new(Program::new(vec!['0', '1'], State(2)), '0').unwrap();
        let zero_one_two = Classic::new(Program::new(vec!['0', '1', '2'], State(2)), '0').unwrap();

        zero_one.with(&zero_one_two).unwrap();
    }

    #[test]
    fn option_with_other_success() {
        let zerofy = new_zerofy_machine();
        let left_shift = new_left_shift_machine();
        let right_shift = new_right_shift_machine();
        let trans = new_trans_machine();

        let choose_machine = right_shift
            .with(&trans)
            .with(&right_shift)
            .with(&zerofy)
            .with(&left_shift)
            .with(&zerofy)
            .with(&left_shift);

        let choose_machine = choose_machine.unwrap();

        let tape = Tape::from("0101101110");
        let result = choose_machine.translate_nrm(tape).unwrap();

        let expected = Tape::from("0110000000");

        assert_eq!(expected, result);

        let tape = Tape::from("010111010");
        let result = choose_machine.translate_nrm(tape).unwrap();

        let expected = Tape::from("011100000");

        assert_eq!(expected, result);
    }

    #[test]
    #[should_panic]
    fn option_with_other_fail() {
        let zero_one = Classic::new(Program::new(vec!['0', '1'], State(2)), '0');
        let zero_one_two = Classic::new(Program::new(vec!['0', '1', '2'], State(2)), '0').unwrap();

        zero_one.with(&zero_one_two).unwrap();
    }
}

#[cfg(test)]
mod clone_with_for_classic {
    use super::*;

    fn new_zerofy_machine() -> Classic<Box<char>> {
        let mut program = Program::new(vec![Box::new('0'), Box::new('1')], State(4));
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

    fn new_left_shift_machine() -> Classic<Box<char>> {
        let mut program = Program::new(vec![Box::new('0'), Box::new('1')], State(2));
        program.extend([
            (1, Box::new('0'), 2, Box::new('0'), Move::Left),
            (2, Box::new('0'), 0, Box::new('0'), Move::None),
            (2, Box::new('1'), 2, Box::new('1'), Move::Left),
        ]);
        Classic::new(program, Box::new('0')).unwrap()
    }

    fn new_right_shift_machine() -> Classic<Box<char>> {
        let mut program = Program::new(vec![Box::new('0'), Box::new('1')], State(2));
        program.extend([
            (1, Box::new('0'), 2, Box::new('0'), Move::Right),
            (2, Box::new('0'), 0, Box::new('0'), Move::None),
            (2, Box::new('1'), 2, Box::new('1'), Move::Right),
        ]);
        Classic::new(program, Box::new('0')).unwrap()
    }

    fn new_trans_machine() -> Classic<Box<char>> {
        let mut program = Program::new(vec![Box::new('0'), Box::new('1')], State(19));
        program.extend([
            (1, Box::new('0'), 2, Box::new('0'), Move::Right),
            (2, Box::new('0'), 3, Box::new('0'), Move::None),
            (2, Box::new('1'), 2, Box::new('1'), Move::Right),
            (3, Box::new('0'), 4, Box::new('0'), Move::Left),
            (4, Box::new('0'), 7, Box::new('0'), Move::None),
            (4, Box::new('1'), 5, Box::new('0'), Move::None),
            (5, Box::new('0'), 6, Box::new('0'), Move::Left),
            (6, Box::new('0'), 7, Box::new('1'), Move::None),
            (6, Box::new('1'), 6, Box::new('1'), Move::Left),
            (7, Box::new('0'), 16, Box::new('1'), Move::None),
            (7, Box::new('1'), 8, Box::new('1'), Move::Left),
            (8, Box::new('0'), 18, Box::new('0'), Move::Right),
            (8, Box::new('1'), 9, Box::new('0'), Move::None),
            (9, Box::new('0'), 10, Box::new('0'), Move::Right),
            (10, Box::new('0'), 11, Box::new('1'), Move::None),
            (10, Box::new('1'), 10, Box::new('1'), Move::Right),
            (11, Box::new('1'), 12, Box::new('1'), Move::Left),
            (12, Box::new('1'), 13, Box::new('0'), Move::None),
            (13, Box::new('0'), 14, Box::new('0'), Move::Left),
            (14, Box::new('0'), 15, Box::new('1'), Move::None),
            (14, Box::new('1'), 14, Box::new('1'), Move::Left),
            (15, Box::new('0'), 7, Box::new('0'), Move::None),
            (15, Box::new('1'), 7, Box::new('1'), Move::None),
            (16, Box::new('1'), 17, Box::new('1'), Move::Left),
            (17, Box::new('0'), 19, Box::new('0'), Move::Right),
            (17, Box::new('1'), 15, Box::new('0'), Move::None),
            (18, Box::new('0'), 0, Box::new('0'), Move::None),
            (18, Box::new('1'), 18, Box::new('1'), Move::Right),
            (19, Box::new('1'), 0, Box::new('0'), Move::None),
        ]);
        Classic::new(program, Box::new('0')).unwrap()
    }

    #[test]
    fn self_with_other_success() {
        let zerofy = new_zerofy_machine();
        let left_shift = new_left_shift_machine();
        let right_shift = new_right_shift_machine();
        let trans = new_trans_machine();

        // This test checks only object with object
        // Thats why it needs to unpack value and try again

        // choose the second from three
        let choose_machine = right_shift.with(&trans).unwrap();
        let choose_machine = choose_machine.with(&right_shift).unwrap();
        let choose_machine = choose_machine.with(&zerofy).unwrap();
        let choose_machine = choose_machine.with(&left_shift).unwrap();
        let choose_machine = choose_machine.with(&zerofy).unwrap();
        let choose_machine = choose_machine.with(&left_shift).unwrap();

        let tape = Tape::new("0101101110".chars().map(|ch| Box::new(ch)));
        let result = choose_machine.translate_nrm(tape).unwrap();

        let expected = Tape::new("0110000000".chars().map(|ch| Box::new(ch)));
        assert_eq!(expected, result);

        let tape = Tape::new("010111010".chars().map(|ch| Box::new(ch)));
        let result = choose_machine.translate_nrm(tape).unwrap();

        let expected = Tape::new("011100000".chars().map(|ch| Box::new(ch)));

        assert_eq!(expected, result);
    }

    #[test]
    #[should_panic]
    fn self_with_other_fail() {
        let zero_one = Classic::new(
            Program::new(vec![Box::new('0'), Box::new('1')], State(2)),
            Box::new('0'),
        )
        .unwrap();
        let zero_one_two = Classic::new(
            Program::new(vec![Box::new('0'), Box::new('1'), Box::new('2')], State(2)),
            Box::new('0'),
        )
        .unwrap();

        zero_one.with(&zero_one_two).unwrap();
    }

    #[test]
    fn option_with_other_success() {
        let zerofy = new_zerofy_machine();
        let left_shift = new_left_shift_machine();
        let right_shift = new_right_shift_machine();
        let trans = new_trans_machine();

        let choose_machine = right_shift
            .with(&trans)
            .with(&right_shift)
            .with(&zerofy)
            .with(&left_shift)
            .with(&zerofy)
            .with(&left_shift);

        let choose_machine = choose_machine.unwrap();

        let tape = Tape::new("0101101110".chars().map(|ch| Box::new(ch)));
        let result = choose_machine.translate_nrm(tape).unwrap();

        let expected = Tape::new("0110000000".chars().map(|ch| Box::new(ch)));

        assert_eq!(expected, result);

        let tape = Tape::new("010111010".chars().map(|ch| Box::new(ch)));
        let result = choose_machine.translate_nrm(tape).unwrap();

        let expected = Tape::new("011100000".chars().map(|ch| Box::new(ch)));

        assert_eq!(expected, result);
    }

    #[test]
    #[should_panic]
    fn option_with_other_fail() {
        let zero_one = Classic::new(
            Program::new(vec![Box::new('0'), Box::new('1')], State(2)),
            Box::new('0'),
        );
        let zero_one_two = Classic::new(
            Program::new(vec![Box::new('0'), Box::new('1'), Box::new('2')], State(2)),
            Box::new('0'),
        )
        .unwrap();

        zero_one.with(&zero_one_two).unwrap();
    }
}
