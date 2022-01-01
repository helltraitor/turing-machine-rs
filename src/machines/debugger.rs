use crate::instruction::{Direction, Head, Tail};
use crate::state::Configuration;
use crate::{Symbol, TuringMachine};

type CHandler<S> = Box<dyn Fn(&Configuration<S>)>;
type IHandler<S> = Box<dyn Fn(&Head<S>, &Tail<S>)>;

pub struct Debugger<Machine, S: Symbol>
where
    Machine: TuringMachine<S>,
{
    machine: Machine,
    c_handler: Option<CHandler<S>>,
    i_handler: Option<IHandler<S>>,
}

impl<Machine, S: Symbol> Debugger<Machine, S>
where
    Machine: TuringMachine<S>,
{
    pub fn new(machine: Machine) -> Self {
        Debugger {
            machine,
            c_handler: None,
            i_handler: None,
        }
    }

    pub fn set_c_handler(&mut self, c_handler: impl Fn(&Configuration<S>) + 'static) {
        self.c_handler = Some(Box::new(c_handler));
    }

    pub fn set_i_handler(&mut self, i_handler: impl Fn(&Head<S>, &Tail<S>) + 'static) {
        self.i_handler = Some(Box::new(i_handler));
    }
}

impl<Machine, S: Symbol> TuringMachine<S> for Debugger<Machine, S>
where
    Machine: TuringMachine<S>,
{
    fn execute_once(&self, conf: Configuration<S>) -> Configuration<S> {
        let next = self.machine.execute_once(conf.clone());
        if let Some(ref c_handler) = self.c_handler {
            c_handler(&conf);
        }
        if let Some(ref i_handler) = self.i_handler {
            let head = Head::new(conf.state, conf.get_symbol().clone());
            let direction = match (conf.index(), next.index()) {
                (old, new) if old < new => Direction::Right,
                (old, new) if old == new => Direction::Center,
                (old, new) if old > new => Direction::Right,
                (old, new) => panic!(
                    "internal error: not all compare cases are covered with old is {} and new is {}",
                    old,
                    new
                )
            };
            let tail = Tail::new(next.state, next.get_symbol().clone(), direction);
            i_handler(&head, &tail);
        }
        next
    }

    fn execute_until(
        &self,
        mut conf: Configuration<S>,
        until: impl Fn(&Configuration<S>) -> bool,
    ) -> Configuration<S> {
        if self.c_handler.is_none() && self.i_handler.is_none() {
            return self.machine.execute_until(conf, until);
        }

        while !until(&conf) {
            conf = self.execute_once(conf);
        }
        conf
    }
}
