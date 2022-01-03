use turing_machine_rs::instruction::{Direction, Head, Instruction, Tail};
use turing_machine_rs::program::{ExtendBy, Program};
use turing_machine_rs::With;

#[cfg(test)]
mod copy {
    use super::*;

    #[test]
    fn creation() {
        let _ = Program::new(vec![' '], 1);
    }

    #[test]
    fn alphabet() {
        let program = Program::new(vec![' '], 1);
        assert_eq!(program.alphabet(), &vec![' ']);
    }

    #[test]
    #[should_panic]
    fn fail_extend_different_alphabet() {
        let origin = Program::new(vec!['0', '1'], 1);
        let extension = Program::new(vec!['1', '2'], 2);
        origin.with(&extension).unwrap();
    }

    #[test]
    fn success_get() {
        let mut origin = Program::new(vec!['0', '1'], 1);
        let _ = origin.insert(Instruction::new(
            Head::new(1, '0'),
            Tail::new(1, '0', Direction::Right),
        ));

        let result = origin.get(&Head::new(1, '0')).unwrap().unwrap();
        let expected = Instruction::new(Head::new(1, '0'), Tail::new(1, '0', Direction::Right));
        assert_eq!(&expected, result);

        let result = origin.get(&Head::new(1, '1')).unwrap();
        assert_eq!(None, result);
    }

    #[test]
    #[should_panic]
    fn fail_get() {
        let origin = Program::new(vec!['0', '1'], 1);
        origin.get(&Head::new(2, '0')).unwrap();
    }

    #[test]
    fn l_state() {
        let origin = Program::new(vec!['0', '1'], 1);
        assert_eq!(1, origin.l_state());
    }

    #[test]
    fn success_insert() {
        let mut origin = Program::new(vec!['0', '1'], 1);
        origin
            .insert(Instruction::new(
                Head::new(1, '0'),
                Tail::new(1, '0', Direction::Right),
            ))
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn fail_insert_zero_head_state() {
        let mut origin = Program::new(vec!['0', '1'], 1);
        origin
            .insert(Instruction::new(
                Head::new(0, '0'),
                Tail::new(1, '0', Direction::Right),
            ))
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn fail_insert_head_symbol_miss() {
        let mut origin = Program::new(vec!['0', '1'], 1);
        origin
            .insert(Instruction::new(
                Head::new(1, '9'),
                Tail::new(1, '0', Direction::Right),
            ))
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn fail_insert_tail_symbol_miss() {
        let mut origin = Program::new(vec!['0', '1'], 1);
        origin
            .insert(Instruction::new(
                Head::new(1, '0'),
                Tail::new(1, '9', Direction::Right),
            ))
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn fail_insert_head_l_state_large() {
        let mut origin = Program::new(vec!['0', '1'], 1);
        origin
            .insert(Instruction::new(
                Head::new(2, '0'),
                Tail::new(1, '0', Direction::Right),
            ))
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn fail_insert_tail_l_state_large() {
        let mut origin = Program::new(vec!['0', '1'], 1);
        origin
            .insert(Instruction::new(
                Head::new(1, '0'),
                Tail::new(2, '0', Direction::Right),
            ))
            .unwrap();
    }
}

#[cfg(test)]
mod copy_extend_by {
    use super::*;

    #[test]
    fn extend_by() {
        let mut expected = Program::new(vec!['0', '1'], 1);
        let _ = expected.insert(Instruction::new(
            Head::new(1, '0'),
            Tail::new(1, '0', Direction::Right),
        ));
        let _ = expected.insert(Instruction::new(
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
mod copy_with {
    use super::*;

    #[test]
    fn success_with() {
        let mut origin = Program::new(vec!['0', '1'], 1);
        let _ = origin.insert(Instruction::new(
            Head::new(1, '0'),
            Tail::new(1, '0', Direction::Right),
        ));
        let _ = origin.insert(Instruction::new(
            Head::new(1, '1'),
            Tail::new(0, '0', Direction::Right),
        ));

        let mut extension = Program::new(vec!['0', '1'], 2);
        let _ = extension.insert(Instruction::new(
            Head::new(1, '0'),
            Tail::new(0, '0', Direction::Left),
        ));
        let _ = extension.insert(Instruction::new(
            Head::new(1, '1'),
            Tail::new(2, '1', Direction::Right),
        ));
        let _ = extension.insert(Instruction::new(
            Head::new(2, '0'),
            Tail::new(0, '0', Direction::Left),
        ));
        let _ = extension.insert(Instruction::new(
            Head::new(2, '1'),
            Tail::new(2, '1', Direction::Right),
        ));

        let result = origin.with(&extension).unwrap();

        let mut expected = Program::new(vec!['0', '1'], 3);
        let _ = expected.insert(Instruction::new(
            Head::new(1, '0'),
            Tail::new(1, '0', Direction::Right),
        ));
        let _ = expected.insert(Instruction::new(
            Head::new(1, '1'),
            Tail::new(2, '0', Direction::Right),
        ));
        let _ = expected.insert(Instruction::new(
            Head::new(2, '0'),
            Tail::new(0, '0', Direction::Left),
        ));
        let _ = expected.insert(Instruction::new(
            Head::new(2, '1'),
            Tail::new(3, '1', Direction::Right),
        ));
        let _ = expected.insert(Instruction::new(
            Head::new(3, '0'),
            Tail::new(0, '0', Direction::Left),
        ));
        let _ = expected.insert(Instruction::new(
            Head::new(3, '1'),
            Tail::new(3, '1', Direction::Right),
        ));

        assert_eq!(expected, result);
    }

    #[test]
    #[should_panic]
    fn fail_with_alphabet() {
        let mut origin = Program::new(vec!['0'], 1);
        let _ = origin.insert(Instruction::new(
            Head::new(1, '0'),
            Tail::new(1, '0', Direction::Right),
        ));

        let mut extension = Program::new(vec!['0', '1'], 2);
        let _ = extension.insert(Instruction::new(
            Head::new(1, '0'),
            Tail::new(0, '0', Direction::Left),
        ));
        let _ = extension.insert(Instruction::new(
            Head::new(1, '1'),
            Tail::new(2, '1', Direction::Right),
        ));

        origin.with(&extension).unwrap();
    }
}

#[cfg(test)]
mod clone {
    use super::*;

    #[test]
    fn creation() {
        let _ = Program::new(vec![Box::new(' ')], 1);
    }

    #[test]
    fn alphabet() {
        let program = Program::new(vec![Box::new(' ')], 1);
        assert_eq!(program.alphabet(), &vec![Box::new(' ')]);
    }

    #[test]
    fn success_get() {
        let mut origin = Program::new(vec![Box::new('0'), Box::new('1')], 1);
        let _ = origin.insert(Instruction::new(
            Head::new(1, Box::new('0')),
            Tail::new(1, Box::new('0'), Direction::Right),
        ));

        let result = origin.get(&Head::new(1, Box::new('0'))).unwrap();

        let e_value = Instruction::new(
            Head::new(1, Box::new('0')),
            Tail::new(1, Box::new('0'), Direction::Right),
        );
        let expected = Some(&e_value);
        assert_eq!(expected, result);

        let result = origin.get(&Head::new(1, Box::new('1'))).unwrap();
        assert_eq!(None, result);
    }

    #[test]
    #[should_panic]
    fn fail_get() {
        let origin = Program::new(vec![Box::new('0'), Box::new('1')], 1);
        origin.get(&Head::new(2, Box::new('0'))).unwrap();
    }

    #[test]
    fn l_state() {
        let origin = Program::new(vec![Box::new('0'), Box::new('1')], 1);
        assert_eq!(1, origin.l_state());
    }

    #[test]
    fn success_insert() {
        let mut origin = Program::new(vec![Box::new('0'), Box::new('1')], 1);
        origin
            .insert(Instruction::new(
                Head::new(1, Box::new('0')),
                Tail::new(1, Box::new('0'), Direction::Right),
            ))
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn fail_set_zero_head_state() {
        let mut origin = Program::new(vec![Box::new('0'), Box::new('1')], 1);
        origin
            .insert(Instruction::new(
                Head::new(0, Box::new('0')),
                Tail::new(1, Box::new('0'), Direction::Right),
            ))
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn fail_set_head_symbol_miss() {
        let mut origin = Program::new(vec![Box::new('0'), Box::new('1')], 1);
        origin
            .insert(Instruction::new(
                Head::new(1, Box::new('9')),
                Tail::new(1, Box::new('0'), Direction::Right),
            ))
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn fail_set_tail_symbol_miss() {
        let mut origin = Program::new(vec![Box::new('0'), Box::new('1')], 1);
        origin
            .insert(Instruction::new(
                Head::new(1, Box::new('0')),
                Tail::new(1, Box::new('9'), Direction::Right),
            ))
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn fail_set_head_l_state_large() {
        let mut origin = Program::new(vec![Box::new('0'), Box::new('1')], 1);
        origin
            .insert(Instruction::new(
                Head::new(2, Box::new('0')),
                Tail::new(1, Box::new('0'), Direction::Right),
            ))
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn fail_set_tail_l_state_large() {
        let mut origin = Program::new(vec![Box::new('0'), Box::new('1')], 1);
        origin
            .insert(Instruction::new(
                Head::new(1, Box::new('0')),
                Tail::new(2, Box::new('0'), Direction::Right),
            ))
            .unwrap();
    }
}

#[cfg(test)]
mod clone_extend_by {
    use super::*;

    #[test]
    fn extend_by() {
        let mut expected = Program::new(vec![Box::new('0'), Box::new('1')], 1);
        let _ = expected.insert(Instruction::new(
            Head::new(1, Box::new('0')),
            Tail::new(1, Box::new('0'), Direction::Right),
        ));
        let _ = expected.insert(Instruction::new(
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

#[cfg(test)]
mod clone_with {
    use super::*;

    #[test]
    fn success_with() {
        let mut origin = Program::new(vec![Box::new('0'), Box::new('1')], 1);
        let _ = origin.insert(Instruction::new(
            Head::new(1, Box::new('0')),
            Tail::new(1, Box::new('0'), Direction::Right),
        ));
        let _ = origin.insert(Instruction::new(
            Head::new(1, Box::new('1')),
            Tail::new(0, Box::new('0'), Direction::Right),
        ));

        let mut extension = Program::new(vec![Box::new('0'), Box::new('1')], 2);
        let _ = extension.insert(Instruction::new(
            Head::new(1, Box::new('0')),
            Tail::new(0, Box::new('0'), Direction::Left),
        ));
        let _ = extension.insert(Instruction::new(
            Head::new(1, Box::new('1')),
            Tail::new(2, Box::new('1'), Direction::Right),
        ));
        let _ = extension.insert(Instruction::new(
            Head::new(2, Box::new('0')),
            Tail::new(0, Box::new('0'), Direction::Left),
        ));
        let _ = extension.insert(Instruction::new(
            Head::new(2, Box::new('1')),
            Tail::new(2, Box::new('1'), Direction::Right),
        ));

        let result = origin.with(&extension).unwrap();

        let mut expected = Program::new(vec![Box::new('0'), Box::new('1')], 3);
        let _ = expected.insert(Instruction::new(
            Head::new(1, Box::new('0')),
            Tail::new(1, Box::new('0'), Direction::Right),
        ));
        let _ = expected.insert(Instruction::new(
            Head::new(1, Box::new('1')),
            Tail::new(2, Box::new('0'), Direction::Right),
        ));
        let _ = expected.insert(Instruction::new(
            Head::new(2, Box::new('0')),
            Tail::new(0, Box::new('0'), Direction::Left),
        ));
        let _ = expected.insert(Instruction::new(
            Head::new(2, Box::new('1')),
            Tail::new(3, Box::new('1'), Direction::Right),
        ));
        let _ = expected.insert(Instruction::new(
            Head::new(3, Box::new('0')),
            Tail::new(0, Box::new('0'), Direction::Left),
        ));
        let _ = expected.insert(Instruction::new(
            Head::new(3, Box::new('1')),
            Tail::new(3, Box::new('1'), Direction::Right),
        ));

        assert_eq!(expected, result);
    }

    #[test]
    #[should_panic]
    fn fail_with_alphabet() {
        let mut origin = Program::new(vec![Box::new('0')], 1);
        let _ = origin.insert(Instruction::new(
            Head::new(1, Box::new('0')),
            Tail::new(1, Box::new('0'), Direction::Right),
        ));

        let mut extension = Program::new(vec![Box::new('0'), Box::new('1')], 2);
        let _ = extension.insert(Instruction::new(
            Head::new(1, Box::new('0')),
            Tail::new(0, Box::new('0'), Direction::Left),
        ));
        let _ = extension.insert(Instruction::new(
            Head::new(1, Box::new('1')),
            Tail::new(2, Box::new('1'), Direction::Right),
        ));

        origin.with(&extension).unwrap();
    }
}
