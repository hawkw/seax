#![crate_name = "seax_svm"]
#![stable(feature="vm_core", since="0.1.2")]
#![crate_type = "lib"]
#![feature(box_syntax,box_patterns)]
#![feature(staged_api)]
#![staged_api]

/// Singly-linked list and stack implementations.
///
/// `List<T>` is a singly-linked cons list with boxed items. `Stack<T>` is
///  defined as a trait providing stack operations(`push()`, `pop()`, and
///  `peek()`), and an implementation for `List`.
#[macro_use]
#[stable(feature="list", since="0.1.0")]
pub mod slist;

/// SVM cell types.
///
/// A cell in the VM can be either an atom (single item, either unsigned
/// int, signed int, float, or string), a pointer to a list cell, or an
/// instruction.
#[stable(feature="vm_core", since="0.1.2")]
pub mod cell;

#[cfg(test)]
mod tests;

/// Contains the Seax Virtual Machine (SVM) and miscellaneous
/// support code.
pub use self::slist::List;
pub use self::slist::List::{Cons,Nil};
pub use self::slist::Stack;
pub use self::cell::{SVMCell,Atom,Inst};
use self::cell::SVMCell::*;
use self::cell::Atom::*;
use self::cell::Inst::*;
use std::io;

/// Represents a SVM machine state
#[derive(PartialEq,Clone,Debug)]
#[stable(feature="vm_core", since="0.1.0")]
pub struct State {
    stack:  List<SVMCell>,
    env:  List<SVMCell>,
    control:  List<SVMCell>,
    dump:  List<SVMCell>
}
#[stable(feature="vm_core", since="0.1.0")]
impl State {

    /// Creates a new empty state
    #[stable(feature="vm_core", since="0.1.0")]
    pub fn new() -> State {
        State {
            stack: Stack::empty(),
            env: Stack::empty(),
            control: Stack::empty(),
            dump: Stack::empty()
        }
    }


    /// Dump state to string
    ///
    /// This produces state dumps suitable for printing as part of
    /// an error report. This is different from fmt::Debug since it
    /// includes a tag for the error reporter.
    #[stable(feature="debug", since="0.2.0")]
    pub fn dump_state(&self, tag: &str) -> String {
        format!(
            "[{t}] State dump:\n[{t}]\t\tStack:\t {s:?}\n[{t}]\t\tEnv:\t {e:?}\n[{t}]\t\tControl: {c:?}\n[{t}]\t\tDump:\t {d:?}\n",
                t = tag,
                s = &self.stack,
                e = &self.env,
                c = &self.control,
                d = &self.dump
            )
    }

    /// Evaluates an instruction.
    ///
    /// Evaluates an instruction against a state, returning a new state.
    ///
    /// # Arguments:
    ///
    ///  - `inp`: an input stream implementing `io::Read`
    ///  - `outp`: an output stream implementing `io::Write`
    ///  - `debug`: whether or not to snapshot the state before evaluating. This provides more detailed debugging information on errors, but may have a significant impact on performance.
    ///
    #[stable(feature="vm_core", since="0.2.4")]
    pub fn eval(self,
                inp: &mut io::Read,
                outp: &mut io::Write,
                debug: bool)
                -> State {
        // TODO: this (by which I mean "the whole caching deal") could likely be made
        // better and/or faster with some clever (mis?)use of RefCell; look into that.
        let prev = if debug { Some(self.clone()) } else { None };
        match self.control.pop() {
            // NIL: pop an empty list onto the stack
            Some((InstCell(NIL), new_control)) => {
                State {
                    stack: self.stack.push(ListCell(box List::new())),
                    env: self.env,
                    control: new_control,
                    dump: self.dump
                }
            }
            // LDC: load constant
            Some((InstCell(LDC), new_control)) => {
                let (atom,newer_control) = new_control.pop().unwrap();
                State {
                    stack: self.stack.push(atom),
                    env: self.env,
                    control: newer_control,
                    dump: self.dump
                }
            },
            // LD: load variable
            Some((InstCell(LD), new_control)) => {
                match new_control.pop() {
                    Some((ListCell(
                        box Cons(AtomCell(UInt(lvl)),
                        box Cons(AtomCell(UInt(idx)),
                        box Nil))
                        ), newer_control)) => match self.env[(lvl-1)] {
                            ListCell(ref level) => State {
                                stack: match level.get(idx-1) {
                                    Some(thing) => self.stack.push(thing.clone()),
                                    None        => self.stack
                                },
                                env: self.env.clone(),
                                control: newer_control,
                                dump: self.dump
                            },
                            // This is a special case for something that, as far as I know,
                            // should never happen. But despite everything, it DOES happen.
                            ref thing @ AtomCell(_) => State { // I give up. Have your special case.
                                stack: self.stack.push(thing.clone()),
                                env: self.env.clone(),
                                control: newer_control,
                                dump: self.dump
                            },
                            _ => panic!(
                                "[fatal][LD]: expected list in $e, found {:?}\n{}",
                                self.env[lvl-1], prev.map_or(String::new(), |x| x.dump_state("fatal") ))
                    },
                   Some((ListCell( // TODO: this uses deprecated signed int indexing, remove
                        box Cons(AtomCell(SInt(lvl)),
                        box Cons(AtomCell(SInt(idx)),
                        box Nil))
                        ), newer_control)) =>  match self.env[(lvl-1)] {
                            SVMCell::ListCell(ref level) => State {
                                stack: self.stack.push(level[(idx-1)].clone()),
                                env: self.env.clone(),
                                control: newer_control,
                                dump: self.dump
                            },
                            _ => panic!(
                                "[fatal][LD]: expected list in $e, found {:?}\n{}",
                                self.env[lvl-1], prev.map_or(String::new(), |x| x.dump_state("fatal") ))
                    },
                   Some((thing,newer_control)) => panic!(
                        "[fatal][LD]: expected pair, found {:?}\n[fatal] new control: {:?}\n{}",
                        thing,
                        newer_control,
                        prev.map_or(String::new(), |x| x.dump_state("fatal") )),
                   None => panic!(
                        "[fatal][LD]: expected pair, found empty stack\n{}",
                        prev.map_or(String::new(), |x| x.dump_state("fatal") ))
                }
            },

            // LDF: load function
            Some((InstCell(LDF), new_control)) => {
                let (func, newer_control) = match new_control.pop() {
                    Some(thing) => thing,
                    None        => panic!(
                        "[fatal][LDF]: pop on empty control stack\n{}",
                        prev.map_or(String::new(), |x| x.dump_state("fatal") ))
                };
                State {
                    stack: self.stack.push(ListCell( box list!(func,self.env.get(0).map_or(ListCell(box Nil), |it| it.clone())) )),
                    env: self.env,
                    control: newer_control,
                    dump: self.dump
                }
            },

            Some((InstCell(JOIN), new_control)) => {
                let (top, new_dump) = match self.dump.pop() {
                    Some(thing) => thing,
                    None        => panic!(
                        "[fatal][JOIN]: pop on empty dump stack\n{}",
                        prev.map_or(String::new(), |x| x.dump_state("fatal") ))
                };
                State {
                    stack: self.stack,
                    env: self.env,
                    control: match top {
                        ListCell(box Nil) => new_control,
                        ListCell(box it)  => it,
                        anything          => panic!(
                            "[fatal][JOIN]: expected list on dump, found {:?}\n{}",
                            anything,prev.map_or(String::new(), |x| x.dump_state("fatal") ))
                    },
                    dump: new_dump
                }
            },
            Some((InstCell(ADD), new_control)) => match self.stack.pop() {
                Some((AtomCell(op1), new_stack)) => match new_stack.pop() {
                    Some((AtomCell(op2), newer_stack)) => State {
                            stack: newer_stack.push(AtomCell(op1 + op2)),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                    any => panic!(
                        "[fatal][ADD]: expected second operand, found {:?}\n{}",
                        any,
                        prev.map_or(String::new(), |x| x.dump_state("fatal") ))
                    },
                any => panic!(
                    "[fatal][ADD]: expected first operand, found {:?}\n{}",
                    any,
                    prev.map_or(String::new(), |x| x.dump_state("fatal") ))
            },
            Some((InstCell(SUB), new_control)) => match self.stack.pop() {
                Some((AtomCell(op1), new_stack)) => match new_stack.pop() {
                    Some((AtomCell(op2), newer_stack)) => State {
                            stack: newer_stack.push(AtomCell(op1 - op2)),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                    any => panic!(
                        "[fatal][SUB]: expected second operand, found {:?}\n{}",
                        any,
                        prev.map_or(String::new(), |x| x.dump_state("fatal") ))
                    },
                any => panic!(
                    "[fatal][SUB]: expected first operand, found {:?}\n{}",
                    any,
                    prev.map_or(String::new(), |x| x.dump_state("fatal") ))
            },
            Some((InstCell(FDIV), new_control)) => {
                let (op1, new_stack) = match self.stack.pop() {
                    Some(thing) => thing,
                    None        => panic!(
                        "[fatal][FDIV]: pop on empty stack\n{}",
                        prev.map_or(String::new(), |x| x.dump_state("fatal") ))
                };
                match op1 {
                    AtomCell(a) => {
                        let (op2, newer_stack) = match new_stack.pop() {
                            Some(thing) => thing,
                            None        => panic!(
                                "[fatal][FDIV]: pop on empty stack\n{}",
                                prev.map_or(String::new(), |x| x.dump_state("fatal") ))
                        };
                        match op2 {
                            AtomCell(b) => State {
                                stack: newer_stack.push(AtomCell(
                                    match (a, b) {
                                        // same type: coerce to float
                                        (SInt(a), SInt(b))      => Float(a as f64 / b as f64),
                                        (UInt(a), UInt(b))      => Float(a as f64 / b as f64),
                                        (Float(a), Float(b))    => Float(a / b),
                                        // float + int: coerce to float
                                        (Float(a), SInt(b))     => Float(a / b as f64),
                                        (Float(a), UInt(b))     => Float(a / b as f64),
                                        (SInt(a), Float(b))     => Float(a as f64 / b),
                                        (UInt(a), Float(b))     => Float(a as f64 / b),
                                        // uint + sint: coerce to float
                                        (UInt(a), SInt(b))      => Float(a as f64 / b as f64),
                                        (SInt(a), UInt(b))      => Float(a as f64 / b as f64),
                                        // char + any: coerce to int -> float
                                        // but if you ever actually do this, then ...wat?
                                        (Char(a), Char(b))      => Float(a as u8 as f64 / b as u8 as f64),
                                        (Char(a), UInt(b))      => Float(a as u8 as f64 / b as f64),
                                        (Char(a), SInt(b))      => Float(a as u8 as f64 / b as f64),
                                        (Char(a), Float(b))     => Float(a as u8 as f64 / b as f64),
                                        (UInt(a), Char(b))      => Float(a as f64 / b as u8 as f64),
                                        (SInt(a), Char(b))      => Float(a as f64 / b as u8 as f64),
                                        (Float(a), Char(b))     => Float(a as f64 / b as u8 as f64)
                                    }
                                    )),
                                env: self.env,
                                control: new_control,
                                dump: self.dump
                            },
                            b => panic!(
                                "[fatal][FDIV]: TypeError: expected compatible operands, found (FDIV {:?} {:?})\n{}",
                                a, b,prev.map_or(String::new(), |x| x.dump_state("fatal") ))
                        }
                    },
                    _ => panic!(
                            "[fatal][FDIV]: Expected first operand to be atom, found list or instruction\n{}",
                            prev.map_or(String::new(), |x| x.dump_state("fatal") )),
                }
            },
            Some((InstCell(DIV), new_control)) => match self.stack.pop() {
                Some((AtomCell(op1), new_stack)) => match new_stack.pop() {
                    Some((AtomCell(op2), newer_stack)) => State {
                            stack: newer_stack.push(AtomCell(op1 / op2)),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                    any => panic!(
                        "[fatal][DIV]: expected second operand, found {:?}\n{}",
                        any,
                        prev.map_or(String::new(), |x| x.dump_state("fatal") ))
                    },
                any => panic!(
                    "[fatal][DIV]: expected first operand, found {:?}\n{}",
                    any,
                    prev.map_or(String::new(), |x| x.dump_state("fatal") ))
            },
            Some((InstCell(MUL), new_control)) => match self.stack.pop() {
                Some((AtomCell(op1), new_stack)) => match new_stack.pop() {
                    Some((AtomCell(op2), newer_stack)) => State {
                            stack: newer_stack.push(AtomCell(op1 * op2)),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                    any => panic!(
                        "[fatal][MUL]: expected second operand, found {:?}\n{}",
                        any,
                        prev.map_or(String::new(), |x| x.dump_state("fatal") ))
                    },
                any => panic!(
                    "[fatal][MUL]: expected first operand, found {:?}\n{}",
                    any,
                    prev.map_or(String::new(), |x| x.dump_state("fatal") ))
            },
            Some((InstCell(MOD), new_control)) => match self.stack.pop() {
                Some((AtomCell(op1), new_stack)) => match new_stack.pop() {
                    Some((AtomCell(op2), newer_stack)) => State {
                            stack: newer_stack.push(AtomCell(op1 % op2)),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                    any => panic!(
                        "[fatal][MOD]: expected second operand, found {:?}\n{}",
                        any,
                        prev.map_or(String::new(), |x| x.dump_state("fatal") ))
                    },
                any => panic!(
                    "[fatal][MOD]: expected first operand, found {:?}\n{}",
                    any,
                    prev.map_or(String::new(), |x| x.dump_state("fatal") ))
            },
            Some((InstCell(EQ), new_control)) => {
                let (op1, new_stack) = self.stack.pop().unwrap();
                let (op2, newer_stack) = new_stack.pop().unwrap();
                match (op1,op2) {
                    (AtomCell(a), AtomCell(b)) => State {
                        stack: newer_stack.push(
                            match a == b {
                                true    => ListCell(box list!(AtomCell(SInt(1)))),
                                false   => ListCell(box Nil)
                            }
                        ),
                        env: self.env,
                        control: new_control,
                        dump: self.dump
                    },
                (_,_) => unimplemented!()
                }
            },
            Some((InstCell(GT), new_control)) => {
                let (op1, new_stack) = self.stack.pop().unwrap();
                let (op2, newer_stack) = new_stack.pop().unwrap();
                match (op1,op2) {
                    (AtomCell(a), AtomCell(b)) => State {
                        stack: newer_stack.push(
                            match a > b {
                                true    => ListCell(box list!(AtomCell(SInt(1)))),
                                false   => ListCell(box Nil)
                            }
                        ),
                        env: self.env,
                        control: new_control,
                        dump: self.dump
                    },
                (_,_) => unimplemented!()
                }
            },
            Some((InstCell(GTE), new_control)) => {
                let (op1, new_stack) = self.stack.pop().unwrap();
                let (op2, newer_stack) = new_stack.pop().unwrap();
                match (op1,op2) {
                    (AtomCell(a), AtomCell(b)) => State {
                        stack: newer_stack.push(
                            match a >= b {
                                true    => ListCell(box list!(AtomCell(SInt(1)))),
                                false   => ListCell(box Nil)
                            }
                        ),
                        env: self.env,
                        control: new_control,
                        dump: self.dump
                    },
                (_,_) => unimplemented!()
                }
            },
            Some((InstCell(LT), new_control)) => {
                let (op1, new_stack) = self.stack.pop().unwrap();
                let (op2, newer_stack) = new_stack.pop().unwrap();
                match (op1,op2) {
                    (AtomCell(a), AtomCell(b)) => State {
                        stack: newer_stack.push(
                            match a < b {
                                true    => ListCell(box list!(AtomCell(SInt(1)))),
                                false   => ListCell(box Nil)
                            }
                        ),
                        env: self.env,
                        control: new_control,
                        dump: self.dump
                    },
                (_,_) => unimplemented!()
                }
            },
            Some((InstCell(LTE), new_control)) => {
                let (op1, new_stack) = self.stack.pop().unwrap();
                let (op2, newer_stack) = new_stack.pop().unwrap();
                match (op1,op2) {
                    (AtomCell(a), AtomCell(b)) => State {
                        stack: newer_stack.push(
                            match a <= b {
                                true    => ListCell(box list!(AtomCell(SInt(1)))),
                                false   => ListCell(box Nil)
                            }
                        ),
                        env: self.env,
                        control: new_control,
                        dump: self.dump
                    },
                (_,_) => unimplemented!()
                }
            },
            Some((InstCell(ATOM), new_control)) => {
                let (target, new_stack) = self.stack.pop().unwrap();
                State {
                    stack: new_stack.push(
                        match target {
                            AtomCell(_) => ListCell(box list!(AtomCell(SInt(1)))),
                            _           => ListCell(box Nil)
                        }
                        ),
                    env: self.env,
                    control: new_control,
                    dump: self.dump
                }
            },
            Some((InstCell(AP), new_control @ _)) => {
                match self.stack.pop().unwrap() {
                    (ListCell(box Cons(ListCell(box func), box Cons(ListCell(params), box Nil))), new_stack) => {
                            match new_stack.pop() {
                                Some((v, newer_stack)) => State {
                                    stack: Stack::empty(),
                                    env: match v {
                                        ListCell(_) => params.push(v),
                                        _           => params.push(ListCell(box list!(v)))
                                    },
                                    control: func,
                                    dump: self.dump
                                        .push(ListCell(box newer_stack))
                                        .push(ListCell(box self.env))
                                        .push(ListCell(box new_control))
                                },/*
                                Some((v @ AtomCell(_), newer_stack)) => State {
                                    stack: Stack::empty(),
                                    env: list!( params,ListCell(box list!(v)) ),
                                    control: func,
                                    dump: self.dump
                                        .push(ListCell(box newer_stack))
                                        .push(ListCell(box self.env))
                                        .push(ListCell(box new_control))
                                },
                                Some((thing, _)) => panic!(
                                    "[fatal][AP]: Expected closure on stack, got:\n[fatal]\t{:?}\n{}",
                                    thing,
                                    prev.map_or(String::new(), |x| x.dump_state("fatal") )),*/
                                None => panic!(
                                    "[fatal][AP]: expected non-empty stack\n{}",
                                    prev.map_or(String::new(), |x| x.dump_state("fatal") ))
                            }
                    },
                    (_, thing) => panic!(
                        "[fatal][AP]: Expected closure on stack, got:\n[fatal]\t{:?}\n{}",
                        thing, prev.map_or(String::new(), |x| x.dump_state("fatal") ))
                }
            },
            Some((InstCell(RAP), new_control)) => {
                 match self.stack.pop().unwrap() {
                    (ListCell(box Cons(ListCell(box func), box Cons(ListCell(box params), box Nil))), new_stack) => {
                        match new_stack.pop() {
                            Some((v @ ListCell(_), newer_stack)) => {
                                State {
                                    stack: Stack::empty(),
                                    env: params.push(v),
                                    control: func,
                                    dump: self.dump
                                            .push(ListCell(box new_control))
                                            .push(ListCell(box self.env.pop().unwrap().1))
                                            .push(ListCell(box newer_stack))
                                }
                            },
                            Some((thing, _)) => panic!(
                                "[fatal][RAP]: Expected closure on stack, got:\n[fatal]\t{:?}\n{}",
                                thing, prev.map_or(String::new(), |x| x.dump_state("fatal") )),
                            None => panic!(
                                "[fatal][RAP]: expected non-empty stack\n{}",
                                prev.map_or(String::new(), |x| x.dump_state("fatal") ))
                        }
                    },
                    (_, thing) => panic!(
                        "[fatal][RAP]: Expected closure on stack, got:\n[fatal]\t{:?}\n{}",
                        thing, prev.map_or(String::new(), |x| x.dump_state("fatal") ))
                }
            },
            Some((InstCell(RET), _)) => {
                let (head, _) = self.stack.pop().unwrap();
                let (new_stack, new_dump) = {
                    match self.dump.pop().unwrap()  {
                        (ListCell(s), d @ _)    => (*s, d),
                        it @ (AtomCell(_),_)    => (list!(it.0), it.1),
                        _                       => panic!(
                            "[fatal][RET]: Expected non-empty stack\n{}",
                            prev.map_or(String::new(), |x| x.dump_state("fatal") ))
                    }
                };
                let (new_env, newer_dump) = {
                    match new_dump.pop().unwrap() {
                        (ListCell(e), d @ _)    => (*e, d),
                        _                       => panic!(
                            "[fatal][RET]: Expected new environment on dump stack\n{}",
                            prev.map_or(String::new(), |x| x.dump_state("fatal") ))
                    }
                };
                let (newer_control, newest_dump) = {
                    match newer_dump.pop().unwrap()  {
                        (ListCell(c), d @ _)    => (*c, d),
                        it @ (InstCell(_),_)    => (list!(it.0), it.1),
                        _                       => panic!(
                            "[fatal][RET]: Expected new control stack on dump stack\n{}",
                            prev.map_or(String::new(), |x| x.dump_state("fatal") ))
                    }
                };
                State {
                    stack: new_stack.push(head),
                    env: new_env,
                    control: newer_control,
                    dump: newest_dump
                }
            },
            Some((InstCell(DUM), new_control)) => {
                State {
                    stack: self.stack,
                    env: self.env.push(ListCell(list!())),
                    control: new_control,
                    dump: self.dump
                }
            },
            Some((InstCell(SEL), new_control)) => {
                match new_control.pop() {
                    Some((ListCell(box true_case), newer_control)) => {
                        match newer_control.pop() {
                            Some((ListCell(box false_case), newest_control)) => {
                                match self.stack.pop() {
                                    Some((ListCell(box Nil), new_stack)) => { // false case
                                        State {
                                            stack: new_stack,
                                            env: self.env,
                                            control: false_case,
                                            dump: self.dump.push(ListCell(box newest_control))
                                        }
                                    },
                                    Some((_, new_stack)) => { // true case
                                        State {
                                            stack: new_stack,
                                            env: self.env,
                                            control: true_case,
                                            dump: self.dump.push(ListCell(box newest_control))
                                        }
                                    },
                                    None => panic!(
                                        "[fatal][SEL]: expected non-empty stack\n{}",
                                        prev.map_or(String::new(), |x| x.dump_state("fatal") ))
                                }
                            },
                            Some((thing, _)) => panic!(
                                "[fatal][SEL]: expected list on control, found {:?}\n{}",
                                thing,prev.map_or(String::new(), |x| x.dump_state("fatal") )),
                            None             => panic!(
                                "[fatal][SEL]: expected list on control, found nothing\n{}",
                                prev.map_or(String::new(), |x| x.dump_state("fatal") ))
                        }
                    },
                    Some((thing, _)) => panic!(
                        "[fatal][SEL]: expected list on control, found {:?}\n{}",
                        thing,prev.map_or(String::new(), |x| x.dump_state("fatal") )),
                    None             => panic!(
                        "[fatal][SEL]: expected list on control, found nothing\n{}",
                        prev.map_or(String::new(), |x| x.dump_state("fatal") ))
                }
            },
            Some((InstCell(CAR), new_control)) => {
                match self.stack.pop() {
                    Some((ListCell(box Cons(car, _)), new_stack)) => State {
                        stack: new_stack.push(car),
                        env: self.env,
                        control: new_control,
                        dump: self.dump
                    },
                    Some((ListCell(box Nil), _)) => panic!(
                        "[fatal][CAR]: expected non-empty list, found Nil\n{}",
                        prev.map_or(String::new(), |x| x.dump_state("fatal") )),
                    Some((thing, _))             => panic!(
                        "[fatal][CAR]: expected non-empty list, found {:?}\n{}",
                        thing, prev.map_or(String::new(), |x| x.dump_state("fatal") )),
                    None                         => panic!(
                        "[fatal][CAR]: Expected non-empty list, found nothing\n{}",
                        prev.map_or(String::new(), |x| x.dump_state("fatal") ))
                }
            },
            Some((InstCell(CDR), new_control)) => {
                match self.stack.pop() {
                    Some((ListCell(box Cons(_, cdr)), new_stack)) => State {
                        stack: new_stack.push(ListCell(cdr)),
                        env: self.env,
                        control: new_control,
                        dump: self.dump
                    },
                    Some((ListCell(box Nil), _)) => panic!(
                        "[fatal][CDR]: expected non-empty list, found Nil\n{}",
                        prev.map_or(String::new(), |x| x.dump_state("fatal") )),
                    Some((thing, _))             => panic!(
                        "[fatal][CDR]: expected non-empty list, found {:?}\n{}",
                        thing, prev.map_or(String::new(), |x| x.dump_state("fatal") )),
                    None                        => panic!(
                        "[fatal][CDR]: Expected non-empty list, found nothing\n{}",
                        prev.map_or(String::new(), |x| x.dump_state("fatal") ))
                }
            },
            Some((InstCell(CONS), new_control)) => {
                match self.stack.pop() {
                    Some((thing, new_stack)) => {
                        match new_stack.pop() {
                            Some((ListCell(list), newer_stack)) => {
                                State {
                                    stack: newer_stack.push(ListCell(box Cons(thing, list))),
                                    env: self.env,
                                    control: new_control,
                                    dump: self.dump
                                }
                            },
                            Some((thing_else, _)) => panic!(
                                "[fatal][CONS]: Expected a list on the stack, found {:?}\n{}",
                                thing_else,
                                prev.map_or(String::new(), |x| x.dump_state("fatal") )),
                            None  => panic!(
                                "[fatal][CONS]: Expected a list on the stack, found nothing.\n{}",
                                prev.map_or(String::new(), |x| x.dump_state("fatal") ))
                        }
                    },
                    None => panic!(
                        "[fatal][CONS]: Expected an item on the stack, found nothing\n{}",
                        prev.map_or(String::new(), |x| x.dump_state("fatal") ))
                }
            },
            Some((InstCell(NULL), new_control)) => {
                let (target, new_stack) = self.stack.pop().unwrap();
                State {
                    stack: new_stack.push(
                        match target {
                            ListCell(box Nil) => ListCell(box list!(AtomCell(SInt(1)))),
                            _                 => ListCell(box Nil)
                        }
                        ),
                    env: self.env,
                    control: new_control,
                    dump: self.dump
                }
            },
            Some((InstCell(WRITEC), new_control)) => {
                match self.stack.pop() {
                    Some((AtomCell(Char(ch)), new_stack)) => {
                        if let Err(msg) = outp.write(&[ch as u8,1]) {
                            panic!("[fatal][WRITEC]: writing failed: {:?}\n{}",
                                msg,
                                prev.map_or(String::new(), |x| x.dump_state("fatal") ))
                        };
                        State {
                            stack: new_stack,
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        }
                    },
                    Some((thing_else,_)) => panic!(
                        "[fatal][WRITEC]: expected char, found {:?}\n{}",
                        thing_else,prev.map_or(String::new(), |x| x.dump_state("fatal") )),
                    None => panic!(
                        "[fatal][WRITEC]: expected char, found nothing\n{}",
                        prev.map_or(String::new(), |x| x.dump_state("fatal") ))
                }
            },
            Some((InstCell(READC), new_control)) => {
                let mut buf: [u8;1] = [0;1];
                match inp.read(&mut buf) {
                    Ok(_) => State {
                        stack: self.stack.push(AtomCell(Char(buf[0] as char))),
                        env: self.env,
                        control: new_control,
                        dump: self.dump
                    },
                    Err(msg) => panic!(
                        "[fatal][READC]: could not read, {:?}\n{}",
                        msg,prev.map_or(String::new(), |x| x.dump_state("fatal") ))
                }
            }
            Some((InstCell(STOP), _)) => panic!(
                    "[fatal]: undefined behaviour\n[fatal]: evaluation of STOP word\n{}",
                    prev.map_or(String::new(), |x| x.dump_state("fatal") )
                    ),
            None => panic!( // this should never happen (barring force majeure)
                "[fatal]: expected an instruction on control stack\n{}",
                prev.map_or(String::new(), |x| x.dump_state("fatal") )),
            Some((thing, _)) => panic!(
                    "[fatal]: Tried to evaluate an unsupported cell type {:?}.\n{}",
                    thing,
                    prev.map_or(String::new(), |x| x.dump_state("fatal") ))
        }
    }
}


/// Evaluates a program.
///
/// Evaluates a program (control stack) and returns the final state.
/// TODO: add (optional?) parameters for stdin and stdout
#[stable(feature="vm_core",since="0.2.0")]
pub fn eval_program(program: List<SVMCell>, debug: bool) -> List<SVMCell> {
    let mut machine = State {
        stack:      Stack::empty(),
        env:        Stack::empty(),
        control:    program,
        dump:       Stack::empty()
    };
    let mut outp = io::stdout();
    let mut inp  = io::stdin();
    // while there are more instructions,
    while {
        machine.control.length() > 0usize &&
        machine.control.peek()!= Some(&InstCell(STOP))
    } {  //TODO: this is kinda heavyweight
        machine = machine.eval(&mut inp, &mut outp, debug) // continue evaling
    };
    machine.stack
}
