use turing_machine_rs::instruction::{Direction, Head, Instruction, Tail};
use turing_machine_rs::machines::Classic;
use turing_machine_rs::program::{ExtendBy, Program};
use turing_machine_rs::state::{Configuration, Tape};
use turing_machine_rs::{TuringMachine, With};

#[cfg(test)]
mod copy {
    use super::*;

    #[test]
    fn success_creation() {
        let program = Program::new(vec![' '], 1);
        let _ = Classic::new(program, ' ').unwrap();
    }

    #[test]
    #[should_panic]
    fn fail_creation() {
        let program = Program::new(vec![' '], 1);
        let _ = Classic::new(program, '_').unwrap();
    }
}

#[cfg(test)]
mod clone {
    use super::*;

    #[test]
    fn success_creation() {
        let program = Program::new(vec![Box::new(' ')], 1);
        let _ = Classic::new(program, Box::new(' ')).unwrap();
    }

    #[test]
    #[should_panic]
    fn fail_creation() {
        let program = Program::new(vec![Box::new(' ')], 1);
        let _ = Classic::new(program, Box::new('_')).unwrap();
    }
}

#[cfg(test)]
mod copy_turing_machine {
    use super::*;

    fn new_fail_machine() -> Classic<char> {
        let mut program = Program::new(vec!['0', '1'], 3);
        program.extend_by([
            (1, '0', 2, '0', Direction::Right),
            (1, '1', 1, '1', Direction::Left),
            (3, '0', 0, '0', Direction::Center),
            (3, '1', 3, '0', Direction::Left),
        ]);

        Classic::new(program, '0').unwrap()
    }

    fn new_success_machine() -> Classic<char> {
        let mut program = Program::new(vec!['0', '1'], 3);
        program.extend_by([
            (1, '0', 2, '0', Direction::Right),
            (1, '1', 1, '1', Direction::Left),
            (2, '0', 3, '1', Direction::Left),
            (2, '1', 2, '1', Direction::Right),
            (3, '0', 0, '0', Direction::Center),
            (3, '1', 3, '0', Direction::Left),
        ]);

        Classic::new(program, '0').unwrap()
    }

    #[test]
    fn execute() {
        let mut program = Program::new(vec![' ', '0', '1'], 2);
        program.extend_by([
            (1, ' ', 2, ' ', Direction::Right),
            (1, '0', 1, '1', Direction::Left),
            (1, '1', 1, '0', Direction::Left),
            (2, ' ', 0, ' ', Direction::Left),
            (2, '0', 2, '0', Direction::Right),
            (2, '1', 2, '1', Direction::Right),
        ]);
        let machine = Classic::new(program, ' ').unwrap();

        let conf = Configuration::new(Tape::from("001100"), 5, 0).unwrap();
        let result = machine.execute(conf.clone());

        let expected = conf;

        assert_eq!(expected, result);

        let conf = Configuration::new_std(Tape::from("001100")).unwrap();
        let result = machine.execute(conf);

        let expected = Configuration::new(Tape::from(" 110011 "), 6, 0).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn success_execute_once() {
        let mut program = Program::new(vec![' ', '1'], 1);
        let _ = program.insert(Instruction::new(
            Head::new(1, '1'),
            Tail::new(1, '1', Direction::Right),
        ));

        let machine = Classic::new(program, ' ').unwrap();

        let conf = Configuration::new_std(Tape::from("1")).unwrap();
        let result = machine.execute_once(conf);

        let expected = Configuration::new(Tape::from("1 "), 1, 1).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    #[should_panic]
    fn fail_execute_once() {
        let mut program = Program::new(vec![' ', '1'], 1);
        let _ = program.insert(Instruction::new(
            Head::new(1, '1'),
            Tail::new(1, '1', Direction::Right),
        ));

        let machine = Classic::new(program, ' ').unwrap();

        let conf = Configuration::new_std(Tape::from(" ")).unwrap();
        let _ = machine.execute_once(conf);
    }

    #[test]
    fn success_execute_until() {
        let machine = new_success_machine();

        let conf = Configuration::new_std(Tape::from("010")).unwrap();
        let result = machine.execute_until(conf, |conf| conf.state == 3);

        let expected = Configuration::new(Tape::from("0101"), 2, 3).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    #[should_panic]
    fn fail_execute_until() {
        let machine = new_fail_machine();

        let conf = Configuration::new_std(Tape::from("010")).unwrap();
        let _ = machine.execute_until(conf, |conf| conf.state == 3);
    }

    #[test]
    fn translate_std() {
        let machine = new_success_machine();

        let result = machine.translate_std(Tape::from("010"));
        let expected = Tape::from("0101");
        assert_eq!(expected, result);
    }

    #[test]
    fn translate_nrm() {
        let machine = new_success_machine();

        let result = machine.translate_nrm(Tape::from("010"));
        let expected = Tape::from("001");
        assert_eq!(expected, result);
    }
}

#[cfg(test)]
mod clone_turing_machine {
    use super::*;

    fn new_fail_machine() -> Classic<Box<char>> {
        let mut program = Program::new(vec![Box::new('0'), Box::new('1')], 3);
        program.extend_by([
            (1, Box::new('0'), 2, Box::new('0'), Direction::Right),
            (1, Box::new('1'), 1, Box::new('1'), Direction::Left),
            (3, Box::new('0'), 0, Box::new('0'), Direction::Center),
            (3, Box::new('1'), 3, Box::new('0'), Direction::Left),
        ]);

        Classic::new(program, Box::new('0')).unwrap()
    }

    fn new_success_machine() -> Classic<Box<char>> {
        let mut program = Program::new(vec![Box::new('0'), Box::new('1')], 3);
        program.extend_by([
            (1, Box::new('0'), 2, Box::new('0'), Direction::Right),
            (1, Box::new('1'), 1, Box::new('1'), Direction::Left),
            (2, Box::new('0'), 3, Box::new('1'), Direction::Left),
            (2, Box::new('1'), 2, Box::new('1'), Direction::Right),
            (3, Box::new('0'), 0, Box::new('0'), Direction::Center),
            (3, Box::new('1'), 3, Box::new('0'), Direction::Left),
        ]);

        Classic::new(program, Box::new('0')).unwrap()
    }

    #[test]
    fn execute() {
        let mut program = Program::new(vec![Box::new(' '), Box::new('0'), Box::new('1')], 2);
        program.extend_by([
            (1, Box::new(' '), 2, Box::new(' '), Direction::Right),
            (1, Box::new('0'), 1, Box::new('1'), Direction::Left),
            (1, Box::new('1'), 1, Box::new('0'), Direction::Left),
            (2, Box::new(' '), 0, Box::new(' '), Direction::Left),
            (2, Box::new('0'), 2, Box::new('0'), Direction::Right),
            (2, Box::new('1'), 2, Box::new('1'), Direction::Right),
        ]);
        let machine = Classic::new(program, Box::new(' ')).unwrap();

        let conf =
            Configuration::new(Tape::new("001100".chars().map(|ch| Box::new(ch))), 5, 0).unwrap();
        let result = machine.execute(conf.clone());

        let expected = conf;

        assert_eq!(expected, result);

        let conf =
            Configuration::new_std(Tape::new("001100".chars().map(|ch| Box::new(ch)))).unwrap();
        let result = machine.execute(conf);

        let expected =
            Configuration::new(Tape::new(" 110011 ".chars().map(|ch| Box::new(ch))), 6, 0).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn success_execute_once() {
        let mut program = Program::new(vec![Box::new(' '), Box::new('1')], 1);
        let _ = program.insert(Instruction::new(
            Head::new(1, Box::new('1')),
            Tail::new(1, Box::new('1'), Direction::Right),
        ));

        let machine = Classic::new(program, Box::new(' ')).unwrap();

        let conf = Configuration::new_std(Tape::new([Box::new('1')])).unwrap();
        let result = machine.execute_once(conf);

        let expected = Configuration::new(Tape::new([Box::new('1'), Box::new(' ')]), 1, 1).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    #[should_panic]
    fn fail_execute_once() {
        let mut program = Program::new(vec![Box::new(' '), Box::new('1')], 1);
        let _ = program.insert(Instruction::new(
            Head::new(1, Box::new('1')),
            Tail::new(1, Box::new('1'), Direction::Right),
        ));

        let machine = Classic::new(program, Box::new(' ')).unwrap();

        let conf = Configuration::new_std(Tape::new([Box::new(' ')])).unwrap();
        let _ = machine.execute_once(conf);
    }

    #[test]
    fn success_execute_until() {
        let machine = new_success_machine();

        let conf = Configuration::new_std(Tape::new("010".chars().map(|ch| Box::new(ch)))).unwrap();
        let result = machine.execute_until(conf, |conf| conf.state == 3);

        let expected =
            Configuration::new(Tape::new("0101".chars().map(|ch| Box::new(ch))), 2, 3).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    #[should_panic]
    fn fail_execute_until() {
        let machine = new_fail_machine();

        let conf = Configuration::new_std(Tape::new("010".chars().map(|ch| Box::new(ch)))).unwrap();
        let _ = machine.execute_until(conf, |conf| conf.state == 3);
    }

    #[test]
    fn translate_std() {
        let machine = new_success_machine();

        let result = machine.translate_std(Tape::new("010".chars().map(|ch| Box::new(ch))));
        let expected = Tape::new("0101".chars().map(|ch| Box::new(ch)));
        assert_eq!(expected, result);
    }

    #[test]
    fn translate_nrm() {
        let machine = new_success_machine();

        let result = machine.translate_nrm(Tape::new("010".chars().map(|ch| Box::new(ch))));
        let expected = Tape::new("001".chars().map(|ch| Box::new(ch)));
        assert_eq!(expected, result);
    }
}

#[cfg(test)]
mod copy_with_for_classic {
    use super::*;

    fn new_zerofy_machine() -> Classic<char> {
        let mut program = Program::new(vec!['0', '1'], 4);
        program.extend_by([
            (1, '0', 2, '0', Direction::Right),
            (2, '0', 3, '0', Direction::Left),
            (2, '1', 2, '1', Direction::Right),
            (3, '0', 0, '0', Direction::Center),
            (3, '1', 4, '0', Direction::Center),
            (4, '0', 3, '0', Direction::Left),
        ]);
        Classic::new(program, '0').unwrap()
    }

    fn new_left_shift_machine() -> Classic<char> {
        let mut program = Program::new(vec!['0', '1'], 2);
        program.extend_by([
            (1, '0', 2, '0', Direction::Left),
            (2, '0', 0, '0', Direction::Center),
            (2, '1', 2, '1', Direction::Left),
        ]);
        Classic::new(program, '0').unwrap()
    }

    fn new_right_shift_machine() -> Classic<char> {
        let mut program = Program::new(vec!['0', '1'], 2);
        program.extend_by([
            (1, '0', 2, '0', Direction::Right),
            (2, '0', 0, '0', Direction::Center),
            (2, '1', 2, '1', Direction::Right),
        ]);
        Classic::new(program, '0').unwrap()
    }

    fn new_trans_machine() -> Classic<char> {
        let mut program = Program::new(vec!['0', '1'], 19);
        program.extend_by([
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
        let result = choose_machine.translate_nrm(tape);

        let expected = Tape::from("0110000000");
        assert_eq!(expected, result);

        let tape = Tape::from("010111010");
        let result = choose_machine.translate_nrm(tape);

        let expected = Tape::from("011100000");

        assert_eq!(expected, result);
    }

    #[test]
    #[should_panic]
    fn self_with_other_fail() {
        let zero_one = Classic::new(Program::new(vec!['0', '1'], 2), '0').unwrap();
        let zero_one_two = Classic::new(Program::new(vec!['0', '1', '2'], 2), '0').unwrap();

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
        let result = choose_machine.translate_nrm(tape);

        let expected = Tape::from("0110000000");

        assert_eq!(expected, result);

        let tape = Tape::from("010111010");
        let result = choose_machine.translate_nrm(tape);

        let expected = Tape::from("011100000");

        assert_eq!(expected, result);
    }

    #[test]
    #[should_panic]
    fn option_with_other_fail() {
        let zero_one = Classic::new(Program::new(vec!['0', '1'], 2), '0');
        let zero_one_two = Classic::new(Program::new(vec!['0', '1', '2'], 2), '0').unwrap();

        zero_one.with(&zero_one_two).unwrap();
    }
}

#[cfg(test)]
mod clone_with_for_classic {
    use super::*;

    fn new_zerofy_machine() -> Classic<Box<char>> {
        let mut program = Program::new(vec![Box::new('0'), Box::new('1')], 4);
        program.extend_by([
            (1, Box::new('0'), 2, Box::new('0'), Direction::Right),
            (2, Box::new('0'), 3, Box::new('0'), Direction::Left),
            (2, Box::new('1'), 2, Box::new('1'), Direction::Right),
            (3, Box::new('0'), 0, Box::new('0'), Direction::Center),
            (3, Box::new('1'), 4, Box::new('0'), Direction::Center),
            (4, Box::new('0'), 3, Box::new('0'), Direction::Left),
        ]);
        Classic::new(program, Box::new('0')).unwrap()
    }

    fn new_left_shift_machine() -> Classic<Box<char>> {
        let mut program = Program::new(vec![Box::new('0'), Box::new('1')], 2);
        program.extend_by([
            (1, Box::new('0'), 2, Box::new('0'), Direction::Left),
            (2, Box::new('0'), 0, Box::new('0'), Direction::Center),
            (2, Box::new('1'), 2, Box::new('1'), Direction::Left),
        ]);
        Classic::new(program, Box::new('0')).unwrap()
    }

    fn new_right_shift_machine() -> Classic<Box<char>> {
        let mut program = Program::new(vec![Box::new('0'), Box::new('1')], 2);
        program.extend_by([
            (1, Box::new('0'), 2, Box::new('0'), Direction::Right),
            (2, Box::new('0'), 0, Box::new('0'), Direction::Center),
            (2, Box::new('1'), 2, Box::new('1'), Direction::Right),
        ]);
        Classic::new(program, Box::new('0')).unwrap()
    }

    fn new_trans_machine() -> Classic<Box<char>> {
        let mut program = Program::new(vec![Box::new('0'), Box::new('1')], 19);
        program.extend_by([
            (1, Box::new('0'), 2, Box::new('0'), Direction::Right),
            (2, Box::new('0'), 3, Box::new('0'), Direction::Center),
            (2, Box::new('1'), 2, Box::new('1'), Direction::Right),
            (3, Box::new('0'), 4, Box::new('0'), Direction::Left),
            (4, Box::new('0'), 7, Box::new('0'), Direction::Center),
            (4, Box::new('1'), 5, Box::new('0'), Direction::Center),
            (5, Box::new('0'), 6, Box::new('0'), Direction::Left),
            (6, Box::new('0'), 7, Box::new('1'), Direction::Center),
            (6, Box::new('1'), 6, Box::new('1'), Direction::Left),
            (7, Box::new('0'), 16, Box::new('1'), Direction::Center),
            (7, Box::new('1'), 8, Box::new('1'), Direction::Left),
            (8, Box::new('0'), 18, Box::new('0'), Direction::Right),
            (8, Box::new('1'), 9, Box::new('0'), Direction::Center),
            (9, Box::new('0'), 10, Box::new('0'), Direction::Right),
            (10, Box::new('0'), 11, Box::new('1'), Direction::Center),
            (10, Box::new('1'), 10, Box::new('1'), Direction::Right),
            (11, Box::new('1'), 12, Box::new('1'), Direction::Left),
            (12, Box::new('1'), 13, Box::new('0'), Direction::Center),
            (13, Box::new('0'), 14, Box::new('0'), Direction::Left),
            (14, Box::new('0'), 15, Box::new('1'), Direction::Center),
            (14, Box::new('1'), 14, Box::new('1'), Direction::Left),
            (15, Box::new('0'), 7, Box::new('0'), Direction::Center),
            (15, Box::new('1'), 7, Box::new('1'), Direction::Center),
            (16, Box::new('1'), 17, Box::new('1'), Direction::Left),
            (17, Box::new('0'), 19, Box::new('0'), Direction::Right),
            (17, Box::new('1'), 15, Box::new('0'), Direction::Center),
            (18, Box::new('0'), 0, Box::new('0'), Direction::Center),
            (18, Box::new('1'), 18, Box::new('1'), Direction::Right),
            (19, Box::new('1'), 0, Box::new('0'), Direction::Center),
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
        let result = choose_machine.translate_nrm(tape);

        let expected = Tape::new("0110000000".chars().map(|ch| Box::new(ch)));
        assert_eq!(expected, result);

        let tape = Tape::new("010111010".chars().map(|ch| Box::new(ch)));
        let result = choose_machine.translate_nrm(tape);

        let expected = Tape::new("011100000".chars().map(|ch| Box::new(ch)));

        assert_eq!(expected, result);
    }

    #[test]
    #[should_panic]
    fn self_with_other_fail() {
        let zero_one = Classic::new(
            Program::new(vec![Box::new('0'), Box::new('1')], 2),
            Box::new('0'),
        )
        .unwrap();
        let zero_one_two = Classic::new(
            Program::new(vec![Box::new('0'), Box::new('1'), Box::new('2')], 2),
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
        let result = choose_machine.translate_nrm(tape);

        let expected = Tape::new("0110000000".chars().map(|ch| Box::new(ch)));

        assert_eq!(expected, result);

        let tape = Tape::new("010111010".chars().map(|ch| Box::new(ch)));
        let result = choose_machine.translate_nrm(tape);

        let expected = Tape::new("011100000".chars().map(|ch| Box::new(ch)));

        assert_eq!(expected, result);
    }

    #[test]
    #[should_panic]
    fn option_with_other_fail() {
        let zero_one = Classic::new(
            Program::new(vec![Box::new('0'), Box::new('1')], 2),
            Box::new('0'),
        );
        let zero_one_two = Classic::new(
            Program::new(vec![Box::new('0'), Box::new('1'), Box::new('2')], 2),
            Box::new('0'),
        )
        .unwrap();

        zero_one.with(&zero_one_two).unwrap();
    }
}
