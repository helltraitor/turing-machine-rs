use turing_machine_rs::state::Tape;

#[cfg(test)]
mod copy {
    use super::*;

    #[test]
    fn creation() {
        let _: Tape<char> = Tape::from("test");
    }

    #[test]
    fn as_vec() {
        let tape: Tape<char> = Tape::from("test");
        let vec = tape.as_vec();

        assert_eq!(*vec, vec!['t', 'e', 's', 't']);
    }

    #[test]
    fn get() {
        let tape = Tape::from("test");

        assert_eq!(tape.get(0), Some(&'t'));
        assert_eq!(tape.get(2), Some(&'s'));
    }

    #[test]
    fn insert() {
        let mut tape = Tape::from("test");
        tape.insert(0, '0');
        tape.insert(5, '1');

        let expected = Tape::from("0test1");

        assert_eq!(expected, tape);
    }

    #[test]
    fn is_empty() {
        let tape: Tape<char> = Tape::from("");
        assert!(tape.is_empty());

        let tape: Tape<char> = Tape::from("_");
        assert!(!tape.is_empty());
    }

    #[test]
    fn len() {
        let mut tape = Tape::from("test");
        assert_eq!(tape.len(), 4);

        tape.insert(0, '0');
        assert_eq!(tape.len(), 5);
    }

    #[test]
    fn set() {
        let mut tape = Tape::from("test");
        tape.set(0, 'n');
        tape.set(1, 'i');
        tape.set(2, 'c');
        tape.set(3, 'e');

        let expected = Tape::from("nice");

        assert_eq!(expected, tape);
    }

    #[test]
    fn to_string() {
        let tape: Tape<char> = Tape::from("test");

        let expected = String::from("test");

        assert_eq!(expected, tape.to_string());
    }

    #[test]
    fn from_string() {
        let tape: Tape<char> = Tape::from(String::from("test"));

        let expected = Tape::from("test");

        assert_eq!(expected, tape);
    }

    #[test]
    fn from_str() {
        let tape = Tape::from("test");

        let expected = Tape::new("test".chars());

        assert_eq!(expected, tape);
    }
}

#[cfg(test)]
mod clone {
    use super::*;

    #[test]
    fn creation() {
        let _: Tape<Box<char>> = Tape::from("test");
    }

    #[test]
    fn as_vec() {
        let tape: Tape<Box<char>> = Tape::from("test");
        let vec = tape.as_vec();

        assert_eq!(
            *vec,
            vec![Box::new('t'), Box::new('e'), Box::new('s'), Box::new('t')]
        );
    }

    #[test]
    fn get() {
        let tape = Tape::from("test");

        assert_eq!(tape.get(0), Some(&Box::new('t')));
        assert_eq!(tape.get(2), Some(&Box::new('s')));
    }

    #[test]
    fn insert() {
        let mut tape = Tape::from("test");
        tape.insert(0, Box::new('0'));
        tape.insert(5, Box::new('1'));

        let expected = Tape::from("0test1");

        assert_eq!(expected, tape);
    }

    #[test]
    fn is_empty() {
        let tape: Tape<Box<char>> = Tape::from("");
        assert!(tape.is_empty());

        let tape: Tape<Box<char>> = Tape::from("_");
        assert!(!tape.is_empty());
    }

    #[test]
    fn len() {
        let mut tape = Tape::from("test");
        assert_eq!(tape.len(), 4);

        tape.insert(0, Box::new('0'));
        assert_eq!(tape.len(), 5);
    }

    #[test]
    fn set() {
        let mut tape = Tape::from("test");
        tape.set(0, Box::new('n'));
        tape.set(1, Box::new('i'));
        tape.set(2, Box::new('c'));
        tape.set(3, Box::new('e'));

        let expected = Tape::from("nice");

        assert_eq!(expected, tape);
    }

    #[test]
    fn to_string() {
        let tape: Tape<Box<char>> = Tape::from("test");

        let expected = String::from("test");

        assert_eq!(expected, tape.to_string());
    }

    #[test]
    fn from_string() {
        let tape: Tape<Box<char>> = Tape::from(String::from("test"));

        let expected = Tape::from("test");

        assert_eq!(expected, tape);
    }

    #[test]
    fn from_str() {
        let tape: Tape<Box<char>> = Tape::from("test");

        let expected = Tape::new(vec![
            Box::new('t'),
            Box::new('e'),
            Box::new('s'),
            Box::new('t'),
        ]);

        assert_eq!(expected, tape);
    }
}
