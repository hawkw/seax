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
    //use std::iter::IteratorExt;
    use std::fmt;
    use std::ops;
    use svm::Inst::*;
    use svm::SVMCell::*;
    use svm::Atom::*;

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
    #[derive(PartialEq,Clone,Debug)]
    pub enum SVMCell {
        AtomCell(Atom),
        ListCell(Box<List<SVMCell>>),
        InstCell(Inst)
    }

    impl fmt::Display for SVMCell {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self)
        }
    }

    /// SVM atom types.
    ///
    /// A VM atom can be either an unsigned int, signed int, float,
    /// char, or bool.
    #[derive(PartialEq,PartialOrd,Copy,Clone,Debug)]
    pub enum Atom {
        /// Unsigned integer atom (machine size)
        UInt(usize),
        /// Signed integer atom (machine size)
        SInt(isize),
        /// Floating point number atom (64-bits)
        Float(f64),
        /// UTF-8 character atom
        Char(char),
        /// Boolean atom
        ///
        /// The original SECD machine used 0 as false and 1 as true.
        /// This is just to make my life slightly easier.
        Bool(bool)
    }

    impl fmt::Display for Atom {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                &Atom::UInt(value) => write!(f, "{}us", value),
                &Atom::SInt(value) => write!(f, "{}is", value),
                &Atom::Float(value) => write!(f, "{}f64", value),
                &Atom::Char(value) => write!(f, "'{}'", value),
                &Atom::Bool(value) => write!(f, "{}", value)
            }
        }
    }

    impl ops::Add for Atom {
        type Output = Atom;

        fn add(self, other: Atom) -> Atom {
            match (self, other) {
                // same type:  no coercion
                (SInt(a), SInt(b))      => SInt(a + b),
                (UInt(a), UInt(b))      => UInt(a + b),
                (Float(a), Float(b))    => Float(a + b),
                (Char(a), Char(b))      => Char((a as u8 + b as u8) as char),
                // float + int: coerce to float
                (Float(a), SInt(b))     => Float(a + b as f64),
                (Float(a), UInt(b))     => Float(a + b as f64),
                (SInt(a), Float(b))     => Float(a as f64 + b),
                (UInt(a), Float(b))     => Float(a as f64 + b),
                // uint + sint: coerce to sint
                (UInt(a), SInt(b))      => SInt(a as isize + b),
                (SInt(a), UInt(b))      => SInt(a + b as isize),
                // char + any: coerce to char
                // because of the supported operations on Rust chars,
                // everything has to be cast to u8 (byte) to allow
                // arithmetic ops and then cast back to char.
                (Char(a), UInt(b))      => Char((a as u8 + b as u8) as char),
                (Char(a), SInt(b))      => Char((a as u8 + b as u8) as char),
                (Char(a), Float(b))     => Char((a as u8 + b as u8) as char),
                (UInt(a), Char(b))      => Char((a as u8 + b as u8) as char),
                (SInt(a), Char(b))      => Char((a as u8 + b as u8) as char),
                (Float(a), Char(b))     => Char((a as u8 + b as u8) as char),
                (_, _)                  => panic!("TypeError: Unsupported operands {:?} + {:?}", self,other)
            }
        }

    }

    impl ops::Sub for Atom {
        type Output = Atom;

        fn sub(self, other: Atom) -> Atom {
            match (self, other) {
                // same type:  no coercion
                (SInt(a), SInt(b))      => SInt(a - b),
                (UInt(a), UInt(b))      => UInt(a - b),
                (Float(a), Float(b))    => Float(a - b),
                (Char(a), Char(b))      => Char((a as u8 - b as u8) as char),
                // float + int: coerce to float
                (Float(a), SInt(b))     => Float(a - b as f64),
                (Float(a), UInt(b))     => Float(a - b as f64),
                (SInt(a), Float(b))     => Float(a as f64 - b),
                (UInt(a), Float(b))     => Float(a as f64 - b),
                // uint + sint: coerce to sint
                (UInt(a), SInt(b))      => SInt(a as isize - b),
                (SInt(a), UInt(b))      => SInt(a - b as isize),
                // char + any: coerce to char
                (Char(a), UInt(b))      => Char((a as u8 - b as u8) as char),
                (Char(a), SInt(b))      => Char((a as u8 - b as u8) as char),
                (Char(a), Float(b))     => Char((a as u8 - b as u8) as char),
                (UInt(a), Char(b))      => Char((a as u8 - b as u8) as char),
                (SInt(a), Char(b))      => Char((a as u8 - b as u8) as char),
                (Float(a), Char(b))     => Char((a as u8 - b as u8) as char),
                (_, _)                  => panic!("TypeError: Unsupported operands {:?} - {:?}", self,other)
            }
        }

    }

    /// SVM instruction types
    #[derive(Debug,Copy,Clone,PartialEq)]
    pub enum Inst {
        /// `nil`
        ///
        /// Pushes an empty list (nil) onto the stack
        NIL,
        /// `ldc`: `L`oa`d` `C`onstant. Loads a constant (atom)
        LDC,
        /// `ld`: `L`oa`d`. Pushes a variable onto the stack.
        ///
        /// The variable is indicated by the argument, a pair.
        /// The pair's `car` specifies the level, the `cdr` the position.
        /// So `(1 . 3)` gives the current function's (level 1) third
        /// parameter.
        LD,
        /// `ldf`: `L`oa`d` `F`unction.
        ///
        ///  Takes one list argument representing a function and constructs
        ///  a closure (a pair containing the function and the current
        ///  environment) and pushes that onto the stack.
        LDF,
        /// `join`
        ///
        /// Pops a list reference from the dump and makes this the new value
        /// of `C`. This instruction occurs at the end of both alternatives of
        ///  a `sel`.
        JOIN,
        /// `ap`: `Ap`ply.
        ///
        /// Pops a closure and a list of parameter values from the stack.
        /// The closure is applied to the parameters by installing its
        /// environment as the current one, pushing the parameter list
        /// in front of that, clearing the stack, and setting `C` to the
        /// closure's function pointer. The previous values of `S`, `E`,
        ///  and the next value of `C` are saved on the dump.
        AP,
        /// `ret`: `Ret`urn.
        ///
        /// Pops one return value from the stack, restores
        /// `S`, `E`, and `C` from the dump, and pushes
        /// the return value onto the now-current stack.
        RET,
        /// `dum`: `Dum`my.
        ///
        /// Pops a dummy environment (an empty list) onto the `E` stack.
        DUM,
        /// `rap`: `R`ecursive `Ap`ply.
        /// Works like `ap`, only that it replaces an occurrence of a
        /// dummy environment with the current one, thus making recursive
        ///  functions possible.
        RAP,
        /// `sel`: `Sel`ect branch
        ///
        /// Expects two list arguments on the control stack, and pops a value
        /// from the stack. The first list is executed if the popped value
        /// was non-nil, the second list otherwise. Before one of these list
        /// pointers is made the new `C`, a pointer to the instruction
        /// following `sel` is saved on the dump.
        SEL,
        /// `add`
        ///
        /// Pops two numbers off of the stack and adds them, pushing the
        /// result onto the stack. This will up-convert integers to floating
        /// point if necessary.
        ///
        /// TODO: figure out what happens when you try to add things that aren't
        /// numbers (maybe the compiler won't let this happen?).
        ADD,
        /// `sub`: `Sub`tract
        ///
        /// Pops two numbers off of the stack and subtracts the first from the
        /// second, pushing the result onto the stack. This will up-convert
        /// integers to floating point if necessary.
        ///
        /// TODO: figure out what happens when you try to subtract things that
        /// aren't numbers (maybe the compiler won't let this happen?).
        SUB,
        /// `mul`: `Mul`tiply
        ///
        /// Pops two numbers off of the stack and multiplies them, pushing the
        /// result onto the stack. This will up-convert integers to floating
        /// point if necessary.
        ///
        /// TODO: figure out what happens when you try to multiply things that
        /// aren't numbers (maybe the compiler won't let this happen?).
        MUL,
        /// `div`: `Div`ide
        ///
        /// Pops two numbers off of the stack and divides the first by the second,
        /// pushing the result onto the stack. This performs integer division.
        ///
        /// TODO: figure out what happens when you try to divide things that
        /// aren't numbers (maybe the compiler won't let this happen?).
        DIV,
        /// `fdiv`: `F`loating-point `div`ide
        ///
        /// Pops two numbers off of the stack and divides the first by the second,
        /// pushing the result onto the stack. This performs float division.
        ///
        /// TODO: figure out what happens when you try to divide things that
        /// aren't numbers (maybe the compiler won't let this happen?).
        ///
        /// TODO: Not sure if there should be separate float and int divide words
        /// I guess the compiler can figure this out
        FDIV,
        /// `mod`: `Mod`ulo
        ///
        /// Pops two numbers off of the stack and divides the first by the second,
        /// pushing the remainder onto the stack.
        ///
        /// TODO: figure out what happens when you try to modulo things that
        /// aren't numbers (maybe the compiler won't let this happen?).
        MOD,
        /// `eq`: `Eq`uality of atoms
        EQ,
        /// `gt`: `G`reater `t`han
        ///
        /// Pops two numbers on the stack and puts a 'true' on the stack
        /// if the first atom is greater than the other atom, false otherwise.
        GT,
        /// `gte`: `G`reater `t`han or `e`qual
        GTE,
        /// `lt`: `L`ess `t`han
        LT,
        /// `lte`: `L`ess `t`han or `e`qual
        LTE,
        /// `atom`: test if `atom`
        ///
        /// Pops an item from the stack and returns true if it's an atom, false
        /// otherwise
        ATOM,
        /// `car`: `C`ontents of `A`ddress `R`egister
        ///
        /// Pops a list from the stack and returns the list's `car` (head)
        CAR,
        /// `cdr`: `C`ontents of `D`ecrement `R`egister
        ///
        /// Pops a list from the stack and returns the list's `cdr` (tail)
        CDR,
        /// `cons`: `Cons`truct
        ///
        /// Pops an item and a list from the stack and returns the list, with
        /// the item prepended.
        CONS,
        // TODO: add some hardcoded I/O instructions here so that you can
        //  do I/O without farming everything out to `stdio`
    }

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
                    // TODO: currently floats are special cased, this should be
                    // fixed with a custom implementation of `PartialOrd` for
                    // Atom.
                    let (op1, new_stack) = self.stack.pop().unwrap();
                    let (op2, newer_stack) = new_stack.pop().unwrap();
                    match (op1,op2){
                        (AtomCell(Float(a)), AtomCell(SInt(b))) => State {
                            stack: newer_stack.push(AtomCell(Bool(a > b as f64))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(Float(a)), AtomCell(UInt(b))) => State {
                            stack: newer_stack.push(AtomCell(Bool(a > b as f64))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(SInt(a)), AtomCell(Float(b))) => State {
                            stack: newer_stack.push(AtomCell(Bool(a as f64 > b ))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(UInt(a)), AtomCell(Float(b))) => State {
                            stack: newer_stack.push(AtomCell(Bool(a as f64 > b ))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
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
                    // TODO: currently floats are special cased, this should be
                    // fixed with a custom implementation of `PartialOrd` for
                    // Atom.
                    let (op1, new_stack) = self.stack.pop().unwrap();
                    let (op2, newer_stack) = new_stack.pop().unwrap();
                    match (op1,op2) {
                        (AtomCell(Float(a)), AtomCell(SInt(b))) => State {
                            stack: newer_stack.push(AtomCell(Bool(a >= b as f64))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(Float(a)), AtomCell(UInt(b))) => State {
                            stack: newer_stack.push(AtomCell(Bool(a >= b as f64))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(SInt(a)), AtomCell(Float(b))) => State {
                            stack: newer_stack.push(AtomCell(Bool(a as f64 >= b ))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(UInt(a)), AtomCell(Float(b))) => State {
                            stack: newer_stack.push(AtomCell(Bool(a as f64 >= b ))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
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
        use super::{State, Atom};
        use super::Inst::*;
        use super::SVMCell::*;
        use super::Atom::*;

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
            assert_eq!(state.stack.peek(), Some(&AtomCell(Bool(false))));

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
        fn test_atom_show () {
            let mut a: Atom;

            a = Char('a');
            assert_eq!(format!("{}", a), "'a'");

            a = UInt(1us);
            assert_eq!(format!("{}", a), "1us");

            a = SInt(42is);
            assert_eq!(format!("{}", a), "42is");

            a = Float(5.55f64);
            assert_eq!(format!("{}", a), "5.55f64");
        }
    }

}
