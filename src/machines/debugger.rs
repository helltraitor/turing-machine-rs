use crate::instruction::{Head, Move, Tail};
use crate::state::Configuration;
use crate::{Symbol, TuringMachine};

type CHandler<S> = Box<dyn Fn(&Configuration<S>)>;
type IHandler<S> = Box<dyn Fn(&Head<S>, &Tail<S>)>;

/// [`Debugger`] is an super useful [`TuringMachine`] for debugging another
/// Turing machine! This machine debugged all Turing machines in tests.
///
/// Note: this machine is not implementing [`crate::With`].
///
/// # Examples
/// ```rust
/// use std::cell::RefCell;
/// use std::ops::Deref;
/// use std::rc::Rc;
///
/// use turing_machine_rs::TuringMachine;
/// use turing_machine_rs::instruction::{Move, State};
/// use turing_machine_rs::machines::{Debugger, Classic};
/// use turing_machine_rs::program::{Extend, Program};
/// use turing_machine_rs::state::{Configuration, Tape};
///
/// fn main() -> Result<(), String> {
///     let mut program = Program::new(vec![' '], State(1));
///     program.extend([(1, ' ', 1, ' ', Move::Right)])?;
///
///     let machine = Classic::new(program, ' ')?;
///     let mut debugger = Debugger::new(machine);
///
///     let conf = Configuration::new_nrm(Tape::from("   "))?;
///
///     let buffer = Rc::new(RefCell::new(String::new()));
///
///     let c_buffer = buffer.clone();
///     debugger.set_c_handler(move |_| {
///         let mut buffer = c_buffer.borrow_mut();
///         buffer.push('c');
///     });
///
///     let conf = debugger.execute_once(conf)?;
///     debugger.execute_once(conf)?;
///
///     assert_eq!(String::from("cc"), buffer.deref().borrow().as_ref());
///     Ok(())
/// }
/// ```
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
    /// Constructs a new [`Debugger`] with a [`TuringMachine`] and no handlers.
    ///
    /// For setup handlers, use the [`Debugger::set_c_handler`]
    /// and the [`Debugger::set_i_handler`] methods.
    pub fn new(machine: Machine) -> Self {
        Debugger {
            machine,
            c_handler: None,
            i_handler: None,
        }
    }

    /// Sets a handler for configurations. Handler must implement
    /// [`Fn(&Configuration<Symbol>)`] trait.
    ///
    /// This function is not permanent so handler can be changed.
    pub fn set_c_handler(&mut self, c_handler: impl Fn(&Configuration<S>) + 'static) {
        self.c_handler = Some(Box::new(c_handler));
    }

    /// Sets a handler for instructions. Handler must implement
    /// [`Fn(&Head<Symbol>, &Tail<Symbol>)`] trait.
    ///
    /// This function is not permanent so handler can be changed.
    pub fn set_i_handler(&mut self, i_handler: impl Fn(&Head<S>, &Tail<S>) + 'static) {
        self.i_handler = Some(Box::new(i_handler));
    }
}

impl<Machine, S: Symbol> TuringMachine<S> for Debugger<Machine, S>
where
    Machine: TuringMachine<S>,
{
    /// Executes [`Configuration`] once by mutation.
    ///
    /// Works quickly when no handler is set (but you probably don't want to
    /// use the debugger without the debugging).
    ///
    /// # Panics
    /// [`Debugger`] could panic only if source code is broken - this would be a bug.
    /// All match cases must and are covered.
    ///
    /// So you could open an issue on [GitHub](https://github.com/Helltraitor/turing-machine-rs).
    fn execute_once(&self, conf: Configuration<S>) -> Result<Configuration<S>, String> {
        let next = self.machine.execute_once(conf.clone())?;
        if let Some(ref c_handler) = self.c_handler {
            c_handler(&conf);
        }
        if let Some(ref i_handler) = self.i_handler {
            let head = Head::new(conf.state, conf.get_symbol().clone());
            let movement = match (conf.index(), next.index()) {
                (old, new) if old < new => Move::Right,
                (old, new) if old == new => Move::None,
                (old, new) if old > new => Move::Right,
                (old, new) => panic!(
                    "execute_once error: not all compare cases are covered for old {} and new {} indexes",
                    old,
                    new
                )
            };
            let tail = Tail::new(next.state, next.get_symbol().clone(), movement);
            i_handler(&head, &tail);
        }
        Ok(next)
    }

    /// Executes [`Configuration`] until predicate is `false` by mutation.
    ///
    /// Uses the [`Debugger::execute_once`] method in the loop. Works quickly
    /// when no handler set (but probably you don't wnat to use the debugger without tools).
    fn execute_until(
        &self,
        mut conf: Configuration<S>,
        until: impl Fn(&Configuration<S>) -> bool,
    ) -> Result<Configuration<S>, String> {
        if self.c_handler.is_none() && self.i_handler.is_none() {
            return self.machine.execute_until(conf, until);
        }
        while !until(&conf) {
            conf = self.execute_once(conf)?;
        }
        Ok(conf)
    }
}
