use turing_machine_rs::instruction::{Head, State};

#[cfg(test)]
mod copy {
    use super::*;

    #[test]
    fn creation() {
        let _ = Head::new(State(0), 'C');
    }

    #[test]
    fn equality() {
        let lhs = Head::new(State(0), 'c');
        let rhs = Head::new(State(0), 'c');

        assert_eq!(lhs, rhs);
    }
}

#[cfg(test)]
mod clone {
    use super::*;

    #[test]
    fn creation() {
        let _ = Head::new(State(0), Box::new('c'));
    }

    #[test]
    fn equality() {
        let lhs = Head::new(State(0), Box::new('c'));
        let rhs = Head::new(State(0), Box::new('c'));

        assert_eq!(lhs, rhs);
    }
}
