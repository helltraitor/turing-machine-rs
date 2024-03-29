use turing_machine_rs::instruction::{Move, State, Tail};
mod copy {
    use super::*;

    #[test]
    fn creation() {
        let _ = Tail::new(State(0), 'c', Move::None);
    }

    #[test]
    fn equality() {
        let lhs = Tail::new(State(0), 'c', Move::None);
        let rhs = Tail::new(State(0), 'c', Move::None);

        assert_eq!(lhs, rhs);
    }
}

mod clone {
    use super::*;

    #[test]
    fn creation() {
        let _ = Tail::new(State(0), Box::new('c'), Move::None);
    }

    #[test]
    fn equality() {
        let lhs = Tail::new(State(0), Box::new('c'), Move::None);
        let rhs = Tail::new(State(0), Box::new('c'), Move::None);

        assert_eq!(lhs, rhs);
    }
}
