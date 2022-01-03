use turing_machine_rs::instruction::Direction;
use turing_machine_rs::state::{Configuration, Tape};

#[cfg(test)]
mod copy {
    use super::*;

    #[test]
    fn success_creation() {
        let tape = Tape::from("test");
        let _ = Configuration::new(tape, 0, 1).unwrap();
    }

    #[test]
    #[should_panic]
    fn fail_creation() {
        let tape = Tape::from("test");
        let _ = Configuration::new(tape, 5, 1).unwrap();
    }

    #[test]
    fn destruction() {
        let conf = Configuration::new(Tape::from("_"), 0, 1).unwrap();
        let items = conf.destruct();

        assert_eq!((Tape::from("_"), 0, 1), items);
    }

    #[test]
    fn index() {
        let conf = Configuration::new(Tape::from("test"), 0, 1).unwrap();
        assert_eq!(conf.index(), 0);

        let conf = Configuration::new(Tape::from("test"), 1, 1).unwrap();
        assert_eq!(conf.index(), 1);
    }

    #[test]
    fn is_empty() {
        let conf = Configuration::new(Tape::from("_"), 0, 1).unwrap();
        assert!(!conf.is_empty());
    }

    #[test]
    fn into_tape() {
        let conf = Configuration::new(Tape::from("_"), 0, 1).unwrap();
        assert_eq!(conf.into_tape(), Tape::from("_"));
    }

    #[test]
    fn nrm_creation() {
        let conf = Configuration::new_nrm(Tape::from("test")).unwrap();

        let expected = Configuration::new(Tape::from("test"), 0, 1).unwrap();

        assert_eq!(expected, conf);
    }

    #[test]
    fn std_creation() {
        let conf = Configuration::new_std(Tape::from("test")).unwrap();
        let expected = Configuration::new(Tape::from("test"), 3, 1).unwrap();

        assert_eq!(expected, conf);
    }

    #[test]
    fn creation_equality() {
        let tape = Tape::from("test");

        let lhs = Configuration::new_nrm(tape.clone()).unwrap();
        let rhs = Configuration::new(tape.clone(), 0, 1).unwrap();
        assert_eq!(lhs, rhs);

        let lhs = Configuration::new_std(tape.clone()).unwrap();
        let rhs = Configuration::new(tape.clone(), tape.len() - 1, 1).unwrap();
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn tape() {
        let tape = Tape::from("test");
        let conf = Configuration::new_std(tape.clone()).unwrap();

        assert_eq!(conf.tape().as_vec(), tape.as_vec());
    }

    #[test]
    fn set_symbol() {
        let mut conf = Configuration::new(Tape::from("test"), 0, 1).unwrap();
        conf.set_symbol('T');

        let expected = Configuration::new(Tape::from("Test"), 0, 1).unwrap();

        assert_eq!(expected, conf);
    }

    #[test]
    fn get_symbol() {
        let conf = Configuration::new_std(Tape::from("test!")).unwrap();

        assert_eq!(*conf.get_symbol(), '!');
    }

    #[test]
    fn shift() {
        let tape = Tape::from("test");
        let mut conf = Configuration::new_std(tape).unwrap();

        conf.shift(Direction::Right, '!'); // test!
                                           // ----^
        conf.shift(Direction::Left, ' '); // test!
                                          // ---^
        conf.shift(Direction::Left, ' '); // test!
                                          // --^
        conf.shift(Direction::Left, ' '); // test!
                                          // -^
        conf.shift(Direction::Left, ' '); // test!
                                          // ^
        conf.set_symbol('T');

        let expected = Configuration::new(Tape::from("Test!"), 0, 1).unwrap();
        assert_eq!(expected, conf);
    }

    #[test]
    fn len() {
        let tape = Tape::from("test");
        let conf = Configuration::new_std(tape.clone()).unwrap();

        assert_eq!(conf.len(), tape.len());

        let mut conf = conf.clone();
        conf.shift(Direction::Right, '!'); // test!
                                           // ----^
        assert_eq!(conf.len(), tape.len() + 1);
    }

    #[test]
    fn state_independence() {
        // Whatever going on, configuration state must not be changed by self methods
        // It must be changed only by outside

        let tape = Tape::from("test");
        let mut conf = Configuration::new(tape, 0, 1).unwrap();

        let _ = conf.index();
        let _ = conf.tape();
        conf.set_symbol('T');
        let _ = conf.get_symbol();

        conf.shift(Direction::Right, ' '); // Test
                                           // ^
        conf.shift(Direction::Right, ' '); // Test
                                           // -^
        conf.shift(Direction::Right, ' '); // Test
                                           // --^
        conf.shift(Direction::Right, ' '); // Test
                                           // ---^
        conf.shift(Direction::Right, '!'); // Test!
                                           // ----^
        let _ = conf.len();

        assert_eq!(conf.state, 1);
    }
}

#[cfg(test)]
mod clone {
    use super::*;

    #[test]
    fn success_creation() {
        let tape = Tape::new("test".chars().map(|ch| Box::new(ch)));
        let _ = Configuration::new(tape, 0, 1);
    }

    #[test]
    #[should_panic]
    fn fail_creation() {
        let tape = Tape::new("test".chars().map(|ch| Box::new(ch)));
        let _ = Configuration::new(tape, 5, 1).unwrap();
    }

    #[test]
    fn destruction() {
        let conf = Configuration::new(Tape::new([Box::new("_")]), 0, 1).unwrap();
        let items = conf.destruct();

        assert_eq!((Tape::new([Box::new("_")]), 0, 1), items);
    }

    #[test]
    fn index() {
        let conf =
            Configuration::new(Tape::new("test".chars().map(|ch| Box::new(ch))), 0, 1).unwrap();
        assert_eq!(conf.index(), 0);

        let conf =
            Configuration::new(Tape::new("test".chars().map(|ch| Box::new(ch))), 1, 1).unwrap();
        assert_eq!(conf.index(), 1);
    }

    #[test]
    fn is_empty() {
        let conf = Configuration::new(Tape::new([Box::new('_')]), 0, 1).unwrap();
        assert!(!conf.is_empty());
    }

    #[test]
    fn into_tape() {
        let conf = Configuration::new(Tape::new([Box::new('_')]), 0, 1).unwrap();
        assert_eq!(conf.into_tape(), Tape::new([Box::new('_')]));
    }

    #[test]
    fn nrm_creation() {
        let conf =
            Configuration::new_nrm(Tape::new("test".chars().map(|ch| Box::new(ch)))).unwrap();

        let expected =
            Configuration::new(Tape::new("test".chars().map(|ch| Box::new(ch))), 0, 1).unwrap();

        assert_eq!(expected, conf);
    }

    #[test]
    fn std_creation() {
        let conf = Configuration::new_std(Tape::new("test".chars().map(|ch| Box::new(ch))));
        let expected = Configuration::new(Tape::new("test".chars().map(|ch| Box::new(ch))), 3, 1);

        assert_eq!(expected, conf);
    }

    #[test]
    fn creation_equality() {
        let tape = Tape::new("test".chars().map(|ch| Box::new(ch)));

        let lhs = Configuration::new_nrm(tape.clone()).unwrap();
        let rhs = Configuration::new(tape.clone(), 0, 1).unwrap();
        assert_eq!(lhs, rhs);

        let lhs = Configuration::new_std(tape.clone()).unwrap();
        let rhs = Configuration::new(tape.clone(), tape.len() - 1, 1).unwrap();
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn tape() {
        let tape = Tape::new("test".chars().map(|ch| Box::new(ch)));
        let conf = Configuration::new_std(tape.clone()).unwrap();

        assert_eq!(conf.tape().as_vec(), tape.as_vec());
    }

    #[test]
    fn set_symbol() {
        let mut conf =
            Configuration::new(Tape::new("test".chars().map(|ch| Box::new(ch))), 0, 1).unwrap();
        conf.set_symbol(Box::new('T'));

        let expected =
            Configuration::new(Tape::new("Test".chars().map(|ch| Box::new(ch))), 0, 1).unwrap();

        assert_eq!(expected, conf);
    }

    #[test]
    fn get_symbol() {
        let conf =
            Configuration::new_std(Tape::new("test!".chars().map(|ch| Box::new(ch)))).unwrap();

        assert_eq!(*conf.get_symbol(), Box::new('!'));
    }

    #[test]
    fn shift() {
        let tape = Tape::new("test".chars().map(|ch| Box::new(ch)));
        let mut conf = Configuration::new_std(tape).unwrap();

        conf.shift(Direction::Right, Box::new('!')); // test!
                                                     // ----^
        conf.shift(Direction::Left, Box::new(' ')); // test!
                                                    // ---^
        conf.shift(Direction::Left, Box::new(' ')); // test!
                                                    // --^
        conf.shift(Direction::Left, Box::new(' ')); // test!
                                                    // -^
        conf.shift(Direction::Left, Box::new(' ')); // test!
                                                    // ^
        conf.set_symbol(Box::new('T'));

        let expected =
            Configuration::new(Tape::new("Test!".chars().map(|ch| Box::new(ch))), 0, 1).unwrap();
        assert_eq!(expected, conf);
    }

    #[test]
    fn len() {
        let tape = Tape::new("test".chars().map(|ch| Box::new(ch)));
        let conf = Configuration::new_std(tape.clone()).unwrap();

        assert_eq!(conf.len(), tape.len());

        let mut conf = conf.clone();
        conf.shift(Direction::Right, Box::new('!')); // test!
                                                     // ----^
        assert_eq!(conf.len(), tape.len() + 1);
    }

    #[test]
    fn state_independence() {
        // Whatever going on, configuration state must not be changed by self methods
        // It must be changed only by outside

        let tape = Tape::new("test".chars().map(|ch| Box::new(ch)));
        let mut conf = Configuration::new(tape, 0, 1).unwrap();

        let _ = conf.index();
        let _ = conf.tape();
        conf.set_symbol(Box::new('T'));
        let _ = conf.get_symbol();

        conf.shift(Direction::Right, Box::new(' ')); // Test
                                                     // ^
        conf.shift(Direction::Right, Box::new(' ')); // Test
                                                     // -^
        conf.shift(Direction::Right, Box::new(' ')); // Test
                                                     // --^
        conf.shift(Direction::Right, Box::new(' ')); // Test
                                                     // ---^
        conf.shift(Direction::Right, Box::new('!')); // Test!
                                                     // ----^
        let _ = conf.len();

        assert_eq!(conf.state, 1);
    }
}
