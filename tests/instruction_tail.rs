use turing_machine_rs::instruction::{Direction, Tail};

mod copy {
    use super::*;

    #[test]
    fn creation() {
        let _ = Tail::new(0, 'c', Direction::Center);
    }

    #[test]
    fn equality() {
        let lhs = Tail::new(0, 'c', Direction::Center);
        let rhs = Tail::new(0, 'c', Direction::Center);

        assert_eq!(lhs, rhs);
    }
}

mod clone {
    use super::*;

    #[test]
    fn creation() {
        let _ = Tail::new(0, Box::new('c'), Direction::Center);
    }

    #[test]
    fn equality() {
        let lhs = Tail::new(0, Box::new('c'), Direction::Center);
        let rhs = Tail::new(0, Box::new('c'), Direction::Center);

        assert_eq!(lhs, rhs);
    }
}
