#![crate_name = "seax_svm"]
#![crate_type = "lib"]
#![feature(box_syntax)]
#![feature(core)]

/// Contains the Seax Virtual Machine (SVM) and miscellaneous
/// support code.
pub mod svm {
    pub use self::slist::List;
    pub use self::slist::List::{Cons,Nil};
    pub use self::slist::Stack;
    pub use self::cell::{SVMCell,Atom,Inst};
    use self::cell::SVMCell::*;
    use self::cell::Atom::*;
    use self::cell::Inst::*;

    /// Singly-linked list and stack implementations.
    ///
    /// `List<T>` is a singly-linked cons list with boxed items. `Stack<T>` is
    ///  defined as a trait providing stack operations(`push()`, `pop()`, and
    ///  `peek()`), and an implementation for `List`.
    #[macro_use]
    pub mod slist;

    /// SVM cell types.
    ///
    /// A cell in the VM can be either an atom (single item, either unsigned
    /// int, signed int, float, or string), a pointer to a list cell, or an
    /// instruction.
    pub mod cell;

    /// Represents a SVM machine state
    pub struct State {
        stack:  List<SVMCell>,
        env:  List<SVMCell>,
        control:  List<SVMCell>,
        dump:  List<SVMCell>
    }

    impl State {

        /// Creates a new empty state
        pub fn new() -> State {
            State {
                stack: Stack::empty(),
                env: Stack::empty(),
                control: Stack::empty(),
                dump: Stack::empty()
            }
        }

        /// Evaluates an instruction.
        ///
        /// Evaluates an instruction against a state, returning a new state.
        /// TODO: rewrite me to use the next instruction on the control stack,
        /// rather than a parameter.
        pub fn eval(self) -> State {
            let (next, new_control) = self.control.pop().unwrap();
            match next {
                // NIL: pop an empty list onto the stack
                InstCell(NIL) => {
                    State {
                        stack: self.stack.push(ListCell(box List::new())),
                        env: self.env,
                        control: new_control,
                        dump: self.dump
                    }
                }
                // LDC: load constant
                InstCell(LDC) => {
                    let (atom,newer_control) = new_control.pop().unwrap();
                    State {
                        stack: self.stack.push(atom),
                        env: self.env,
                        control: newer_control,
                        dump: self.dump
                    }
                },
                // LD: load variable
               InstCell(LD) => {
                    let (top, newer_control) = new_control.pop().unwrap();
                    match top {
                       ListCell(
                            box Cons(AtomCell(SInt(level)),
                            box Cons(AtomCell(SInt(pos)),
                            box Nil))
                        ) => {
                            let environment = match self.env[level-1] {
                                SVMCell::ListCell(ref l) => l.clone(),
                                _ => panic!("[LD]: Fatal: expected list in $e, found {:?}",self.env[level-1])
                            };
                            State {
                                stack: self.stack.push(environment[pos-1].clone()),
                                env: self.env,
                                control: newer_control,
                                dump: self.dump
                            }
                        },
                        _ => panic!("[LD] Fatal: expected pair, found {:?}", top)
                    }
                },

                // LDF: load function
                InstCell(LDF) => {
                    let (func, newer_control) = new_control.pop().unwrap();
                    State {
                        stack: self.stack.push(ListCell(box list!(func,self.env[1is].clone()))),
                        env: self.env,
                        control: newer_control,
                        dump: self.dump
                    }
                },

                InstCell(JOIN) => {
                    let (top, new_dump) = self.dump.pop().unwrap();
                    State {
                        stack: self.stack,
                        env: self.env,
                        control: new_control.push(top),
                        dump: new_dump
                    }
                },
                InstCell(ADD) => {
                    let (op1, new_stack) = self.stack.pop().unwrap();
                    match op1 {
                        AtomCell(a) => {
                            let (op2, newer_stack) = new_stack.pop().unwrap();
                            match op2 {
                                AtomCell(b) => State {
                                    stack: newer_stack.push(AtomCell(a + b)),
                                    env: self.env,
                                    control: new_control,
                                    dump: self.dump
                                },
                                b => panic!("[ADD] TypeError: expected compatible operands, found (ADD {:?} {:?})", a, b)
                            }
                        },
                        _ => panic!("[ADD]: Expected first operand to be atom, found list or instruction"),
                    }
                },
                InstCell(SUB) => {
                    let (op1, new_stack) = self.stack.pop().unwrap();
                    match op1 {
                        AtomCell(a) => {
                            let (op2, newer_stack) = new_stack.pop().unwrap();
                            match op2 {
                                AtomCell(b) => State {
                                    stack: newer_stack.push(AtomCell(a - b)),
                                    env: self.env,
                                    control: new_control,
                                    dump: self.dump
                                },
                                b => panic!("[SUB] TypeError: expected compatible operands, found (SUB {:?} {:?})", a, b)
                            }
                        },
                        _ => panic!("[SUB]: Expected first operand to be atom, found list or instruction"),
                    }
                },
                InstCell(FDIV) => {
                    let (op1, new_stack) = self.stack.pop().unwrap();
                    match op1 {
                        AtomCell(a) => {
                            let (op2, newer_stack) = new_stack.pop().unwrap();
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
                                            (Float(a), Char(b))     => Float(a as f64 / b as u8 as f64),
                                            (_, _)                  => panic!("TypeError: Unsupported operands {:?} * {:?}", a,b)
                                        }
                                        )),
                                    env: self.env,
                                    control: new_control,
                                    dump: self.dump
                                },
                                b => panic!("[FDIV] TypeError: expected compatible operands, found (DIV {:?} {:?})", a, b)
                            }
                        },
                        _ => panic!("[FDIV]: Expected first operand to be atom, found list or instruction"),
                    }
                },
                InstCell(DIV) => {
                    let (op1, new_stack) = self.stack.pop().unwrap();
                    match op1 {
                        AtomCell(a) => {
                            let (op2, newer_stack) = new_stack.pop().unwrap();
                            match op2 {
                                AtomCell(b) => State {
                                    stack: newer_stack.push(AtomCell(a / b)),
                                    env: self.env,
                                    control: new_control,
                                    dump: self.dump
                                },
                                b => panic!("[DIV] TypeError: expected compatible operands, found (DIV {:?} {:?})", a, b)
                            }
                        },
                        _ => panic!("[DIV]: Expected first operand to be atom, found list or instruction"),
                    }
                },
                InstCell(MUL) => {
                    let (op1, new_stack) = self.stack.pop().unwrap();
                    match op1 {
                        AtomCell(a) => {
                            let (op2, newer_stack) = new_stack.pop().unwrap();
                            match op2 {
                                AtomCell(b) => State {
                                    stack: newer_stack.push(AtomCell(a * b)),
                                    env: self.env,
                                    control: new_control,
                                    dump: self.dump
                                },
                                b => panic!("[MUL] TypeError: expected compatible operands, found (MUL {:?} {:?})", a, b)
                            }
                        },
                        _ => panic!("[MUL]: Expected first operand to be atom, found list or instruction"),
                    }
                },
                InstCell(MOD) => {
                    let (op1, new_stack) = self.stack.pop().unwrap();
                    match op1 {
                        AtomCell(a) => {
                            let (op2, newer_stack) = new_stack.pop().unwrap();
                            match op2 {
                                AtomCell(b) => State {
                                    stack: newer_stack.push(AtomCell(a % b)),
                                    env: self.env,
                                    control: new_control,
                                    dump: self.dump
                                },
                                b => panic!("[MOD] TypeError: expected compatible operands, found (MOD {:?} {:?})", a, b)
                            }
                        },
                        _ => panic!("[MOD]: Expected first operand to be atom, found list or instruction"),
                    }
                },
                InstCell(EQ) => {
                    let (op1, new_stack) = self.stack.pop().unwrap();
                    let (op2, newer_stack) = new_stack.pop().unwrap();
                    match (op1,op2) {
                        (AtomCell(a), AtomCell(b)) => State {
                            stack: newer_stack.push(AtomCell(Bool(a == b))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                    (_,_) => unimplemented!()
                    }
                },
                InstCell(GT) => {
                    let (op1, new_stack) = self.stack.pop().unwrap();
                    let (op2, newer_stack) = new_stack.pop().unwrap();
                    match (op1,op2) {
                        (AtomCell(a), AtomCell(b)) => State {
                            stack: newer_stack.push(AtomCell(Bool(a > b))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                    (_,_) => unimplemented!()
                    }
                },
                InstCell(GTE) => {
                    let (op1, new_stack) = self.stack.pop().unwrap();
                    let (op2, newer_stack) = new_stack.pop().unwrap();
                    match (op1,op2) {
                        (AtomCell(a), AtomCell(b)) => State {
                            stack: newer_stack.push(AtomCell(Bool(a >= b))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                    (_,_) => unimplemented!()
                    }
                },
                _ => { unimplemented!() }
            }
        }
    }

    /*
    /// Evaluates a program.
    ///
    /// Evaluates a program represented as an `Iterator` of `Inst`s.
    /// Returns the final machine state at the end of execution

    pub fn evalProgram(insts: Iterator<Item=Inst>) -> State {
        insts.fold(State::new(), |last_state: State, inst: Inst| last_state.eval(inst));
    }*/

    #[cfg(test)]
    mod tests {
        use super::slist::Stack;
        use super::slist::List::{Cons,Nil};
        use super::State;
        use super::cell::Atom::*;
        use super::cell::SVMCell::*;
        use super::Inst::*;

        #[test]
        fn test_empty_state() {
            let state = State::new();
            assert_eq!(state.stack.length(), 0);
            assert_eq!(state.env.length(), 0);
            assert_eq!(state.control.length(), 0);
            assert_eq!(state.dump.length(), 0);
        }

        #[test]
        fn test_eval_nil () {
            let mut state =  State {
                stack: Stack::empty(),
                env: Stack::empty(),
                control: list!(InstCell(NIL),AtomCell(SInt(1))),
                dump: Stack::empty()
            };
            assert_eq!(state.stack.peek(), None);
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&ListCell(box Nil)));
        }

        #[test]
        fn test_eval_ldc () {
            let mut state = State::new();
            assert_eq!(state.stack.peek(), None);
            state = State {
                stack: state.stack,
                env: state.env,
                control: list!(InstCell(LDC),AtomCell(SInt(1))),
                dump: state.dump
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(SInt(1))));

            state = State {
                stack: state.stack,
                env: state.env,
                control: list!(InstCell(LDC),AtomCell(Char('a'))),
                dump: state.dump
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Char('a'))));

            state = State {
                stack: state.stack,
                env: state.env,
                control: list!(InstCell(LDC),AtomCell(Float(1.0f64))),
                dump: state.dump
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Float(1.0f64))));
        }

        #[test]
        fn test_eval_ld () {
            let mut state = State {
                stack: Stack::empty(),
                env: list!(ListCell(box list!(AtomCell(SInt(155)),AtomCell(UInt(388))))),
                control: list!(InstCell(LD),ListCell(box list!(AtomCell(SInt(1)),AtomCell(SInt(2))))),
                dump: Stack::empty()
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(SInt(155))));
        }

        #[test]
        fn test_eval_ldf () {
                let mut state = State {
                stack: Stack::empty(),
                env: list!(
                    ListCell(
                        box list!(
                            AtomCell(SInt(155)),
                            AtomCell(UInt(388))
                        )
                    ),
                    ListCell(
                        box list!(
                            AtomCell(Float(6.66)),
                            AtomCell(SInt(666))
                            )
                        )
                    ),
                control: list!(InstCell(LDF), ListCell(box list!(AtomCell(SInt(133))))),
                dump: Stack::empty()
            };
            state = state.eval();
            assert_eq!(
                state.stack.peek(),
                Some(&ListCell(
                    box list!(
                        ListCell(
                            box list!(
                                AtomCell(SInt(133))
                            )),
                        ListCell(
                            box list!(
                                AtomCell(SInt(155)),
                                AtomCell(UInt(388))
                            ),
                        )
                    )
                )
            )
        );
        }

        #[test]
        fn test_eval_join() {
            let mut state = State {
                stack: Stack::empty(),
                env: Stack::empty(),
                control: list!(InstCell(JOIN)),
                dump: list!(ListCell(box list!(
                        AtomCell(SInt(1)),
                        AtomCell(SInt(2))
                    )))
            };
            state = state.eval();
            assert_eq!(state.dump.peek(), None);
            assert_eq!(state.control.peek(), Some(&ListCell(box list!(
                        AtomCell(SInt(1)),
                        AtomCell(SInt(2))
                    ))))
        }

        #[test]
        fn test_eval_add () {
            // ---- Unsigned int addition ----
            let mut state = State {
                stack: list!(AtomCell(UInt(1)), AtomCell(UInt(1))),
                env: Stack::empty(),
                control: list!(InstCell(ADD)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(UInt(2))));

            // ---- Signed int addition ----
            state = State {
                stack: list!(AtomCell(SInt(-1)), AtomCell(SInt(-1))),
                env: Stack::empty(),
                control: list!(InstCell(ADD)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(SInt(-2))));

            // ---- Float-float addition ----
            state = State {
                stack: list!(AtomCell(Float(1.5)), AtomCell(Float(1.5))),
                env: Stack::empty(),
                control: list!(InstCell(ADD)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Float(3.0))));

            // ---- Float-int type lifting addition ----
            state = State {
                stack: list!(AtomCell(Float(1.5)), AtomCell(SInt(1))),
                env: Stack::empty(),
                control: list!(InstCell(ADD)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Float(2.5))));
            state = State {
                stack: list!(AtomCell(Float(3.5)), AtomCell(UInt(1))),
                env: Stack::empty(),
                control: list!(InstCell(ADD)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Float(4.5))));
        }

        #[test]
        fn test_eval_sub () {
            // ---- Unsigned int subtraction ----
            let mut state = State {
                stack: list!(AtomCell(UInt(3)), AtomCell(UInt(3))),
                env: Stack::empty(),
                control: list!(InstCell(SUB)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(UInt(0))));

            // ---- Signed int subtraction----
            state = State {
                stack: list!(AtomCell(SInt(-3)), AtomCell(SInt(3))),
                env: Stack::empty(),
                control: list!(InstCell(SUB)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(SInt(-6))));

            // ---- Float-float subtraction ----
            state = State {
                stack: list!(AtomCell(Float(1.5)), AtomCell(Float(2.0))),
                env: Stack::empty(),
                control: list!(InstCell(SUB)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Float(-0.5))));

            // ---- Float-int type lifting subtraction ----
            state = State {
                stack: list!(AtomCell(Float(2.5)), AtomCell(SInt(-2))),
                env: Stack::empty(),
                control: list!(InstCell(SUB)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Float(4.5))));

            state = State {
                stack: list!(AtomCell(Float(3.5)), AtomCell(UInt(2))),
                env: Stack::empty(),
                control: list!(InstCell(SUB)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Float(1.5))));
        }

        #[test]
        fn test_eval_mul () {
            // ---- Unsigned int multiplication ----
            let mut state = State {
                stack: list!(AtomCell(UInt(2)), AtomCell(UInt(3))),
                env: Stack::empty(),
                control: list!(InstCell(MUL)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(UInt(6))));

            // ---- Signed int multiplication----
            state = State {
                stack: list!(AtomCell(SInt(-2)), AtomCell(SInt(-3))),
                env: Stack::empty(),
                control: list!(InstCell(MUL)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(SInt(6))));

            // ---- Float-float multiplication ----
            state = State {
                stack: list!(AtomCell(Float(1.5)), AtomCell(Float(2.0))),
                env: Stack::empty(),
                control: list!(InstCell(MUL)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Float(3.0))));

            // ---- Float-int type lifting multiplication ----
            state = State {
                stack: list!(AtomCell(Float(1.5)), AtomCell(SInt(2))),
                env: Stack::empty(),
                control: list!(InstCell(MUL)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Float(3.0))));

            state = State {
                stack: list!(AtomCell(Float(3.5)), AtomCell(UInt(2))),
                env: Stack::empty(),
                control: list!(InstCell(MUL)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Float(7.0))));
        }

        #[test]
        fn test_eval_div () {
            // ---- Unsigned int divison ----
            let mut state = State {
                stack: list!(AtomCell(UInt(6)), AtomCell(UInt(2))),
                env: Stack::empty(),
                control: list!(InstCell(DIV)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(UInt(3))));

            // ---- Signed int divison ----
            state = State {
                stack: list!(AtomCell(SInt(-6)), AtomCell(SInt(2))),
                env: Stack::empty(),
                control: list!(InstCell(DIV)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(SInt(-3))));

            // ---- Float-float divison ----
            state = State {
                stack: list!(AtomCell(Float(3.0)), AtomCell(Float(2.0))),
                env: Stack::empty(),
                control: list!(InstCell(DIV)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Float(1.5))));

            // ---- Float-int type lifting divison ----
            state = State {
                stack: list!(AtomCell(Float(3.0)), AtomCell(SInt(2))),
                env: Stack::empty(),
                control: list!(InstCell(DIV)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Float(1.5))));

            state = State {
                stack: list!(AtomCell(Float(3.0)), AtomCell(UInt(2))),
                env: Stack::empty(),
                control: list!(InstCell(DIV)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Float(1.5))));
        }

        #[test]
        fn test_eval_fdiv () {
            // ---- Unsigned int divison ----
            let mut state = State {
                stack: list!(AtomCell(UInt(3)), AtomCell(UInt(2))),
                env: Stack::empty(),
                control: list!(InstCell(FDIV)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Float(1.5))));

            // ---- Signed int divison ----
            state = State {
                stack: list!(AtomCell(SInt(-3)), AtomCell(SInt(2))),
                env: Stack::empty(),
                control: list!(InstCell(FDIV)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Float(-1.5))));

            // ---- Float-float divison ---
            state = State {
                stack: list!(AtomCell(Float(3.0)), AtomCell(Float(2.0))),
                env: Stack::empty(),
                control: list!(InstCell(FDIV)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Float(1.5))));
        }

        #[test]
        fn test_eval_mod () {
            // ---- Unsigned int modulus ----
            let mut state = State {
                stack: list!(AtomCell(UInt(3)), AtomCell(UInt(2))),
                env: Stack::empty(),
                control: list!(InstCell(MOD)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(UInt(3%2))));

            // ---- Signed int modulus ----
            state = State {
                stack: list!(AtomCell(SInt(-3)), AtomCell(SInt(2))),
                env: Stack::empty(),
                control: list!(InstCell(MOD)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(SInt(-3%2))));

            // ---- Float-float modulus---
            state = State {
                stack: list!(AtomCell(Float(3.0)), AtomCell(Float(2.0))),
                env: Stack::empty(),
                control: list!(InstCell(MOD)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Float(3.0%2.0))));

            // ---- Float-int type lifting modulus----
            state = State {
                stack: list!(AtomCell(Float(3.0)), AtomCell(SInt(2))),
                env: Stack::empty(),
                control: list!(InstCell(MOD)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Float(3.0%2.0))));

            state = State {
                stack: list!(AtomCell(Float(3.0)), AtomCell(UInt(2))),
                env: Stack::empty(),
                control: list!(InstCell(MOD)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Float(3.0%2.0))));
        }

        #[test]
        fn test_eval_eq () {
            // ---- Unsigned int equality ----
            let mut state = State {
                stack: list!(AtomCell(UInt(3)), AtomCell(UInt(3))),
                env: Stack::empty(),
                control: list!(InstCell(EQ)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(true))));

            state = State {
                stack: list!(AtomCell(UInt(1)), AtomCell(UInt(2))),
                env: Stack::empty(),
                control: list!(InstCell(EQ)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(false))));

            // ---- Signed int equality ----
            state = State {
                stack: list!(AtomCell(SInt(3)), AtomCell(SInt(3))),
                env: Stack::empty(),
                control: list!(InstCell(EQ)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(true))));

            state = State {
                stack: list!(AtomCell(SInt(-2)), AtomCell(SInt(2))),
                env: Stack::empty(),
                control: list!(InstCell(EQ)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(false))));


            // ---- Float equality ----
            state = State {
                stack: list!(AtomCell(Float(3.0)), AtomCell(Float(3.0))),
                env: Stack::empty(),
                control: list!(InstCell(EQ)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(true))));

            state = State {
                stack: list!(AtomCell(Float(-2.0)), AtomCell(Float(2.0))),
                env: Stack::empty(),
                control: list!(InstCell(EQ)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(false))));

            state = State {
                stack: list!(AtomCell(Float(2.11)), AtomCell(Float(2.1))),
                env: Stack::empty(),
                control: list!(InstCell(EQ)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(false))));

        }

        #[test]
        fn test_eval_gt () {
            // ---- Unsigned int greater-than ----
            let mut state = State {
                stack: list!(AtomCell(UInt(3)), AtomCell(UInt(2))),
                env: Stack::empty(),
                control: list!(InstCell(GT)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(true))));

            state = State {
                stack: list!(AtomCell(UInt(1)), AtomCell(UInt(2))),
                env: Stack::empty(),
                control: list!(InstCell(GT)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(false))));

            // ---- Signed int greater-than ----
            state = State {
                stack: list!(AtomCell(SInt(3)), AtomCell(SInt(2))),
                env: Stack::empty(),
                control: list!(InstCell(GT)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(true))));

            state = State {
                stack: list!(AtomCell(SInt(-2)), AtomCell(SInt(2))),
                env: Stack::empty(),
                control: list!(InstCell(GT)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(false))));


            // ---- Float greater-than----
            state = State {
                stack: list!(AtomCell(Float(3.0)), AtomCell(Float(1.0))),
                env: Stack::empty(),
                control: list!(InstCell(GT)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(true))));

            state = State {
                stack: list!(AtomCell(Float(-2.0)), AtomCell(Float(2.0))),
                env: Stack::empty(),
                control: list!(InstCell(GT)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(false))));

            state = State {
                stack: list!(AtomCell(Float(2.11)), AtomCell(Float(2.1))),
                env: Stack::empty(),
                control: list!(InstCell(GT)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(true))));

            // ---- Mixed type greater-than ---
            state = State {
                stack: list!(AtomCell(Float(3.0)), AtomCell(SInt(2))),
                env: Stack::empty(),
                control: list!(InstCell(GT)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(true))));

            state = State {
                stack: list!(AtomCell(Float(1.0)), AtomCell(SInt(1))),
                env: Stack::empty(),
                control: list!(InstCell(GT)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(true))));

            state = State {
                stack: list!(AtomCell(UInt(1)), AtomCell(Float(2.0))),
                env: Stack::empty(),
                control: list!(InstCell(GT)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(false))));

        }

        #[test]
        fn test_eval_gte () {
            // ---- Unsigned int greater-than ----
            let mut state = State {
                stack: list!(AtomCell(UInt(3)), AtomCell(UInt(2))),
                env: Stack::empty(),
                control: list!(InstCell(GTE)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(true))));

            state = State {
                stack: list!(AtomCell(UInt(1)), AtomCell(UInt(1))),
                env: Stack::empty(),
                control: list!(InstCell(GTE)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(true))));

            state = State {
                stack: list!(AtomCell(UInt(1)), AtomCell(UInt(2))),
                env: Stack::empty(),
                control: list!(InstCell(GTE)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(false))));


            // ---- Signed int greater-than ----
            state = State {
                stack: list!(AtomCell(SInt(3)), AtomCell(SInt(2))),
                env: Stack::empty(),
                control: list!(InstCell(GTE)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(true))));

            state = State {
                stack: list!(AtomCell(SInt(1)), AtomCell(SInt(1))),
                env: Stack::empty(),
                control: list!(InstCell(GTE)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(true))));

            state = State {
                stack: list!(AtomCell(SInt(1)), AtomCell(SInt(2))),
                env: Stack::empty(),
                control: list!(InstCell(GTE)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(false))));


            // ---- Float greater-than----
            state = State {
                stack: list!(AtomCell(Float(3.0)), AtomCell(Float(2.0))),
                env: Stack::empty(),
                control: list!(InstCell(GTE)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(true))));

            state = State {
                stack: list!(AtomCell(Float(1.0)), AtomCell(Float(1.0))),
                env: Stack::empty(),
                control: list!(InstCell(GTE)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(true))));

            state = State {
                stack: list!(AtomCell(Float(1.0)), AtomCell(Float(2.0))),
                env: Stack::empty(),
                control: list!(InstCell(GTE)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(false))));

            // ---- Mixed type greater-than-equal ---
            state = State {
                stack: list!(AtomCell(Float(3.0)), AtomCell(SInt(2))),
                env: Stack::empty(),
                control: list!(InstCell(GTE)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(true))));

            state = State {
                stack: list!(AtomCell(Float(1.0)), AtomCell(SInt(1))),
                env: Stack::empty(),
                control: list!(InstCell(GTE)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(true))));

            state = State {
                stack: list!(AtomCell(UInt(1)), AtomCell(Float(2.0))),
                env: Stack::empty(),
                control: list!(InstCell(GTE)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(false))));
        }

        #[test]
        fn test_eval_lt () {
            // ---- Unsigned int greater-than ----
            let mut state = State {
                stack: list!(AtomCell(UInt(3)), AtomCell(UInt(2))),
                env: Stack::empty(),
                control: list!(InstCell(LT)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(false))));

            state = State {
                stack: list!(AtomCell(UInt(1)), AtomCell(UInt(2))),
                env: Stack::empty(),
                control: list!(InstCell(LT)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(true))));

            state = State {
                stack: list!(AtomCell(UInt(1)), AtomCell(UInt(1))),
                env: Stack::empty(),
                control: list!(InstCell(LT)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(false))));


            // ---- Signed int greater-than ----
            state = State {
                stack: list!(AtomCell(SInt(3)), AtomCell(SInt(2))),
                env: Stack::empty(),
                control: list!(InstCell(LT)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(false))));

            state = State {
                stack: list!(AtomCell(SInt(-2)), AtomCell(SInt(2))),
                env: Stack::empty(),
                control: list!(InstCell(LT)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(true))));

            state = State {
                stack: list!(AtomCell(SInt(2)), AtomCell(SInt(2))),
                env: Stack::empty(),
                control: list!(InstCell(LT)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(false))));


            // ---- Float greater-than----
            state = State {
                stack: list!(AtomCell(Float(3.0)), AtomCell(Float(1.0))),
                env: Stack::empty(),
                control: list!(InstCell(LT)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(false))));

            state = State {
                stack: list!(AtomCell(Float(-2.0)), AtomCell(Float(2.0))),
                env: Stack::empty(),
                control: list!(InstCell(LT)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(true))));

            state = State {
                stack: list!(AtomCell(Float(2.11)), AtomCell(Float(2.1))),
                env: Stack::empty(),
                control: list!(InstCell(LT)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(false))));

               state = State {
                stack: list!(AtomCell(Float(2.0)), AtomCell(Float(2.0))),
                env: Stack::empty(),
                control: list!(InstCell(LT)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(false))));

            // ---- Mixed type greater-than ---
            state = State {
                stack: list!(AtomCell(Float(3.0)), AtomCell(SInt(2))),
                env: Stack::empty(),
                control: list!(InstCell(LT)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(false))));

            state = State {
                stack: list!(AtomCell(Float(1.0)), AtomCell(SInt(1))),
                env: Stack::empty(),
                control: list!(InstCell(LT)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(false))));

            state = State {
                stack: list!(AtomCell(UInt(1)), AtomCell(Float(2.0))),
                env: Stack::empty(),
                control: list!(InstCell(LT)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(true))));

        }

        #[test]
        fn test_eval_lte () {
            // ---- Unsigned int greater-than ----
            let mut state = State {
                stack: list!(AtomCell(UInt(3)), AtomCell(UInt(2))),
                env: Stack::empty(),
                control: list!(InstCell(LTE)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(false))));

            state = State {
                stack: list!(AtomCell(UInt(1)), AtomCell(UInt(1))),
                env: Stack::empty(),
                control: list!(InstCell(LTE)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(true))));

            state = State {
                stack: list!(AtomCell(UInt(1)), AtomCell(UInt(2))),
                env: Stack::empty(),
                control: list!(InstCell(LTE)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(true))));


            // ---- Signed int greater-than ----
            state = State {
                stack: list!(AtomCell(SInt(3)), AtomCell(SInt(2))),
                env: Stack::empty(),
                control: list!(InstCell(LTE)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(false))));

            state = State {
                stack: list!(AtomCell(SInt(1)), AtomCell(SInt(1))),
                env: Stack::empty(),
                control: list!(InstCell(LTE)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(true))));

            state = State {
                stack: list!(AtomCell(SInt(1)), AtomCell(SInt(2))),
                env: Stack::empty(),
                control: list!(InstCell(LTE)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(true))));


            // ---- Float greater-than----
            state = State {
                stack: list!(AtomCell(Float(3.0)), AtomCell(Float(2.0))),
                env: Stack::empty(),
                control: list!(InstCell(LTE)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(false))));

            state = State {
                stack: list!(AtomCell(Float(1.0)), AtomCell(Float(1.0))),
                env: Stack::empty(),
                control: list!(InstCell(LTE)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(true))));

            state = State {
                stack: list!(AtomCell(Float(1.0)), AtomCell(Float(2.0))),
                env: Stack::empty(),
                control: list!(InstCell(LTE)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(true))));

            // ---- Mixed type greater-than-equal ---
            state = State {
                stack: list!(AtomCell(Float(3.0)), AtomCell(SInt(2))),
                env: Stack::empty(),
                control: list!(InstCell(LTE)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(false))));

            state = State {
                stack: list!(AtomCell(Float(1.0)), AtomCell(SInt(1))),
                env: Stack::empty(),
                control: list!(InstCell(LTE)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(true))));

            state = State {
                stack: list!(AtomCell(UInt(1)), AtomCell(Float(2.0))),
                env: Stack::empty(),
                control: list!(InstCell(LTE)),
                dump: Stack::empty(),
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(true))));
        }

    }

}
