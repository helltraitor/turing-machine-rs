use turing_machine_rs::instruction::{Head, Instruction, Move, State, Tail};
use turing_machine_rs::program::{Extend, Program};
use turing_machine_rs::With;

#[cfg(test)]
mod copy {
    use super::*;

    #[test]
    fn creation() {
        let _ = Program::new(vec![' '], State(1));
    }

    #[test]
    fn alphabet() {
        let program = Program::new(vec![' '], State(1));
        assert_eq!(program.alphabet(), &vec![' ']);
    }

    #[test]
    #[should_panic]
    fn fail_extend_different_alphabet() {
        let origin = Program::new(vec!['0', '1'], State(1));
        let extension = Program::new(vec!['1', '2'], State(2));
        origin.with(&extension).unwrap();
    }

    #[test]
    fn success_get() {
        let mut origin = Program::new(vec!['0', '1'], State(1));
        let _ = origin.insert(Instruction::new(
            Head::new(State(1), '0'),
            Tail::new(State(1), '0', Move::Right),
        ));

        let result = origin.get(&Head::new(State(1), '0')).unwrap().unwrap();
        let expected = Instruction::new(
            Head::new(State(1), '0'),
            Tail::new(State(1), '0', Move::Right),
        );
        assert_eq!(&expected, result);

        let result = origin.get(&Head::new(State(1), '1')).unwrap();
        assert_eq!(None, result);
    }

    #[test]
    #[should_panic]
    fn fail_get() {
        let origin = Program::new(vec!['0', '1'], State(1));
        origin.get(&Head::new(State(2), '0')).unwrap();
    }

    #[test]
    fn l_state() {
        let origin = Program::new(vec!['0', '1'], State(1));
        assert_eq!(State(1), origin.l_state());
    }

    #[test]
    fn success_insert() {
        let mut origin = Program::new(vec!['0', '1'], State(1));
        origin
            .insert(Instruction::new(
                Head::new(State(1), '0'),
                Tail::new(State(1), '0', Move::Right),
            ))
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn fail_insert_zero_head_state() {
        let mut origin = Program::new(vec!['0', '1'], State(1));
        origin
            .insert(Instruction::new(
                Head::new(State(0), '0'),
                Tail::new(State(1), '0', Move::Right),
            ))
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn fail_insert_head_symbol_miss() {
        let mut origin = Program::new(vec!['0', '1'], State(1));
        origin
            .insert(Instruction::new(
                Head::new(State(1), '9'),
                Tail::new(State(1), '0', Move::Right),
            ))
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn fail_insert_tail_symbol_miss() {
        let mut origin = Program::new(vec!['0', '1'], State(1));
        origin
            .insert(Instruction::new(
                Head::new(State(1), '0'),
                Tail::new(State(1), '9', Move::Right),
            ))
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn fail_insert_head_l_state_large() {
        let mut origin = Program::new(vec!['0', '1'], State(1));
        origin
            .insert(Instruction::new(
                Head::new(State(2), '0'),
                Tail::new(State(1), '0', Move::Right),
            ))
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn fail_insert_tail_l_state_large() {
        let mut origin = Program::new(vec!['0', '1'], State(1));
        origin
            .insert(Instruction::new(
                Head::new(State(1), '0'),
                Tail::new(State(2), '0', Move::Right),
            ))
            .unwrap();
    }
}

#[cfg(test)]
mod copy_extend {
    use super::*;

    #[test]
    fn extend() {
        let mut expected = Program::new(vec!['0', '1'], State(1));
        let _ = expected.insert(Instruction::new(
            Head::new(State(1), '0'),
            Tail::new(State(1), '0', Move::Right),
        ));
        let _ = expected.insert(Instruction::new(
            Head::new(State(1), '1'),
            Tail::new(State(0), '0', Move::Right),
        ));

        let mut program = Program::new(vec!['0', '1'], State(1));
        program
            .extend([(1, '0', 1, '0', Move::Right), (1, '1', 0, '0', Move::Right)])
            .unwrap();

        assert_eq!(expected, program);
    }
}

#[cfg(test)]
mod copy_with {
    use super::*;

    #[test]
    fn success_with() {
        let mut origin = Program::new(vec!['0', '1'], State(1));
        let _ = origin.insert(Instruction::new(
            Head::new(State(1), '0'),
            Tail::new(State(1), '0', Move::Right),
        ));
        let _ = origin.insert(Instruction::new(
            Head::new(State(1), '1'),
            Tail::new(State(0), '0', Move::Right),
        ));

        let mut extension = Program::new(vec!['0', '1'], State(2));
        let _ = extension.insert(Instruction::new(
            Head::new(State(1), '0'),
            Tail::new(State(0), '0', Move::Left),
        ));
        let _ = extension.insert(Instruction::new(
            Head::new(State(1), '1'),
            Tail::new(State(2), '1', Move::Right),
        ));
        let _ = extension.insert(Instruction::new(
            Head::new(State(2), '0'),
            Tail::new(State(0), '0', Move::Left),
        ));
        let _ = extension.insert(Instruction::new(
            Head::new(State(2), '1'),
            Tail::new(State(2), '1', Move::Right),
        ));

        let result = origin.with(&extension).unwrap();

        let mut expected = Program::new(vec!['0', '1'], State(3));
        let _ = expected.insert(Instruction::new(
            Head::new(State(1), '0'),
            Tail::new(State(1), '0', Move::Right),
        ));
        let _ = expected.insert(Instruction::new(
            Head::new(State(1), '1'),
            Tail::new(State(2), '0', Move::Right),
        ));
        let _ = expected.insert(Instruction::new(
            Head::new(State(2), '0'),
            Tail::new(State(0), '0', Move::Left),
        ));
        let _ = expected.insert(Instruction::new(
            Head::new(State(2), '1'),
            Tail::new(State(3), '1', Move::Right),
        ));
        let _ = expected.insert(Instruction::new(
            Head::new(State(3), '0'),
            Tail::new(State(0), '0', Move::Left),
        ));
        let _ = expected.insert(Instruction::new(
            Head::new(State(3), '1'),
            Tail::new(State(3), '1', Move::Right),
        ));

        assert_eq!(expected, result);
    }

    #[test]
    #[should_panic]
    fn fail_with_alphabet() {
        let mut origin = Program::new(vec!['0'], State(1));
        let _ = origin.insert(Instruction::new(
            Head::new(State(1), '0'),
            Tail::new(State(1), '0', Move::Right),
        ));

        let mut extension = Program::new(vec!['0', '1'], State(2));
        let _ = extension.insert(Instruction::new(
            Head::new(State(1), '0'),
            Tail::new(State(0), '0', Move::Left),
        ));
        let _ = extension.insert(Instruction::new(
            Head::new(State(1), '1'),
            Tail::new(State(2), '1', Move::Right),
        ));

        origin.with(&extension).unwrap();
    }
}

#[cfg(test)]
mod clone {
    use super::*;

    #[test]
    fn creation() {
        let _ = Program::new(vec![Box::new(' ')], State(1));
    }

    #[test]
    fn alphabet() {
        let program = Program::new(vec![Box::new(' ')], State(1));
        assert_eq!(program.alphabet(), &vec![Box::new(' ')]);
    }

    #[test]
    fn success_get() {
        let mut origin = Program::new(vec![Box::new('0'), Box::new('1')], State(1));
        let _ = origin.insert(Instruction::new(
            Head::new(State(1), Box::new('0')),
            Tail::new(State(1), Box::new('0'), Move::Right),
        ));

        let result = origin.get(&Head::new(State(1), Box::new('0'))).unwrap();

        let e_value = Instruction::new(
            Head::new(State(1), Box::new('0')),
            Tail::new(State(1), Box::new('0'), Move::Right),
        );
        let expected = Some(&e_value);
        assert_eq!(expected, result);

        let result = origin.get(&Head::new(State(1), Box::new('1'))).unwrap();
        assert_eq!(None, result);
    }

    #[test]
    #[should_panic]
    fn fail_get() {
        let origin = Program::new(vec![Box::new('0'), Box::new('1')], State(1));
        origin.get(&Head::new(State(2), Box::new('0'))).unwrap();
    }

    #[test]
    fn l_state() {
        let origin = Program::new(vec![Box::new('0'), Box::new('1')], State(1));
        assert_eq!(State(1), origin.l_state());
    }

    #[test]
    fn success_insert() {
        let mut origin = Program::new(vec![Box::new('0'), Box::new('1')], State(1));
        origin
            .insert(Instruction::new(
                Head::new(State(1), Box::new('0')),
                Tail::new(State(1), Box::new('0'), Move::Right),
            ))
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn fail_set_zero_head_state() {
        let mut origin = Program::new(vec![Box::new('0'), Box::new('1')], State(1));
        origin
            .insert(Instruction::new(
                Head::new(State(0), Box::new('0')),
                Tail::new(State(1), Box::new('0'), Move::Right),
            ))
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn fail_set_head_symbol_miss() {
        let mut origin = Program::new(vec![Box::new('0'), Box::new('1')], State(1));
        origin
            .insert(Instruction::new(
                Head::new(State(1), Box::new('9')),
                Tail::new(State(1), Box::new('0'), Move::Right),
            ))
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn fail_set_tail_symbol_miss() {
        let mut origin = Program::new(vec![Box::new('0'), Box::new('1')], State(1));
        origin
            .insert(Instruction::new(
                Head::new(State(1), Box::new('0')),
                Tail::new(State(1), Box::new('9'), Move::Right),
            ))
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn fail_set_head_l_state_large() {
        let mut origin = Program::new(vec![Box::new('0'), Box::new('1')], State(1));
        origin
            .insert(Instruction::new(
                Head::new(State(2), Box::new('0')),
                Tail::new(State(1), Box::new('0'), Move::Right),
            ))
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn fail_set_tail_l_state_large() {
        let mut origin = Program::new(vec![Box::new('0'), Box::new('1')], State(1));
        origin
            .insert(Instruction::new(
                Head::new(State(1), Box::new('0')),
                Tail::new(State(2), Box::new('0'), Move::Right),
            ))
            .unwrap();
    }
}

#[cfg(test)]
mod clone_extend {
    use super::*;

    #[test]
    fn extend() {
        let mut expected = Program::new(vec![Box::new('0'), Box::new('1')], State(1));
        let _ = expected.insert(Instruction::new(
            Head::new(State(1), Box::new('0')),
            Tail::new(State(1), Box::new('0'), Move::Right),
        ));
        let _ = expected.insert(Instruction::new(
            Head::new(State(1), Box::new('1')),
            Tail::new(State(0), Box::new('0'), Move::Right),
        ));

        let mut program = Program::new(vec![Box::new('0'), Box::new('1')], State(1));
        program
            .extend([
                (1, Box::new('0'), 1, Box::new('0'), Move::Right),
                (1, Box::new('1'), 0, Box::new('0'), Move::Right),
            ])
            .unwrap();

        assert_eq!(expected, program);
    }
}

#[cfg(test)]
mod clone_with {
    use super::*;

    #[test]
    fn success_with() {
        let mut origin = Program::new(vec![Box::new('0'), Box::new('1')], State(1));
        let _ = origin.insert(Instruction::new(
            Head::new(State(1), Box::new('0')),
            Tail::new(State(1), Box::new('0'), Move::Right),
        ));
        let _ = origin.insert(Instruction::new(
            Head::new(State(1), Box::new('1')),
            Tail::new(State(0), Box::new('0'), Move::Right),
        ));

        let mut extension = Program::new(vec![Box::new('0'), Box::new('1')], State(2));
        let _ = extension.insert(Instruction::new(
            Head::new(State(1), Box::new('0')),
            Tail::new(State(0), Box::new('0'), Move::Left),
        ));
        let _ = extension.insert(Instruction::new(
            Head::new(State(1), Box::new('1')),
            Tail::new(State(2), Box::new('1'), Move::Right),
        ));
        let _ = extension.insert(Instruction::new(
            Head::new(State(2), Box::new('0')),
            Tail::new(State(0), Box::new('0'), Move::Left),
        ));
        let _ = extension.insert(Instruction::new(
            Head::new(State(2), Box::new('1')),
            Tail::new(State(2), Box::new('1'), Move::Right),
        ));

        let result = origin.with(&extension).unwrap();

        let mut expected = Program::new(vec![Box::new('0'), Box::new('1')], State(3));
        let _ = expected.insert(Instruction::new(
            Head::new(State(1), Box::new('0')),
            Tail::new(State(1), Box::new('0'), Move::Right),
        ));
        let _ = expected.insert(Instruction::new(
            Head::new(State(1), Box::new('1')),
            Tail::new(State(2), Box::new('0'), Move::Right),
        ));
        let _ = expected.insert(Instruction::new(
            Head::new(State(2), Box::new('0')),
            Tail::new(State(0), Box::new('0'), Move::Left),
        ));
        let _ = expected.insert(Instruction::new(
            Head::new(State(2), Box::new('1')),
            Tail::new(State(3), Box::new('1'), Move::Right),
        ));
        let _ = expected.insert(Instruction::new(
            Head::new(State(3), Box::new('0')),
            Tail::new(State(0), Box::new('0'), Move::Left),
        ));
        let _ = expected.insert(Instruction::new(
            Head::new(State(3), Box::new('1')),
            Tail::new(State(3), Box::new('1'), Move::Right),
        ));

        assert_eq!(expected, result);
    }

    #[test]
    #[should_panic]
    fn fail_with_alphabet() {
        let mut origin = Program::new(vec![Box::new('0')], State(1));
        let _ = origin.insert(Instruction::new(
            Head::new(State(1), Box::new('0')),
            Tail::new(State(1), Box::new('0'), Move::Right),
        ));

        let mut extension = Program::new(vec![Box::new('0'), Box::new('1')], State(2));
        let _ = extension.insert(Instruction::new(
            Head::new(State(1), Box::new('0')),
            Tail::new(State(0), Box::new('0'), Move::Left),
        ));
        let _ = extension.insert(Instruction::new(
            Head::new(State(1), Box::new('1')),
            Tail::new(State(2), Box::new('1'), Move::Right),
        ));

        origin.with(&extension).unwrap();
    }
}
