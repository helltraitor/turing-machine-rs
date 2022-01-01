use std::fmt::{Display, Error, Formatter};

use crate::instruction::{Direction, Head, Instruction, Tail};
use crate::Symbol;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Program<S: Symbol> {
    container: Vec<Instruction<S>>,
    alphabet: Vec<S>,
    l_state: u32,
}

impl<S: Symbol> Program<S> {
    pub fn new(alphabet: Vec<S>, l_state: u32) -> Self {
        assert!(!alphabet.is_empty(), "new error: alphabet cannot be empty");
        assert!(
            l_state > 0,
            "new error: l_state must have (be) 1 state at least (start)"
        );

        let capacity = alphabet.len() * (l_state as usize);
        let container = Vec::with_capacity(capacity);
        Program {
            alphabet,
            container,
            l_state,
        }
    }

    pub fn alphabet(&self) -> &Vec<S> {
        &self.alphabet
    }

    pub fn extend(&mut self, other: &Program<S>) {
        assert!(
            self.alphabet == other.alphabet,
            "extend error: alphabet {:?} and {:?} must be equal",
            &self.alphabet,
            &other.alphabet
        );
        assert!(
            self.container.len() + other.container.len() > self.container.capacity(),
            "extend error: program type has limited size (count of alphabet * (count of states - 1))"
        );
        let old_l_state = self.l_state;
        self.l_state += other.l_state;

        for inst in self.container.iter_mut() {
            if inst.tail.state == 0 {
                inst.tail.state = old_l_state + 1;
            }
        }

        for inst in other.container.iter() {
            let mut inst = inst.clone();
            inst.head.state += old_l_state;
            inst.tail.state += match inst.tail.state {
                0 => 0,
                _ => old_l_state,
            };
            self.set(inst);
        }
    }

    pub fn get(&self, head: &Head<S>) -> Option<&Instruction<S>> {
        assert!(
            self.l_state >= head.state,
            "get error: required state {} is large then largest {}",
            head.state,
            self.l_state
        );
        self.container
            .iter()
            .find(|inst: &&Instruction<S>| &inst.head == head)
    }

    pub fn l_state(&self) -> u32 {
        self.l_state
    }

    pub fn set(&mut self, inst: Instruction<S>) {
        assert!(
            inst.head.state != 0,
            "set error: instruction {} cannot have 0 state in head",
            inst
        );
        assert!(
            self.alphabet.contains(&inst.head.symbol) && self.alphabet.contains(&inst.tail.symbol),
            "set error: instruction {} not for program with alphabet {:?}",
            inst,
            &self.alphabet
        );
        assert!(
            self.l_state >= inst.head.state && self.l_state >= inst.tail.state,
            "set error: instruction {} have states which is large then program largest state {}",
            inst,
            self.l_state
        );
        let position = self
            .container
            .iter()
            .position(|cand: &Instruction<S>| cand.head == inst.head);
        match position {
            Some(index) => self.container[index] = inst,
            None => self.container.push(inst),
        };
    }
}

pub trait ExtendBy<I: ?Sized> {
    fn extend_by(&mut self, _: I);
}

impl<S: Symbol, I> ExtendBy<I> for Program<S>
where
    I: IntoIterator<Item = (u32, S, u32, S, Direction)>,
{
    fn extend_by(&mut self, iterable: I) {
        for (h_state, h_symbol, t_state, t_symbol, t_direction) in iterable {
            self.set(Instruction::new(
                Head::new(h_state, h_symbol),
                Tail::new(t_state, t_symbol, t_direction),
            ));
        }
    }
}

impl<S: Symbol> Display for Program<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        use std::any::type_name;

        write!(
            f,
            "Program<{}> {{ alphabet {:?} instuctions: {}, l_state: {} }}",
            type_name::<S>(),
            self.alphabet,
            self.container.len(),
            self.l_state
        )
    }
}
