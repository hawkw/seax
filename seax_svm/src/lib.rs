#![crate_name = "seax_svm"]
#![stable(feature="vm_core", since="0.1.2")]
#![crate_type = "lib"]
#![feature(box_syntax,box_patterns)]
#![feature(staged_api)]
#![staged_api]

#[macro_use]
extern crate log;

extern crate byteorder;

/// Singly-linked list and stack implementations.
///
/// `List<T>` is a singly-linked `cons` list.
/// `Stack<T>` is a trait providing stack operations(`push()`, `pop()`, and
/// `peek()`), and an implementation for `List`.
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

// Reexports
pub use self::slist::List;
pub use self::slist::List::{Cons,Nil};
pub use self::slist::Stack;
pub use self::cell::{SVMCell,Atom,Inst};

use self::cell::SVMCell::*;
use self::cell::Atom::*;
use self::cell::Inst::*;

/// Represents a SVM machine state
#[derive(PartialEq,Clone,Debug)]
#[stable(feature="vm_core", since="0.1.0")]
pub struct State {
    stack:  List<SVMCell>,
    env:    List<SVMCell>,
    control:List<SVMCell>,
    dump:   List<SVMCell>
}

/// A VM state's IO action
///
/// Take note that this will eventually be replaced with memory-mapped IO
/// when the main memory management scheme is done; therefore, it should
/// never be marked as stable. Consider this struct and anything that depends
/// on it to be an ugly hack.
#[derive(PartialEq,Clone,Debug)]
#[unstable(feature="eval")]
pub enum IOEvent {
    /// A character was requested from the buffer
    Req,
    /// A character was buffered
    Buf(char)
}

#[unstable(feature="eval")]
pub type EvalResult = Result<(State,Option<IOEvent>), String>;

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
    #[stable(feature="vm_core", since="0.3.0")]
    pub fn eval(self,
                input: Option<u8>,
                debug: bool)
                -> EvalResult {
        debug!("[eval]: Evaluating {:?}", self.control);
        // TODO: this (by which I mean "the whole caching deal") could likely be made
        // better and/or faster with some clever (mis?)use of RefCell; look into that.
        let mut prev = if debug { Some(self.clone()) } else { None };
        // in ths pattern match, we begin The Great Work
        match self.control.pop().unwrap() {
            // NIL: pop an empty list onto the stack
            (InstCell(NIL), new_control) => Ok((State {
                stack: self.stack.push(ListCell(box List::new())),
                env: self.env,
                control: new_control,
                dump: self.dump
            }, None)),
            // LDC: load constant
            (InstCell(LDC), new_control) => {
                let (atom,newer_control) = try!(new_control.pop().ok_or(
                    format!("[fatal][LDC]: pop on empty stack\n{}",
                        prev.take().map_or(String::new(), |x| x.dump_state("fatal") ))) );
                Ok((State {
                    stack: self.stack.push(atom),
                    env: self.env,
                    control: newer_control,
                    dump: self.dump
                }, None))
            },
            // LD: load variable
            (InstCell(LD), new_control) => match new_control.pop() {
                Some((ListCell(
                    box Cons(AtomCell(UInt(lvl)),
                    box Cons(AtomCell(UInt(idx)),
                    box Nil))
                    ), newer_control)) => match self.env[(lvl-1)] {
                        ListCell(ref level) => Ok((State {
                            stack: match level.get(idx-1) {
                                Some(thing) => self.stack.push(thing.clone()),
                                None        => self.stack
                            },
                            env: self.env.clone(),
                            control: newer_control,
                            dump: self.dump
                        }, None)),
                        // This is a special case for something that, as far as I know,
                        // should never happen. But despite everything, it DOES happen.
                        ref thing @ AtomCell(_) => Ok((State {
                        // I give up. Have your special case.
                            stack: self.stack.push(thing.clone()),
                            env: self.env.clone(),
                            control: newer_control,
                            dump: self.dump
                        }, None)),
                        _ => Err(format!(
                            "[fatal][LD]: expected list in $e, found {:?}\n{}",
                            self.env[lvl-1], prev.map_or(String::new(), |x| x.dump_state("fatal") )))
                },
               Some((ListCell( // TODO: this uses deprecated signed int indexing, remove
                    box Cons(AtomCell(SInt(lvl)),
                    box Cons(AtomCell(SInt(idx)),
                    box Nil))
                    ), newer_control)) =>  match self.env[(lvl-1)] {
                        SVMCell::ListCell(ref level) => Ok((State {
                            stack: self.stack.push(level[(idx-1)].clone()),
                            env: self.env.clone(),
                            control: newer_control,
                            dump: self.dump
                        }, None)),
                        _ => Err(format!(
                            "[fatal][LD]: expected list in $e, found {:?}\n{}",
                            self.env[lvl-1], prev.map_or(String::new(), |x| x.dump_state("fatal") )))
                },
               Some((thing,newer_control)) => Err(format!(
                    "[fatal][LD]: expected pair, found {:?}\n[fatal] new control: {:?}\n{}",
                    thing,
                    newer_control,
                    prev.map_or(String::new(), |x| x.dump_state("fatal") ))),
               None => Err(format!(
                    "[fatal][LD]: expected pair, found empty stack\n{}",
                    prev.map_or(String::new(), |x| x.dump_state("fatal") )))
            },

            // LDF: load function
            (InstCell(LDF), new_control) => {
                let (func, newer_control) = try!(match new_control.pop() {
                    Some(thing) => Ok(thing),
                    None        => Err(format!(
                        "[fatal][LDF]: pop on empty control stack\n{}",
                        prev.map_or(String::new(), |x| x.dump_state("fatal") )))
                });
                Ok((State {
                    stack: self.stack.push(ListCell( box list!(
                        func,
                            self.env
                                .get(0)
                                .map_or(ListCell(box Nil), |it| it.clone())) )),
                    env: self.env,
                    control: newer_control,
                    dump: self.dump
                }, None))
            },

            (InstCell(JOIN), new_control) => {
                let (top, new_dump) = try!(match self.dump.pop() {
                    Some(thing) => Ok(thing),
                    None        => Err(format!(
                        "[fatal][JOIN]: pop on empty dump stack") )
                });
                match top {
                    ListCell(box Nil) => Ok((State {
                        stack: self.stack,
                        env: self.env,
                        control: new_control,
                        dump: new_dump
                    }, None)),
                    ListCell(box it)  => Ok((State {
                        stack: self.stack,
                        env: self.env,
                        control: it,
                        dump: new_dump
                    }, None)),
                    anything          => Err(format!(
                        "[fatal][JOIN]: expected list on dump, found {:?}\n{}",
                        anything, prev.map_or(String::new(), |x| x.dump_state("fatal") )) )
                }
            },
            (InstCell(ADD), new_control) => match self.stack.pop() {
                Some((AtomCell(op1), new_stack)) => match new_stack.pop() {
                    Some((AtomCell(op2), newer_stack)) => Ok((State {
                            stack: newer_stack.push(AtomCell(op1 + op2)),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        }, None)),
                    any => Err(format!(
                        "[fatal][ADD]: expected second operand, found {:?}\n{}",
                        any,
                        prev.map_or(String::new(), |x| x.dump_state("fatal") )) )
                    },
                any => Err(format!(
                    "[fatal][ADD]: expected first operand, found {:?}\n{}",
                    any,
                    prev.map_or(String::new(), |x| x.dump_state("fatal") )) )
            },
            (InstCell(SUB), new_control) => match self.stack.pop() {
                Some((AtomCell(op1), new_stack)) => match new_stack.pop() {
                    Some((AtomCell(op2), newer_stack)) => Ok((State {
                            stack: newer_stack.push(AtomCell(op1 - op2)),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        }, None)),
                    any => Err(format!(
                        "[fatal][SUB]: expected second operand, found {:?}\n{}",
                        any,
                        prev.map_or(String::new(), |x| x.dump_state("fatal") )) )
                    },
                any => Err(format!(
                    "[fatal][SUB]: expected first operand, found {:?}\n{}",
                    any,
                    prev.map_or(String::new(), |x| x.dump_state("fatal") )) )
            },
            (InstCell(FDIV), new_control) => {
                let (op1, new_stack) = try!(match self.stack.pop() {
                    Some(thing) => Ok(thing),
                    None        => Err(format!(
                        "[fatal][FDIV]: pop on empty stack\n{}",
                        prev.map_or(String::new(), |x| x.dump_state("fatal") )) )
                });
                match op1 {
                    AtomCell(a) => {
                        let (op2, newer_stack) = try!(match new_stack.pop() {
                            Some(thing) => Ok(thing),
                            None        => Err("[fatal][FDIV]: pop on empty stack")
                        });
                        match op2 {
                            AtomCell(b) => Ok((State {
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
                            }, None)),
                            b => Err(format!(
                                "[fatal][FDIV]: TypeError: expected compatible operands, found (FDIV {:?} {:?})", a, b) )
                        }
                    },
                    _ => Err(format!(
                            "[fatal][FDIV]: Expected first operand to be atom, found list or instruction" )),
                }
            },
            (InstCell(DIV), new_control) => match self.stack.pop() {
                Some((AtomCell(op1), new_stack)) => match new_stack.pop() {
                    Some((AtomCell(op2), newer_stack)) => Ok((State {
                            stack: newer_stack.push(AtomCell(op1 / op2)),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },None)),
                    any => Err(format!(
                        "[fatal][DIV]: expected second operand, found {:?}\n{}",
                        any,
                        prev.map_or(String::new(), |x| x.dump_state("fatal") )) )
                    },
                any => Err(format!(
                    "[fatal][DIV]: expected first operand, found {:?}\n{}",
                    any,
                    prev.map_or(String::new(), |x| x.dump_state("fatal") )) )
            },
            (InstCell(MUL), new_control) => match self.stack.pop() {
                Some((AtomCell(op1), new_stack)) => match new_stack.pop() {
                    Some((AtomCell(op2), newer_stack)) => Ok((State {
                            stack: newer_stack.push(AtomCell(op1 * op2)),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        }, None)),
                    any => Err(format!(
                        "[fatal][MUL]: expected second operand, found {:?}\n{}",
                        any,
                        prev.map_or(String::new(), |x| x.dump_state("fatal") )) )
                    },
                any => Err(format!(
                    "[fatal][MUL]: expected first operand, found {:?}\n{}",
                    any,
                    prev.map_or(String::new(), |x| x.dump_state("fatal") )) )
            },
            (InstCell(MOD), new_control) => match self.stack.pop() {
                Some((AtomCell(op1), new_stack)) => match new_stack.pop() {
                    Some((AtomCell(op2), newer_stack)) => Ok((State {
                            stack: newer_stack.push(AtomCell(op1 % op2)),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        }, None)),
                    any => Err(format!(
                        "[fatal][MOD]: expected second operand, found {:?}\n{}",
                        any,
                        prev.map_or(String::new(), |x| x.dump_state("fatal") )) )
                    },
                any => Err(format!(
                    "[fatal][MOD]: expected first operand, found {:?}\n{}",
                    any,
                    prev.map_or(String::new(), |x| x.dump_state("fatal") )) )
            },
            (InstCell(EQ), new_control) => {
                let (op1, new_stack) = self.stack.pop().unwrap();
                let (op2, newer_stack) = new_stack.pop().unwrap();
                match (op1,op2) {
                    (AtomCell(a), AtomCell(b)) => Ok((State {
                        stack: newer_stack.push(
                            match a == b {
                                true    => ListCell(box list!(AtomCell(SInt(1)))),
                                false   => ListCell(box Nil)
                            }),
                        env: self.env,
                        control: new_control,
                        dump: self.dump
                    }, None)),
                    (_,_) => unimplemented!() // TODO: sane error pls
                }
            },
            (InstCell(GT), new_control) => {
                let (op1, new_stack) = self.stack.pop().unwrap();
                let (op2, newer_stack) = new_stack.pop().unwrap();
                match (op1,op2) {
                    (AtomCell(a), AtomCell(b)) => Ok((State {
                        stack: newer_stack.push(
                            match a > b {
                                true    => ListCell(box list!(AtomCell(SInt(1)))),
                                false   => ListCell(box Nil)
                            }
                        ),
                        env: self.env,
                        control: new_control,
                        dump: self.dump
                    }, None)),
                (_,_) => unimplemented!()
                }
            },
            (InstCell(GTE), new_control) => {
                let (op1, new_stack) = self.stack.pop().unwrap();
                let (op2, newer_stack) = new_stack.pop().unwrap();
                match (op1,op2) {
                    (AtomCell(a), AtomCell(b)) => Ok((State {
                        stack: newer_stack.push(
                            match a >= b {
                                true    => ListCell(box list!(AtomCell(SInt(1)))),
                                false   => ListCell(box Nil)
                            }
                        ),
                        env: self.env,
                        control: new_control,
                        dump: self.dump
                    }, None)),
                (_,_) => unimplemented!()
                }
            },
            (InstCell(LT), new_control) => {
                let (op1, new_stack) = self.stack.pop().unwrap();
                let (op2, newer_stack) = new_stack.pop().unwrap();
                match (op1,op2) {
                    (AtomCell(a), AtomCell(b)) => Ok((State {
                        stack: newer_stack.push(
                            match a < b {
                                true    => ListCell(box list!(AtomCell(SInt(1)))),
                                false   => ListCell(box Nil)
                            }
                        ),
                        env: self.env,
                        control: new_control,
                        dump: self.dump
                    }, None)),
                (_,_) => unimplemented!()
                }
            },
            (InstCell(LTE), new_control) => {
                let (op1, new_stack) = self.stack.pop().unwrap();
                let (op2, newer_stack) = new_stack.pop().unwrap();
                match (op1,op2) {
                    (AtomCell(a), AtomCell(b)) => Ok((State {
                        stack: newer_stack.push(
                            match a <= b {
                                true    => ListCell(box list!(AtomCell(SInt(1)))),
                                false   => ListCell(box Nil)
                            }),
                        env: self.env,
                        control: new_control,
                        dump: self.dump
                    }, None)),
                (_,_) => unimplemented!()
                }
            },
            (InstCell(ATOM), new_control) => {
                let (target, new_stack) = self.stack.pop().unwrap();
                Ok((State {
                    stack: new_stack.push(
                        match target {
                            AtomCell(_) => ListCell(box list!(AtomCell(SInt(1)))),
                            _           => ListCell(box Nil)
                        }
                        ),
                    env: self.env,
                    control: new_control,
                    dump: self.dump
                },None))
            },
            (InstCell(AP), new_control) => match self.stack.pop().unwrap() {
                (ListCell(box Cons(ListCell(box func), box Cons(ListCell(params), box Nil))), new_stack) => {
                        match new_stack.pop() {
                            Some((v, newer_stack)) => Ok((State {
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
                            }, None)),/*
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
                            None => Err(format!(
                                "[fatal][AP]: expected non-empty stack\n{}",
                                prev.map_or(String::new(), |x| x.dump_state("fatal") )) )
                        }
                },
                (_, thing) => Err(format!(
                    "[fatal][AP]: Expected closure on stack, got:\n[fatal]\t{:?}\n{}",
                    thing, prev.map_or(String::new(), |x| x.dump_state("fatal") )) )
            },
            (InstCell(RAP), new_control) => match self.stack.pop().unwrap() {
                (ListCell(box Cons(ListCell(box func), box Cons(ListCell(box params), box Nil))), new_stack) => {
                    match new_stack.pop() {
                        Some((v @ ListCell(_), newer_stack)) => Ok(( State {
                            stack: Stack::empty(),
                            env: params.push(v),
                            control: func,
                            dump: self.dump
                                    .push(ListCell(box new_control))
                                    .push(ListCell(box self.env.pop().unwrap().1))
                                    .push(ListCell(box newer_stack))
                        }, None)),
                        Some((thing, _)) => Err(format!(
                            "[fatal][RAP]: Expected closure on stack, got:\n[fatal]\t{:?}\n{}",
                            thing, prev.map_or(String::new(), |x| x.dump_state("fatal") )) ),
                        None => Err(format!(
                            "[fatal][RAP]: expected non-empty stack\n{}",
                            prev.map_or(String::new(), |x| x.dump_state("fatal") )) )
                    }
                },
                (_, thing) => Err(format!(
                    "[fatal][RAP]: Expected closure on stack, got:\n[fatal]\t{:?}\n{}",
                    thing, prev.map_or(String::new(), |x| x.dump_state("fatal") )) )
            },
            (InstCell(RET), _) => {
                let (head, _) = self.stack.pop().unwrap();
                let (new_stack, new_dump) = try!(match self.dump.pop()  {
                    Some((ListCell(s), d))      => Ok((*s, d)),
                    Some(it @ (AtomCell(_),_))  => Ok((list!(it.0), it.1)),
                    _                           => Err(
                        "[fatal][RET]: Expected non-empty stack")
                });
                let (new_env, newer_dump) = try!(match new_dump.pop() {
                    Some((ListCell(e), d))  => Ok((*e, d)),
                    _                       => Err(
                        "[fatal][RET]: Expected new environment on dump stack")
                });
                let (newer_control, newest_dump) = try!(match newer_dump.pop() {
                    Some((ListCell(c), d))      => Ok((*c, d)),
                    Some(it @ (InstCell(_),_))  => Ok((list!(it.0), it.1)),
                    _                           => Err(
                        "[fatal][RET]: Expected new control stack on dump stack")
                });
                Ok((State {
                    stack: new_stack.push(head),
                    env: new_env,
                    control: newer_control,
                    dump: newest_dump
                }, None))
            },
            (InstCell(DUM), new_control) => Ok((State {
                stack: self.stack,
                env: self.env.push(ListCell(list!())),
                control: new_control,
                dump: self.dump
            }, None)),
            (InstCell(SEL), new_control) => match new_control.pop() {
                Some((ListCell(box true_case), newer_control)) => {
                    match newer_control.pop() {
                        Some((ListCell(box false_case), newest_control)) => {
                            match self.stack.pop() {
                                // False case
                                Some((ListCell(box Nil), new_stack)) => Ok((State {
                                    stack: new_stack,
                                    env: self.env,
                                    control: false_case,
                                    dump: self.dump.push(ListCell(box newest_control))
                                }, None)),
                                // True case
                                Some((_, new_stack)) => Ok((State {
                                    stack: new_stack,
                                    env: self.env,
                                    control: true_case,
                                    dump: self.dump.push(ListCell(box newest_control))
                                }, None)),
                                None => Err(format!(
                                    "[fatal][SEL]: expected non-empty stack\n{}",
                                    prev.map_or(String::new(), |x| x.dump_state("fatal") )) )
                            }
                        },
                        Some((thing, _)) => Err(format!(
                            "[fatal][SEL]: expected list on control, found {:?}\n{}",
                            thing,prev.map_or(String::new(), |x| x.dump_state("fatal") )) ),
                        None             => Err(format!(
                            "[fatal][SEL]: expected list on control, found nothing\n{}",
                            prev.map_or(String::new(), |x| x.dump_state("fatal") )) )
                    }
                },
                Some((thing, _)) => Err(format!(
                    "[fatal][SEL]: expected list on control, found {:?}\n{}",
                    thing,prev.map_or(String::new(), |x| x.dump_state("fatal") )) ),
                None             => Err(format!(
                    "[fatal][SEL]: expected list on control, found nothing\n{}",
                    prev.map_or(String::new(), |x| x.dump_state("fatal") )) )
            },
            (InstCell(CAR), new_control) => match self.stack.pop() {
                Some((ListCell(box Cons(car, _)), new_stack)) => Ok(( State {
                    stack: new_stack.push(car),
                    env: self.env,
                    control: new_control,
                    dump: self.dump
                }, None)),
                Some((ListCell(box Nil), _)) => Err(format!(
                    "[fatal][CAR]: expected non-empty list, found Nil\n{}",
                    prev.map_or(String::new(), |x| x.dump_state("fatal") )) ),
                Some((thing, _))             => Err(format!(
                    "[fatal][CAR]: expected non-empty list, found {:?}\n{}",
                    thing, prev.map_or(String::new(), |x| x.dump_state("fatal") )) ),
                None                         => Err(format!(
                    "[fatal][CAR]: Expected non-empty list, found nothing\n{}",
                    prev.map_or(String::new(), |x| x.dump_state("fatal") )) )
            },
            (InstCell(CDR), new_control) => match self.stack.pop() {
                Some((ListCell(box Cons(_, cdr)), new_stack)) => Ok((State {
                    stack: new_stack.push(ListCell(cdr)),
                    env: self.env,
                    control: new_control,
                    dump: self.dump
                }, None)),
                Some((ListCell(box Nil), _)) => panic!(
                    "[fatal][CDR]: expected non-empty list, found Nil\n{}",
                    prev.map_or(String::new(), |x| x.dump_state("fatal") )),
                Some((thing, _))             => panic!(
                    "[fatal][CDR]: expected non-empty list, found {:?}\n{}",
                    thing, prev.map_or(String::new(), |x| x.dump_state("fatal") )),
                None                        => panic!(
                    "[fatal][CDR]: Expected non-empty list, found nothing\n{}",
                    prev.map_or(String::new(), |x| x.dump_state("fatal") ))
            },
            (InstCell(CONS), new_control) => match self.stack.pop() {
                Some((thing, new_stack)) => {
                    match new_stack.pop() {
                        Some((ListCell(list), newer_stack)) => Ok((State {
                            stack: newer_stack.push(ListCell(box Cons(thing, list))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        }, None)),
                        Some((thing_else, _)) => Err(format!(
                            "[fatal][CONS]: Expected a list on the stack, found {:?}\n{}",
                            thing_else,
                            prev.map_or(String::new(), |x| x.dump_state("fatal") )) ),
                        None  => Err(format!(
                            "[fatal][CONS]: Expected a list on the stack, found nothing.\n{}",
                            prev.map_or(String::new(), |x| x.dump_state("fatal") )) )
                    }
                },
                None => Err(format!(
                    "[fatal][CONS]: Expected an item on the stack, found nothing\n{}",
                    prev.map_or(String::new(), |x| x.dump_state("fatal") )) )
            },
            (InstCell(NULL), new_control) => {
                let (target, new_stack) = self.stack.pop().unwrap();
                Ok((State {
                    stack: new_stack.push(
                        match target {
                            ListCell(box Nil) => ListCell(box list!(AtomCell(SInt(1)))),
                            _                 => ListCell(box Nil)
                        }
                        ),
                    env: self.env,
                    control: new_control,
                    dump: self.dump
                }, None))
            },
            (InstCell(WRITEC), new_control) => match self.stack.pop() {
                Some((AtomCell(Char(ch)), new_stack)) => {/*
                    if let Err(msg) = outp.write(&[ch as u8,1]) {
                        panic!("[fatal][WRITEC]: writing failed: {:?}\n{}",
                            msg,
                            prev.map_or(String::new(), |x| x.dump_state("fatal") ))
                    };*/
                    Ok((State {
                        stack: new_stack,
                        env: self.env,
                        control: new_control,
                        dump: self.dump
                    }, Some(IOEvent::Buf(ch))) )
                },
                Some((thing_else,_)) => panic!(
                    "[fatal][WRITEC]: expected char, found {:?}\n{}",
                    thing_else,prev.map_or(String::new(), |x| x.dump_state("fatal") )),
                None => panic!(
                    "[fatal][WRITEC]: expected char, found nothing\n{}",
                    prev.map_or(String::new(), |x| x.dump_state("fatal") ))
            },
            (InstCell(READC), new_control) => {
                // todo: figure out how to make it work with the new thing
                match input {
                    Some(ch) => Ok((State {
                        stack: self.stack.push(AtomCell(Char(ch as char))),
                        env: self.env,
                        control: new_control,
                        dump: self.dump
                    }, None)),
                    _       => panic!("No input, something went wrong (this is not supposed to happen")
                } /*,
                    .map_err(|msg| format!(
                        "[fatal][READC]: could not read, {:?}\n{}",
                        msg,prev.map_or(String::new(), |x| x.dump_state("fatal") )))*/
            },
            (InstCell(STOP), _) => panic!(
                "[fatal]: undefined behaviour\n[fatal]: evaluation of STOP word\n{}",
                prev.map_or(String::new(), |x| x.dump_state("fatal") )
                ),
            (thing, _) => panic!(
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
pub fn eval_program(program: List<SVMCell>,
                    debug: bool)
    -> Result<List<SVMCell>,String> {
    debug!("evaluating {:?}", program);
    let mut machine = State {
        stack:      Stack::empty(),
        env:        Stack::empty(),
        control:    program,
        dump:       Stack::empty()
    };
    // while there are more instructions,
    while {
        machine.control.length() > 0usize &&
        machine.control.peek()!= Some(&InstCell(STOP))
    } {  //TODO: this is kinda heavyweight
        machine = try!(machine.eval(None,debug)).0 // continue evaling
    };
    Ok(machine.stack)
}
