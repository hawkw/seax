#![crate_name = "seax_svm"]
#![crate_type = "lib"]
#![feature(box_syntax)]
#![feature(core)]

/// Contains the Seax Virtual Machine (SVM) and miscellaneous
/// support code.
pub mod svm {
    use svm::slist::List;
    use svm::slist::List::{Cons,Nil};
    use svm::slist::Stack;
    //use std::iter::IteratorExt;
    use std::fmt;
    use svm::Inst::*;
    use svm::SVMCell::*;
    use svm::Atom::*;

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

            /// Get the next element from the list.
            ///
            /// Get the next element from the list. Returns a `Some<T>`, or `Nil`
            /// if at the end of the list.
            ///
            /// # Examples:
            /// ```
            /// # #[macro_use] extern crate seax_svm;
            /// # use seax_svm::svm::slist;
            /// # use seax_svm::svm::slist::List;
            /// # use seax_svm::svm::slist::List::{Cons, Nil};
            /// # fn main () {
            /// let list = list!(1,2,3);
            /// let mut iter = list.iter();
            /// assert_eq!(iter.next().unwrap(), &1);
            /// assert_eq!(iter.next().unwrap(), &2);
            /// assert_eq!(iter.next().unwrap(), &3);
            /// # }
            /// ```
            /// ```
            /// # #[macro_use] extern crate seax_svm;
            /// # use seax_svm::svm::slist;
            /// # use seax_svm::svm::slist::List;
            /// # use seax_svm::svm::slist::List::{Cons, Nil};
            /// # fn main () {
            /// let l: List<isize> = list!(1,2,3,4,5,6);
            /// let mut string = String::new();
            /// for item in l.iter() {
            ///     string.push_str((item.to_string() + ", ").as_slice());
            /// }
            /// assert_eq!(string.as_slice(), "1, 2, 3, 4, 5, 6, ")
            /// # }
            /// ```
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
        /// Implementation of indexing for `List<T>`.
        ///
        /// # Examples:
        /// ```
        /// # #[macro_use] extern crate seax_svm;
        /// # use seax_svm::svm::slist;
        /// # use seax_svm::svm::slist::List;
        /// # use seax_svm::svm::slist::List::{Cons, Nil};
        /// # fn main () {
        /// let list = list!(1,2,3,4,5,6);
        /// assert_eq!(list[1us], 1);
        /// # }
        /// ```
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
        /// Implementation of indexing for `List<T>`.
        ///
        /// # Examples:
        /// ```
        /// # #[macro_use] extern crate seax_svm;
        /// # use seax_svm::svm::slist;
        /// # use seax_svm::svm::slist::List;
        /// # use seax_svm::svm::slist::List::{Cons, Nil};
        /// # fn main () {
        /// let list = list!(1,2,3,4,5,6);
        /// assert_eq!(list[1is], 1);
        /// # }
        /// ```
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
    /// char, bool, or string.
    ///
    /// TODO: Strings could be implemented as char lists rather than
    /// Rust strings.
    #[derive(PartialEq,Clone,Debug)]
    pub enum Atom {
        /// Unsigned integer atom (machine size)
        UInt(usize),
        /// Signed integer atom (machine size)
        SInt(isize),
        /// Floating point number atom (64-bits)
        Float(f64),
        /// UTF-8 character atom
        Char(char),
        /// String atom
        ///
        /// TODO: this should be implemented as a list of char atoms
        Str(String), // todo: string is uncopyable
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
                &Atom::Str(ref value) => write!(f, "\"{}\"", value),
                &Atom::Bool(value) => write!(f, "{}", value)
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
                    let (op2, newer_stack) = new_stack.pop().unwrap();
                    match (op1.clone(),op2.clone()) { // TODO: rather not clone every time I want to add two ints
                        (AtomCell(SInt(a)), AtomCell(SInt(b))) => State {
                            stack: newer_stack.push(AtomCell(SInt(a + b))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(UInt(a)), AtomCell(UInt(b))) => State {
                            stack: newer_stack.push(AtomCell(UInt(a + b))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(Float(a)), AtomCell(Float(b))) => State {
                            stack: newer_stack.push(AtomCell(Float(a + b))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(Float(a)), AtomCell(SInt(b))) => State {
                            stack: newer_stack.push(AtomCell(Float(a + b as f64))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(Float(a)), AtomCell(UInt(b))) => State {
                            stack: newer_stack.push(AtomCell(Float(a + b as f64))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(SInt(a)), AtomCell(Float(b))) => State {
                            stack: newer_stack.push(AtomCell(Float(a as f64 + b))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(UInt(a)), AtomCell(Float(b))) => State {
                            stack: newer_stack.push(AtomCell(Float(a as f64 + b))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (_,_) =>  panic!("[ADD] TypeError: expected compatible operands, found (ADD {:?} {:?})", op1, op2)
                    }
                },
                InstCell(SUB) => {
                    let (op1, new_stack) = self.stack.pop().unwrap();
                    let (op2, newer_stack) = new_stack.pop().unwrap();
                    match (op1.clone(),op2.clone()) { // TODO: rather not clone every time I want to subtract two ints
                        (AtomCell(SInt(a)), AtomCell(SInt(b))) => State {
                            stack: newer_stack.push(AtomCell(SInt(a - b))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(UInt(a)), AtomCell(UInt(b))) => State {
                            stack: newer_stack.push(AtomCell(UInt(a - b))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(Float(a)), AtomCell(Float(b))) => State {
                            stack: newer_stack.push(AtomCell(Float(a - b))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(Float(a)), AtomCell(SInt(b))) => State {
                            stack: newer_stack.push(AtomCell(Float(a - b as f64))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(Float(a)), AtomCell(UInt(b))) => State {
                            stack: newer_stack.push(AtomCell(Float(a - b as f64))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(SInt(a)), AtomCell(Float(b))) => State {
                            stack: newer_stack.push(AtomCell(Float(a as f64 - b))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(UInt(a)), AtomCell(Float(b))) => State {
                            stack: newer_stack.push(AtomCell(Float(a as f64 - b))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (_,_) =>  panic!("[SUB] TypeError: expected compatible operands, found (SUB {:?} {:?})", op1, op2)
                    }
                },
                InstCell(MUL) => {
                    let (op1, new_stack) = self.stack.pop().unwrap();
                    let (op2, newer_stack) = new_stack.pop().unwrap();
                    match (op1.clone(),op2.clone()) { // TODO: rather not clone every time I want to multiply two ints
                        (AtomCell(SInt(a)), AtomCell(SInt(b))) => State {
                            stack: newer_stack.push(AtomCell(SInt(a * b))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(UInt(a)), AtomCell(UInt(b))) => State {
                            stack: newer_stack.push(AtomCell(UInt(a * b))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(Float(a)), AtomCell(Float(b))) => State {
                            stack: newer_stack.push(AtomCell(Float(a * b))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(Float(a)), AtomCell(SInt(b))) => State {
                            stack: newer_stack.push(AtomCell(Float(a * b as f64))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(Float(a)), AtomCell(UInt(b))) => State {
                            stack: newer_stack.push(AtomCell(Float(a * b as f64))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(SInt(a)), AtomCell(Float(b))) => State {
                            stack: newer_stack.push(AtomCell(Float(a as f64 * b))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(UInt(a)), AtomCell(Float(b))) => State {
                            stack: newer_stack.push(AtomCell(Float(a as f64 * b))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (_,_) =>  panic!("[MUL] TypeError: expected compatible operands, found (MUL {:?} {:?})", op1, op2)
                    }
                },
                InstCell(DIV) => {
                    let (op1, new_stack) = self.stack.pop().unwrap();
                    let (op2, newer_stack) = new_stack.pop().unwrap();
                    match (op1.clone(),op2.clone()) { // TODO: rather not clone every time I want to divide two ints
                        (AtomCell(SInt(a)), AtomCell(SInt(b))) => State {
                            stack: newer_stack.push(AtomCell(SInt(a / b))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(UInt(a)), AtomCell(UInt(b))) => State {
                            stack: newer_stack.push(AtomCell(UInt(a / b))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(Float(a)), AtomCell(Float(b))) => State {
                            stack: newer_stack.push(AtomCell(Float(a / b))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(Float(a)), AtomCell(SInt(b))) => State {
                            stack: newer_stack.push(AtomCell(Float(a / b as f64))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(Float(a)), AtomCell(UInt(b))) => State {
                            stack: newer_stack.push(AtomCell(Float(a / b as f64))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(SInt(a)), AtomCell(Float(b))) => State {
                            stack: newer_stack.push(AtomCell(Float(a as f64 / b))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(UInt(a)), AtomCell(Float(b))) => State {
                            stack: newer_stack.push(AtomCell(Float(a as f64 / b))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (_,_) =>  panic!("[DIV] TypeError: expected compatible operands, found (DIV {:?} {:?})", op1, op2)
                    }
                },
                InstCell(FDIV) => {
                    let (op1, new_stack) = self.stack.pop().unwrap();
                    let (op2, newer_stack) = new_stack.pop().unwrap();
                    match (op1.clone(),op2.clone()) { // TODO: rather not clone every time I want to divide two ints
                        (AtomCell(SInt(a)), AtomCell(SInt(b))) => State {
                            stack: newer_stack.push(AtomCell(Float(a as f64 / b as f64))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(UInt(a)), AtomCell(UInt(b))) => State {
                            stack: newer_stack.push(AtomCell(Float(a as f64 / b as f64))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(Float(a)), AtomCell(Float(b))) => State {
                            stack: newer_stack.push(AtomCell(Float(a / b))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(Float(a)), AtomCell(SInt(b))) => State {
                            stack: newer_stack.push(AtomCell(Float(a / b as f64))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(Float(a)), AtomCell(UInt(b))) => State {
                            stack: newer_stack.push(AtomCell(Float(a / b as f64))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(SInt(a)), AtomCell(Float(b))) => State {
                            stack: newer_stack.push(AtomCell(Float(a as f64 / b))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(UInt(a)), AtomCell(Float(b))) => State {
                            stack: newer_stack.push(AtomCell(Float(a as f64 / b))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (_,_) =>  panic!("[FDIV] TypeError: expected compatible operands, found (FDIV {:?} {:?})", op1, op2)
                    }
                },
                InstCell(MOD) => {
                    let (op1, new_stack) = self.stack.pop().unwrap();
                    let (op2, newer_stack) = new_stack.pop().unwrap();
                    match (op1.clone(),op2.clone()) { // TODO: rather not clone every time I want to divide two ints
                        (AtomCell(SInt(a)), AtomCell(SInt(b))) => State {
                            stack: newer_stack.push(AtomCell(SInt(a % b))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(UInt(a)), AtomCell(UInt(b))) => State {
                            stack: newer_stack.push(AtomCell(UInt(a % b))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(Float(a)), AtomCell(Float(b))) => State {
                            stack: newer_stack.push(AtomCell(Float(a % b))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(Float(a)), AtomCell(SInt(b))) => State {
                            stack: newer_stack.push(AtomCell(Float(a % b as f64))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(Float(a)), AtomCell(UInt(b))) => State {
                            stack: newer_stack.push(AtomCell(Float(a % b as f64))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(SInt(a)), AtomCell(Float(b))) => State {
                            stack: newer_stack.push(AtomCell(Float(a as f64 % b))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (AtomCell(UInt(a)), AtomCell(Float(b))) => State {
                            stack: newer_stack.push(AtomCell(Float(a as f64 % b))),
                            env: self.env,
                            control: new_control,
                            dump: self.dump
                        },
                        (_,_) =>  panic!("[MOD] TypeError: expected compatible operands, found (MOD {:?} {:?})", op1, op2)
                    }
                },
                InstCell(EQ) => {
                    let (op1, new_stack) = self.stack.pop().unwrap();
                    let (op2, newer_stack) = new_stack.pop().unwrap();
                    match (op1.clone(), op2.clone()) {
                        (AtomCell(a), AtomCell(b)) => State {
                            stack: newer_stack.push(AtomCell(Bool(a == b))),
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
                env: list!(ListCell(box list!(AtomCell(Str(String::from_str("load me!"))),AtomCell(Str(String::from_str("don't load me!")))))),
                control: list!(InstCell(LD),ListCell(box list!(AtomCell(SInt(1)),AtomCell(SInt(2))))),
                dump: Stack::empty()
            };
            state = state.eval();
            assert_eq!(state.stack.peek(), Some(&AtomCell(Str(String::from_str("load me!")))));
        }

        #[test]
        fn test_eval_ldf () {
                let mut state = State {
                stack: Stack::empty(),
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
                control: list!(InstCell(LDF), ListCell(box list!(AtomCell(Str(String::from_str("i'm in the function")))))),
                dump: Stack::empty()
            };
            state = state.eval();
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
                control: list!(InstCell(JOIN)),
                dump: list!(ListCell(box list!(
                        AtomCell(Str(String::from_str("load me!"))),
                        AtomCell(Str(String::from_str("load me too!")))
                    )))
            };
            state = state.eval();
            assert_eq!(state.dump.peek(), None);
            assert_eq!(state.control.peek(), Some(&ListCell(box list!(
                    AtomCell(Str(String::from_str("load me!"))),
                    AtomCell(Str(String::from_str("load me too!")))
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

            a = Str(String::from_str("help I'm trapped in a SECD virtual machine!"));
            assert_eq!(format!("{}", a), "\"help I'm trapped in a SECD virtual machine!\"");
        }
    }

}
