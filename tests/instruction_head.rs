use turing_machine_rs::instruction::Head;

#[cfg(test)]
mod copy {
    use super::*;

    #[test]
    fn creation() {
        let _ = Head::new(0, 'C');
    }

    #[test]
    fn equality() {
        let lhs = Head::new(0, 'c');
        let rhs = Head::new(0, 'c');

        assert_eq!(lhs, rhs);
    }
}

#[cfg(test)]
mod clone {
    use super::*;

    #[test]
    fn creation() {
        let _ = Head::new(0, Box::new('c'));
    }

    #[test]
    fn equality() {
        let lhs = Head::new(0, Box::new('c'));
        let rhs = Head::new(0, Box::new('c'));

        assert_eq!(lhs, rhs);
    }
}
