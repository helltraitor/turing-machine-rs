use turing_machine_rs::instruction::{Direction, Head, Instruction, Tail};

#[cfg(test)]
mod copy {
    use super::*;

    #[test]
    fn creation() {
        let _ = Instruction::new(Head::new(1, 'c'), Tail::new(0, 'b', Direction::Right));
    }

    #[test]
    fn equality() {
        let lhs = Instruction::new(Head::new(1, 'c'), Tail::new(0, 'b', Direction::Right));
        let rhs = Instruction::new(Head::new(1, 'c'), Tail::new(0, 'b', Direction::Right));

        assert_eq!(lhs, rhs);
    }
}

#[cfg(test)]
mod clone {
    use super::*;

    #[test]
    fn creation() {
        let _ = Instruction::new(
            Head::new(1, Box::new('c')),
            Tail::new(0, Box::new('b'), Direction::Right),
        );
    }

    #[test]
    fn equality() {
        let lhs = Instruction::new(
            Head::new(1, Box::new('c')),
            Tail::new(0, Box::new('b'), Direction::Right),
        );
        let rhs = Instruction::new(
            Head::new(1, Box::new('c')),
            Tail::new(0, Box::new('b'), Direction::Right),
        );

        assert_eq!(lhs, rhs);
    }
}
