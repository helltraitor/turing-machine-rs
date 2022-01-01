use turing_machine_rs::instruction::{Direction, Head, Instruction, Tail};
use turing_machine_rs::program::{ExtendBy, Program};

#[cfg(test)]
mod copy {
    use super::*;

    #[test]
    fn success_creation() {
        let _ = Program::new(vec![' '], 1);
    }

    #[test]
    #[should_panic]
    fn fail_creation_empty_alphabet() {
        let _: Program<char> = Program::new(vec![], 1);
    }

    #[test]
    #[should_panic]
    fn fail_creation_zero_l_state() {
        let _ = Program::new(vec![' '], 0);
    }

    #[test]
    fn alphabet() {
        let program = Program::new(vec![' '], 1);
        assert_eq!(program.alphabet(), &vec![' ']);
    }

    #[test]
    fn success_extend() {
        let mut origin = Program::new(vec!['0', '1'], 1);
        origin.set(Instruction::new(
            Head::new(1, '0'),
            Tail::new(1, '0', Direction::Right),
        ));
        origin.set(Instruction::new(
            Head::new(1, '1'),
            Tail::new(0, '0', Direction::Right),
        ));

        let mut extension = Program::new(vec!['0', '1'], 2);
        extension.set(Instruction::new(
            Head::new(1, '0'),
            Tail::new(0, '0', Direction::Left),
        ));
        extension.set(Instruction::new(
            Head::new(1, '1'),
            Tail::new(2, '1', Direction::Right),
        ));
        extension.set(Instruction::new(
            Head::new(2, '0'),
            Tail::new(0, '0', Direction::Left),
        ));
        extension.set(Instruction::new(
            Head::new(2, '1'),
            Tail::new(2, '1', Direction::Right),
        ));

        let mut result = origin;
        result.extend(&extension);

        let mut expected = Program::new(vec!['0', '1'], 3);
        expected.set(Instruction::new(
            Head::new(1, '0'),
            Tail::new(1, '0', Direction::Right),
        ));
        expected.set(Instruction::new(
            Head::new(1, '1'),
            Tail::new(2, '0', Direction::Right),
        ));
        expected.set(Instruction::new(
            Head::new(2, '0'),
            Tail::new(0, '0', Direction::Left),
        ));
        expected.set(Instruction::new(
            Head::new(2, '1'),
            Tail::new(3, '1', Direction::Right),
        ));
        expected.set(Instruction::new(
            Head::new(3, '0'),
            Tail::new(0, '0', Direction::Left),
        ));
        expected.set(Instruction::new(
            Head::new(3, '1'),
            Tail::new(3, '1', Direction::Right),
        ));

        assert_eq!(expected, result);
    }

    #[test]
    #[should_panic]
    fn fail_extend_different_alphabet() {
        let extension = Program::new(vec!['1', '2'], 2);

        let mut origin = Program::new(vec!['0', '1'], 1);
        origin.extend(&extension);
    }

    #[test]
    #[should_panic]
    fn fail_extend_not_enough_capacity() {
        let extension = Program::new(vec!['0', '1'], 2);

        let mut origin = Program::new(vec!['0', '1'], 1);
        origin.extend(&extension);
    }

    #[test]
    fn success_get() {
        let mut origin = Program::new(vec!['0', '1'], 1);
        origin.set(Instruction::new(
            Head::new(1, '0'),
            Tail::new(1, '0', Direction::Right),
        ));

        let result = origin.get(&Head::new(1, '0'));

        let e_value = Instruction::new(Head::new(1, '0'), Tail::new(1, '0', Direction::Right));
        let expected = Some(&e_value);
        assert_eq!(expected, result);

        let result = origin.get(&Head::new(1, '1'));
        assert_eq!(None, result);
    }

    #[test]
    #[should_panic]
    fn fail_get() {
        let origin = Program::new(vec!['0', '1'], 1);
        let _ = origin.get(&Head::new(2, '0'));
    }

    #[test]
    fn l_state() {
        let origin = Program::new(vec!['0', '1'], 1);
        assert_eq!(1, origin.l_state());
    }

    #[test]
    fn success_set() {
        let mut origin = Program::new(vec!['0', '1'], 1);
        origin.set(Instruction::new(
            Head::new(1, '0'),
            Tail::new(1, '0', Direction::Right),
        ));
    }

    #[test]
    #[should_panic]
    fn fail_set_zero_head_state() {
        let mut origin = Program::new(vec!['0', '1'], 1);
        origin.set(Instruction::new(
            Head::new(0, '0'),
            Tail::new(1, '0', Direction::Right),
        ));
    }

    #[test]
    #[should_panic]
    fn fail_set_head_symbol_miss() {
        let mut origin = Program::new(vec!['0', '1'], 1);
        origin.set(Instruction::new(
            Head::new(1, '9'),
            Tail::new(1, '0', Direction::Right),
        ));
    }

    #[test]
    #[should_panic]
    fn fail_set_tail_symbol_miss() {
        let mut origin = Program::new(vec!['0', '1'], 1);
        origin.set(Instruction::new(
            Head::new(1, '0'),
            Tail::new(1, '9', Direction::Right),
        ));
    }

    #[test]
    #[should_panic]
    fn fail_set_head_l_state_large() {
        let mut origin = Program::new(vec!['0', '1'], 1);
        origin.set(Instruction::new(
            Head::new(2, '0'),
            Tail::new(1, '0', Direction::Right),
        ));
    }

    #[test]
    #[should_panic]
    fn fail_set_tail_l_state_large() {
        let mut origin = Program::new(vec!['0', '1'], 1);
        origin.set(Instruction::new(
            Head::new(1, '0'),
            Tail::new(2, '0', Direction::Right),
        ));
    }
}

#[cfg(test)]
mod copy_extend_by {
    use super::*;

    #[test]
    fn extend_by() {
        let mut expected = Program::new(vec!['0', '1'], 1);
        expected.set(Instruction::new(
            Head::new(1, '0'),
            Tail::new(1, '0', Direction::Right),
        ));
        expected.set(Instruction::new(
            Head::new(1, '1'),
            Tail::new(0, '0', Direction::Right),
        ));

        let mut program = Program::new(vec!['0', '1'], 1);
        program.extend_by([
            (1, '0', 1, '0', Direction::Right),
            (1, '1', 0, '0', Direction::Right),
        ]);

        assert_eq!(expected, program);
    }
}

#[cfg(test)]
mod clone {
    use super::*;

    #[test]
    fn success_creation() {
        let _ = Program::new(vec![Box::new(' ')], 1);
    }

    #[test]
    #[should_panic]
    fn fail_creation_empty_alphabet() {
        let _: Program<Box<char>> = Program::new(vec![], 1);
    }

    #[test]
    #[should_panic]
    fn fail_creation_zero_l_state() {
        let _ = Program::new(vec![Box::new(' ')], 0);
    }

    #[test]
    fn alphabet() {
        let program = Program::new(vec![Box::new(' ')], 1);
        assert_eq!(program.alphabet(), &vec![Box::new(' ')]);
    }

    #[test]
    fn success_extend() {
        let mut origin = Program::new(vec![Box::new('0'), Box::new('1')], 1);
        origin.set(Instruction::new(
            Head::new(1, Box::new('0')),
            Tail::new(1, Box::new('0'), Direction::Right),
        ));
        origin.set(Instruction::new(
            Head::new(1, Box::new('1')),
            Tail::new(0, Box::new('0'), Direction::Right),
        ));

        let mut extension = Program::new(vec![Box::new('0'), Box::new('1')], 2);
        extension.set(Instruction::new(
            Head::new(1, Box::new('0')),
            Tail::new(0, Box::new('0'), Direction::Left),
        ));
        extension.set(Instruction::new(
            Head::new(1, Box::new('1')),
            Tail::new(2, Box::new('1'), Direction::Right),
        ));
        extension.set(Instruction::new(
            Head::new(2, Box::new('0')),
            Tail::new(0, Box::new('0'), Direction::Left),
        ));
        extension.set(Instruction::new(
            Head::new(2, Box::new('1')),
            Tail::new(2, Box::new('1'), Direction::Right),
        ));

        let mut result = origin;
        result.extend(&extension);

        let mut expected = Program::new(vec![Box::new('0'), Box::new('1')], 3);
        expected.set(Instruction::new(
            Head::new(1, Box::new('0')),
            Tail::new(1, Box::new('0'), Direction::Right),
        ));
        expected.set(Instruction::new(
            Head::new(1, Box::new('1')),
            Tail::new(2, Box::new('0'), Direction::Right),
        ));
        expected.set(Instruction::new(
            Head::new(2, Box::new('0')),
            Tail::new(0, Box::new('0'), Direction::Left),
        ));
        expected.set(Instruction::new(
            Head::new(2, Box::new('1')),
            Tail::new(3, Box::new('1'), Direction::Right),
        ));
        expected.set(Instruction::new(
            Head::new(3, Box::new('0')),
            Tail::new(0, Box::new('0'), Direction::Left),
        ));
        expected.set(Instruction::new(
            Head::new(3, Box::new('1')),
            Tail::new(3, Box::new('1'), Direction::Right),
        ));

        assert_eq!(expected, result);
    }

    #[test]
    #[should_panic]
    fn fail_extend_different_alphabet() {
        let extension = Program::new(vec![Box::new('1'), Box::new('2')], 2);

        let mut origin = Program::new(vec![Box::new('0'), Box::new('1')], 1);
        origin.extend(&extension);
    }

    #[test]
    #[should_panic]
    fn fail_extend_not_enough_capacity() {
        let extension = Program::new(vec![Box::new('0'), Box::new('1')], 2);

        let mut origin = Program::new(vec![Box::new('0'), Box::new('1')], 1);
        origin.extend(&extension);
    }

    #[test]
    fn success_get() {
        let mut origin = Program::new(vec![Box::new('0'), Box::new('1')], 1);
        origin.set(Instruction::new(
            Head::new(1, Box::new('0')),
            Tail::new(1, Box::new('0'), Direction::Right),
        ));

        let result = origin.get(&Head::new(1, Box::new('0')));

        let e_value = Instruction::new(
            Head::new(1, Box::new('0')),
            Tail::new(1, Box::new('0'), Direction::Right),
        );
        let expected = Some(&e_value);
        assert_eq!(expected, result);

        let result = origin.get(&Head::new(1, Box::new('1')));
        assert_eq!(None, result);
    }

    #[test]
    #[should_panic]
    fn fail_get() {
        let origin = Program::new(vec![Box::new('0'), Box::new('1')], 1);
        let _ = origin.get(&Head::new(2, Box::new('0')));
    }

    #[test]
    fn l_state() {
        let origin = Program::new(vec![Box::new('0'), Box::new('1')], 1);
        assert_eq!(1, origin.l_state());
    }

    #[test]
    fn success_set() {
        let mut origin = Program::new(vec![Box::new('0'), Box::new('1')], 1);
        origin.set(Instruction::new(
            Head::new(1, Box::new('0')),
            Tail::new(1, Box::new('0'), Direction::Right),
        ));
    }

    #[test]
    #[should_panic]
    fn fail_set_zero_head_state() {
        let mut origin = Program::new(vec![Box::new('0'), Box::new('1')], 1);
        origin.set(Instruction::new(
            Head::new(0, Box::new('0')),
            Tail::new(1, Box::new('0'), Direction::Right),
        ));
    }

    #[test]
    #[should_panic]
    fn fail_set_head_symbol_miss() {
        let mut origin = Program::new(vec![Box::new('0'), Box::new('1')], 1);
        origin.set(Instruction::new(
            Head::new(1, Box::new('9')),
            Tail::new(1, Box::new('0'), Direction::Right),
        ));
    }

    #[test]
    #[should_panic]
    fn fail_set_tail_symbol_miss() {
        let mut origin = Program::new(vec![Box::new('0'), Box::new('1')], 1);
        origin.set(Instruction::new(
            Head::new(1, Box::new('0')),
            Tail::new(1, Box::new('9'), Direction::Right),
        ));
    }

    #[test]
    #[should_panic]
    fn fail_set_head_l_state_large() {
        let mut origin = Program::new(vec![Box::new('0'), Box::new('1')], 1);
        origin.set(Instruction::new(
            Head::new(2, Box::new('0')),
            Tail::new(1, Box::new('0'), Direction::Right),
        ));
    }

    #[test]
    #[should_panic]
    fn fail_set_tail_l_state_large() {
        let mut origin = Program::new(vec![Box::new('0'), Box::new('1')], 1);
        origin.set(Instruction::new(
            Head::new(1, Box::new('0')),
            Tail::new(2, Box::new('0'), Direction::Right),
        ));
    }
}

#[cfg(test)]
mod clone_extend_by {
    use super::*;

    #[test]
    fn extend_by() {
        let mut expected = Program::new(vec![Box::new('0'), Box::new('1')], 1);
        expected.set(Instruction::new(
            Head::new(1, Box::new('0')),
            Tail::new(1, Box::new('0'), Direction::Right),
        ));
        expected.set(Instruction::new(
            Head::new(1, Box::new('1')),
            Tail::new(0, Box::new('0'), Direction::Right),
        ));

        let mut program = Program::new(vec![Box::new('0'), Box::new('1')], 1);
        program.extend_by([
            (1, Box::new('0'), 1, Box::new('0'), Direction::Right),
            (1, Box::new('1'), 0, Box::new('0'), Direction::Right),
        ]);

        assert_eq!(expected, program);
    }
}
