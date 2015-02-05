#![crate_name = "seax_svm"]
#![crate_type = "lib"]
#![feature(box_syntax)]

/// Contains the Seax Virtual Machine (SVM) and miscellaneous
/// support code.
pub mod svm {
    use svm::slist::List;
    use svm::slist::List::{Cons,Nil};
    use svm::slist::Stack;
    use std::iter::IteratorExt;
    use std::fmt;


    /// Singly-linked list and stack implementations.
    ///
    /// `List<T>` is a singly-linked cons list with boxed items. `Stack<T>` is
    ///  defined as a trait providing stack operations(`push()`, `pop()`, and
    ///  `peek()`), and an implementation for `List`.
    #[macro_use]
    pub mod slist {

        use svm::slist::List::{Cons,Nil};
        use std::fmt;
        use std::ops::Index;

        /// Common functions for an immutable Stack abstract data type.
        pub trait Stack<T> {

            /// Push an item to the top of the stack, returning a new stack
            fn push(self, item : T) -> Self;

            /// Pop the top element of the stack. Returns an Option on a T and
            /// a new Stack<T> to replace this.
            fn pop(self)            -> Option<(T, Self)>;

            /// Peek at the top item of the stack.
            ///
            /// Returns Some<T> if there is an item on top of the stack,
            /// and None if the stack is empty.
            fn peek(&self)          -> Option<&T>;

            /// Returns an empty stack.
            fn empty()              -> Self;
        }

        /// Stack implementation using a cons list
        impl<T> Stack<T> for List<T> {

            /// Push an item to the top of the stack, returning a new stack.
            ///
            /// # Examples:
            /// ```
            /// use seax_svm::svm::slist::{List,Stack};
            ///
            /// let mut s: List<isize> = Stack::empty();
            /// assert_eq!(s.peek(), None);
            /// s = s.push(1);
            /// assert_eq!(s.peek(), Some(&1));
            /// s = s.push(6);
            /// assert_eq!(s.peek(), Some(&6));
            /// ```
            fn push(self, item: T) -> List<T> {
                Cons(item, box self)
            }

            /// Pop the top element of the stack.
            ///
            /// Pop the top element of the stack. Returns an
            /// `Option<(T,List<T>)>` containing the top element and a new
            /// `List<T>` with that item removed, or `None` if the stack is
            /// empty.
            ///
            /// # Examples:
            /// ```
            /// # use seax_svm::svm::slist::{List,Stack};
            ///
            /// let mut s: List<isize> = Stack::empty();
            /// s = s.push(2);
            /// s = s.push(1);
            /// let pop_result = s.pop().unwrap();
            /// s = pop_result.1;
            /// assert_eq!(s.peek(), Some(&2));
            /// assert_eq!(pop_result.0, 1);
            /// ```
            fn pop(self) -> Option<(T,List<T>)> {
                match self {
                    Cons(item, new_self)    => Some((item, *new_self)),
                    Nil                     => None
                }
            }

            fn empty() -> List<T> {
                Nil
            }


            /// Peek at the top element of the stack.
            ///
            /// Peek at the top element of the stack. Returns an `Option<&T>`
            /// with a borrowed pointer to the top element, or `None` if the
            /// stack is empty.
            ///
            /// # Examples:
            /// ```
            /// # use seax_svm::svm::slist::{List,Stack};
            ///
            /// let mut s: List<isize> = Stack::empty();
            /// s = s.push(2);
            /// s = s.push(1);
            /// let pop_result = s.pop().unwrap();
            /// s = pop_result.1;
            /// assert_eq!(s.peek(), Some(&2));
            /// assert_eq!(pop_result.0, 1);
            /// ```
            fn peek(&self) -> Option<&T> {
                match self {
                    &Nil => None,
                    &Cons(ref it,_) => Some(it)
                }
            }

        }

        /// Singly-linked cons list.
        ///
        /// This is used internally to represent list primitives in the
        /// machine.
        #[derive(PartialEq,Clone,Debug)]
        pub enum List<T> {
            /// Cons cell containing a `T` and a link to the tail
            Cons(T, Box<List<T>>),
            /// The empty list.
            Nil,
        }

        /// Public implementation for List.
        impl<T> List<T> {


            /// Creates a new empty list
            pub fn new() -> List<T> {
                Nil
            }

            /// Prepends the given item to the list.
            ///
            /// Returns the list containing the new  head item.
            /// This is an O(1) operation.
            pub fn prepend(self, it: T) -> List<T> {
                Cons(it, box self)
            }

            /// Appends an item to the end of the list.
            ///
            /// This is an O(n) operation.
            pub fn append(self, it: T) {
                unimplemented!()
            }

            /// Returns the length of the list.
            pub fn length (&self) -> usize {
                match *self {
                    Cons(_, ref tail) => 1 + tail.length(),
                    Nil => 0
                }
            }

            /// Provide a forward iterator
            #[inline]
            pub fn iter<'a>(&'a self) -> ListIterator<'a, T> {
                ListIterator{current: self}
            }
        }

        impl<'a, T> fmt::Display for List<T> where T: fmt::Display{

            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                // TODO: replace toString with this
                match *self {
                    Cons(ref head, ref tail) => write!(f, "({}, {})", head, tail),
                    Nil => write!(f,"nil")
                }
            }
        }

        /// Wraps a List<T> to allow it to be used as an Iterator<T>
        pub struct ListIterator<'a, T:'a> {
            current: &'a List<T>
        }

        /// Implementation of Iterator for List. This allows iteration by
        /// link hopping.
        impl<'a, T> Iterator for ListIterator<'a, T> {
            type Item = &'a T;

            /// Get the next element from the list. Returns a Some<T>, or Nil
            /// if at the end of the list.
            fn next(&mut self) -> Option<&'a T> {
                match self.current {
                    &Cons(ref head, box ref tail) => { self.current = tail; Some(head) },
                    &Nil => None
                }
            }
        }

        impl<'a, T> ExactSizeIterator for ListIterator<'a, T> {
            fn len(&self) -> usize {
                self.current.length()
            }
        }

        impl<T> Index<usize> for List<T> {
            type Output = T;

            fn index<'a>(&'a self, _index: &usize) -> &'a T {
                let mut it = self.iter();
                for _ in 0..*_index-1 {
                    it.next();
                }
                it.next().unwrap()
            }
        }
        impl<T> Index<isize> for List<T> {
            type Output = T;

            fn index<'a>(&'a self, _index: &isize) -> &'a T {
                let mut it = self.iter();
                for _ in 0..*_index-1 {
                    it.next();
                }
                it.next().unwrap()
            }
        }


        /// Convenience macro for making lists.
        ///
        /// # Example:
        ///
        /// ```
        /// # #[macro_use] extern crate seax_svm;
        /// # use seax_svm::svm::slist;
        /// # use seax_svm::svm::slist::List::{Cons, Nil};
        /// # fn main () {
        /// assert_eq!(
        ///     list!(1i32, 2i32, 3i32),
        ///     Cons(1i32, Box::new(Cons(2i32, Box::new(Cons(3i32, Box::new(Nil))))))
        ///     );
        /// # }
        /// ```
        #[macro_export]
        macro_rules! list(
            ( $e:expr, $($rest:expr),+ ) => ( Cons($e, Box::new(list!( $( $rest ),+ )) ));
            ( $e:expr ) => ( Cons($e, Box::new(Nil)) );
            () => ( @Empty )
        );

        #[cfg(test)]
        mod tests {
            use super::{List, Stack};
            use super::List::{Cons,Nil};

            #[test]
            fn test_list_length() {
                let full_list: List<i32> = list!(1i32, 2i32, 3i32);
                let empty_list: List<i32> = List::new();
                assert_eq!(full_list.length(), 3);
                assert_eq!(empty_list.length(), 0);
            }

            #[test]
            fn test_list_to_string() {
                let l: List<i32> = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
                assert_eq!(l.to_string(), "(1, (2, (3, nil)))");
            }

            #[test]
            fn test_stack_length() {
                let full_stack: List<i32> = list!(1i32, 2i32, 3i32);
                let empty_stack: List<i32> = Stack::empty();
                assert_eq!(full_stack.length(), 3);
                assert_eq!(empty_stack.length(), 0);
            }

            #[test]
            fn test_stack_peek() {
                let full_stack: List<i32> = list!(1i32, 2i32, 3i32);
                let empty_stack: List<i32> = Stack::empty();
                assert_eq!(full_stack.peek(), Some(&1));
                assert_eq!(empty_stack.peek(), None);
            }

            #[test]
            fn test_stack_push() {
                let mut s: List<i32> = Stack::empty();
                assert_eq!(s.peek(), None);
                s = s.push(1);
                assert_eq!(s.peek(), Some(&1));
                s = s.push(6);
                assert_eq!(s.peek(), Some(&6));
            }

            #[test]
            fn test_stack_pop() {
                let mut s: List<i32> = Stack::empty();
                assert_eq!(s.peek(), None);
                s = s.push(1);
                assert_eq!(s.peek(), Some(&1));
                s = s.push(6);
                assert_eq!(s.peek(), Some(&6));
                let pop_result = s.pop().unwrap(); // should not break
                s = pop_result.1;
                assert_eq!(s.peek(), Some(&1));
                assert_eq!(pop_result.0, 6);
            }

            #[test]
            fn test_list_macro() {
                let l: List<i32> = list!(1i32, 2i32, 3i32);
                assert_eq!(l.to_string(), "(1, (2, (3, nil)))")
            }

            #[test]
            fn test_list_iter() {
                let l: List<isize> = list!(1,2,3,4,5,6);
                let mut string = String::new();
                for item in l.iter() {
                    string.push_str((item.to_string() + ", ").as_slice());
                }
                assert_eq!(string.as_slice(), "1, 2, 3, 4, 5, 6, ")
            }

        }
    }

    /// SVM cell types.
    ///
    /// A cell in the VM can be either an atom (single item, either unsigned
    /// int, signed int, float, or string) or a pointer to a list cell.

    #[derive(PartialEq,Clone,Debug)]
    pub enum SVMCell {
        AtomCell(Atom),
        ListCell(Box<List<SVMCell>>)
    }

    impl fmt::Display for SVMCell {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self)
        }
    }

    /// SVM atom types.
    ///
    /// A VM atom can be either an unsigned int, signed int, float,
    /// char, or string.
    ///
    /// TODO: Strings could be implemented as char lists rather than
    /// Rust strings.
    #[derive(PartialEq,Clone,Debug)]
    pub enum Atom {
        UInt(usize),
        SInt(isize),
        Float(f64),
        Char(char),
        Str(String),
    }

    impl fmt::Display for Atom {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                &Atom::UInt(value) => write!(f, "{}us", value),
                &Atom::SInt(value) => write!(f, "{}is", value),
                &Atom::Float(value) => write!(f, "{}f64", value),
                &Atom::Char(value) => write!(f, "'{}'", value),
                &Atom::Str(ref value) => write!(f, "\"{}\"", value)
            }
        }
    }

    /// SVM instruction types
    pub enum SVMInstruction {
        /// `nil`
        ///
        /// Pushes an empty list (nil) onto the stack
        InstNIL,
        /// `ldc`: `L`oa`d` `C`onstant. Loads a constant (atom)
        InstLDC(Atom),
        /// `ld`: `L`oa`d`. Pushes a variable onto the stack.
        ///
        /// The variable is indicated by the argument, a pair.
        /// The pair's `car` specifies the level, the `cdr` the position.
        /// So `(1 . 3)` gives the current function's (level 1) third
        /// parameter.
        InstLD,
        /// `ldf`: `L`oa`d` `F`unction.
        ///
        ///  Takes one list argument representing a function and constructs
        ///  a closure (a pair containing the function and the current
        ///  environment) and pushes that onto the stack.
        InstLDF,
        /// `join`
        ///
        /// Pops a list reference from the dump and makes this the new value
        /// of `C`. This instruction occurs at the end of both alternatives of
        ///  a `sel`.
        InstJOIN,
        /// `ap`: `Ap`ply.
        ///
        /// Pops a closure and a list of parameter values from the stack.
        /// The closure is applied to the parameters by installing its
        /// environment as the current one, pushing the parameter list
        /// in front of that, clearing the stack, and setting `C` to the
        /// closure's function pointer. The previous values of `S`, `E`,
        ///  and the next value of `C` are saved on the dump.
        InstAP,
        /// `ret`: `Ret`urn.
        ///
        /// Pops one return value from the stack, restores
        /// `S`, `E`, and `C` from the dump, and pushes
        /// the return value onto the now-current stack.
        InstRET,
        /// `dum`: `Dum`my.
        ///
        /// Pops a dummy environment (an empty list) onto the `E` stack.
        InstDUM,
        /// `rap`: `R`ecursive `Ap`ply.
        /// Works like `ap`, only that it replaces an occurrence of a
        /// dummy environment with the current one, thus making recursive
        ///  functions possible.
        InstRAP,
        /// `sel`: `Sel`ect
        ///
        /// Expects two list arguments, and pops a value from the stack.
        /// The first list is executed if the popped value was non-nil,
        /// the second list otherwise. Before one of these list pointers
        ///  is made the new `C`, a pointer to the instruction following
        ///  `sel` is saved on the dump.
        InstSEL,
        /// `add`
        ///
        /// Pops two numbers off of the stack and adds them, pushing the
        /// result onto the stack. This will up-convert integers to floating
        /// point if necessary.
        ///
        /// TODO: figure out what happens when you try to add things that aren't
        /// numbers (maybe the compiler won't let this happen?).
        InstADD,
        /// `sub`: `Sub`tract
        ///
        /// Pops two numbers off of the stack and subtracts the first from the
        /// second, pushing the result onto the stack. This will up-convert
        /// integers to floating point if necessary.
        ///
        /// TODO: figure out what happens when you try to subtract things that
        /// aren't numbers (maybe the compiler won't let this happen?).
        InstSUB,
        /// `mul`: `Mul`tiply
        ///
        /// Pops two numbers off of the stack and multiplies them, pushing the
        /// result onto the stack. This will up-convert integers to floating
        /// point if necessary.
        ///
        /// TODO: figure out what happens when you try to multiply things that
        /// aren't numbers (maybe the compiler won't let this happen?).
        InstMUL,
        /// `div`: `Div`ide
        ///
        /// Pops two numbers off of the stack and divides the first by the second,
        /// pushing the result onto the stack. This performs integer division.
        ///
        /// TODO: figure out what happens when you try to divide things that
        /// aren't numbers (maybe the compiler won't let this happen?).
        InstDIV,
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
        InstFDIV,
        /// `mod`: `Mod`ulo
        ///
        /// Pops two numbers off of the stack and divides the first by the second,
        /// pushing the remainder onto the stack.
        ///
        /// TODO: figure out what happens when you try to modulo things that
        /// aren't numbers (maybe the compiler won't let this happen?).
        InstMOD
        // TODO: add some hardcoded I/O instructions here so that you can
        //  do I/O without farming everything out to `stdio`
        // TODO: add `cons` and `cdr` words
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
        fn new() -> State {
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
        pub fn eval(self, inst: SVMInstruction) -> State {
            match inst {
                // NIL: pop an empty list onto the stack
                SVMInstruction::InstNIL => {
                    State {
                        stack: self.stack.push(SVMCell::ListCell(box List::new())),
                        env: self.env,
                        control: self.control,
                        dump: self.dump
                    }
                }
                // LDC: load constant
                SVMInstruction::InstLDC(atom) => {
                    State {
                        stack: self.stack.push(SVMCell::AtomCell(atom)),
                        env: self.env,
                        control: self.control,
                        dump: self.dump
                    }
                },
                // LD: load variable
                SVMInstruction::InstLD => {
                    let (top, new_stack) = self.stack.pop().unwrap();
                    match top {
                        SVMCell::ListCell(box Cons(
                            SVMCell::AtomCell(
                                Atom::SInt(level)
                                ),
                            box Cons(
                                SVMCell::AtomCell(
                                    Atom::SInt(pos)
                                    ),
                                box Nil
                                )
                            )
                        ) => {
                            let environment = match self.env[level-1] {
                                SVMCell::ListCell(ref l) => l.clone(),
                                _ => panic!()
                            };
                            State {
                                stack: new_stack.push(environment[pos-1].clone()),
                                env: self.env,
                                control: self.control,
                                dump: self.dump
                            }
                        },
                        _ => panic!() //TODO: put error on stack instead
                    }
                },

                // LDF: load function
                SVMInstruction::InstLDF => {
                    let (top, new_stack) = self.stack.pop().unwrap();
                    State {
                        stack: new_stack.push(SVMCell::ListCell(box list!(top,self.env[1is].clone()))),
                        env: self.env,
                        control: self.control,
                        dump: self.dump
                    }
                },
                _ => { unimplemented!() }
            }
        }
    }

    /*
    /// Evaluates a program.
    ///
    /// Evaluates a program represented as an `Iterator` of `SVMInstruction`s.
    /// Returns the final machine state at the end of execution

    pub fn evalProgram(insts: Iterator<Item=SVMInstruction>) -> State {
        insts.fold(State::new(), |last_state: State, inst: SVMInstruction| last_state.eval(inst));
    }*/

    #[cfg(test)]
    mod tests {
        use super::slist;
        use super::slist::Stack;
        use super::slist::List::{Cons,Nil};
        use super::State;
        use super::{SVMInstruction, SVMCell, Atom};
        use super::SVMCell::{AtomCell, ListCell};
        use super::Atom::{SInt, Char, Float, Str};

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
            let mut state = State::new();
            assert_eq!(state.stack.peek(), None);
            state = state.eval(SVMInstruction::InstNIL);
            assert_eq!(state.stack.peek(), Some(&SVMCell::ListCell(box Nil)));
        }

        #[test]
        fn test_eval_ldc () {
            let mut state = State::new();
            assert_eq!(state.stack.peek(), None);

            state = state.eval(SVMInstruction::InstLDC(SInt(1)));
            assert_eq!(state.stack.peek(), Some(&AtomCell(SInt(1))));

            state = state.eval(SVMInstruction::InstLDC(Char('a')));
            assert_eq!(state.stack.peek(), Some(&AtomCell(Char('a'))));

            state = state.eval(SVMInstruction::InstLDC(Float(1.0f64)));
            assert_eq!(state.stack.peek(), Some(&AtomCell(Float(1.0f64))));
        }

        #[test]
        fn test_eval_ld () {
            let mut state = State {
                stack: list!(ListCell(box list!(AtomCell(SInt(1)),AtomCell(SInt(2))))),
                env: list!(ListCell(box list!(AtomCell(Str(String::from_str("load me!"))),AtomCell(Str(String::from_str("don't load me!")))))),
                control: Stack::empty(),
                dump: Stack::empty()
            };
            state = state.eval(SVMInstruction::InstLD);
            assert_eq!(state.stack.peek(), Some(&AtomCell(Str(String::from_str("load me!")))));
        }

        #[test]
        fn test_eval_ldf () {
                let mut state = State {
                stack: list!(ListCell(box list!(AtomCell(Str(String::from_str("i'm in the function")))))),
                env: list!(
                    ListCell(
                        box list!(
                            AtomCell(
                                Str(
                                    String::from_str("load me!")
                                    )
                                ),
                            AtomCell(Str(String::from_str("load me too!")))
                        )
                    ),
                    ListCell(box list!(AtomCell(Str(String::from_str("don't load me!"))),AtomCell(Str(String::from_str("don't load me either!")))))),
                control: Stack::empty(),
                dump: Stack::empty()
            };
            state = state.eval(SVMInstruction::InstLDF);
            assert_eq!(
                state.stack.peek(),
                Some(&ListCell(
                    box list!(
                        ListCell( box list!(
                        AtomCell(
                            Str(
                                String::from_str("i'm in the function")
                                )
                            )
                        )
                        ),
                        ListCell(
                            box list!(
                                AtomCell(Str(String::from_str("load me!"))),
                                AtomCell(Str(String::from_str("load me too!")))
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
                control: Stack::empty(),
                dump: list!(ListCell(box list!(
                    AtomCell(Str(String::from_str("load me!"))),
                    AtomCell(Str(String::from_str("load me too!")))
                    )))
            };
            state = state.eval(SVMInstruction::InstJOIN);
            assert_eq!(state.dump.peek(), None);
            assert_eq!(state.control.peek(), Some(&ListCell(box list!(
                    AtomCell(Str(String::from_str("load me!"))),
                    AtomCell(Str(String::from_str("load me too!")))
                    ))))
        }

        #[test]
        fn test_atom_show () {
            let mut a: Atom;

            a = Atom::Char('a');
            assert_eq!(format!("{}", a), "'a'");

            a = Atom::UInt(1us);
            assert_eq!(format!("{}", a), "1us");

            a = Atom::SInt(42is);
            assert_eq!(format!("{}", a), "42is");

            a = Atom::Float(5.55f64);
            assert_eq!(format!("{}", a), "5.55f64");

            a = Atom::Str(String::from_str("help I'm trapped in a SECD virtual machine!"));
            assert_eq!(format!("{}", a), "\"help I'm trapped in a SECD virtual machine!\"");
        }
    }

}
