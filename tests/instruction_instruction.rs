use turing_machine_rs::instruction::{Head, Instruction, Move, State, Tail};

#[cfg(test)]
mod copy {
    use super::*;

    #[test]
    fn creation() {
        let _ = Instruction::new(
            Head::new(State(1), 'c'),
            Tail::new(State(0), 'b', Move::Right),
        );
    }

    #[test]
    fn equality() {
        let lhs = Instruction::new(
            Head::new(State(1), 'c'),
            Tail::new(State(0), 'b', Move::Right),
        );
        let rhs = Instruction::new(
            Head::new(State(1), 'c'),
            Tail::new(State(0), 'b', Move::Right),
        );

        assert_eq!(lhs, rhs);
    }
}

#[cfg(test)]
mod clone {
    use super::*;

    #[test]
    fn creation() {
        let _ = Instruction::new(
            Head::new(State(1), Box::new('c')),
            Tail::new(State(0), Box::new('b'), Move::Right),
        );
    }

    #[test]
    fn equality() {
        let lhs = Instruction::new(
            Head::new(State(1), Box::new('c')),
            Tail::new(State(0), Box::new('b'), Move::Right),
        );
        let rhs = Instruction::new(
            Head::new(State(1), Box::new('c')),
            Tail::new(State(0), Box::new('b'), Move::Right),
        );

        assert_eq!(lhs, rhs);
    }
}
