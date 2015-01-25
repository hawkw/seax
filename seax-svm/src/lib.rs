#![crate_id = "seax-svm"]
#![crate_type="lib"]

//! Contains the Seax Virtual Machine (SVM) and miscellaneous
//! support code.

#![feature(box_syntax)]
#![allow(dead_code)]

pub mod svm {

    use svm::slist::List;
    use svm::slist::Stack;

    /// Singly-linked list and stack implementations.
    pub mod slist {

        use svm::slist::List::{Cons,Nil};
        use std::fmt::Show;

        /// A stack implementation wrapping a List<T>
        pub struct Stack<T> {
            /// The head item of the list.
            head: Box<List<T>>
        }

        impl<T> Stack<T> {
            /// Push an item to the top of the stack, returning a new stack
            pub fn push(self, it: T) -> Stack<T> {
                Stack { head: Box::new(self.head.prepend(it)) }
            }

            /// Peak at the top item of the stack.
            ///
            /// Returns Some<T> if there is an item on top of the stack,
            /// and None if the stack is empty.
            pub fn peek(&self) -> Option<&T> {
                match *self.head {
                    Nil => None,
                    Cons(ref it,_) => Some(it)
                }
            }

            /// Returns an empty stack.
            pub fn empty() -> Stack<T> {
                Stack { head: box Nil }
            }

            /// Wraps a list into a stack.
            pub fn new(l: List<T>) -> Stack<T> {
                Stack { head: box l }
            }
        }

        /// Singly-linked cons list.
        ///
        /// This is used internally to represent list primitives in the machine.
        pub enum List<T> {
            Cons(T, Box<List<T>>),
            Nil,
        }

        /// Public implementation for List.
        impl<T> List<T> {


            /// Creates a new empty list
            pub fn new() -> List<T> {
                Nil
            }

            /// Prepends the given item to the list
            pub fn prepend(self, it: T) -> List<T> {
                Cons(it, box self)
            }

            /// Returns the length of the list
            pub fn length (&self) -> i32 {
                match *self {
                    Cons(_, ref tail) => 1 + tail.length(),
                    Nil => 0
                }
            }
        }

        impl<T> List<T> where T: Show {
            /// Return a string representation of the list
            fn to_string(&self) -> String {
                match *self {
                    Cons(ref head, ref tail) => format!("({:?}, {})", head, tail.to_string()),
                    Nil => format!("nil")
                }
            }

        }

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
                let l: List<i32> = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
                assert_eq!(l.length(), 3);
            }

            #[test]
            fn test_list_to_string() {
                let l: List<i32> = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
                assert_eq!(l.to_string(), "(1i32, (2i32, (3i32, nil)))");
            }

            #[test]
            fn test_stack_peek() {
                let s: Stack<i32> = Stack::new(Cons(1, Box::new(Cons(2, Box::new(Nil)))));
                assert_eq!(s.peek(), Some(&1));
            }

            #[test]
            fn test_stack_push() {
                let mut s: Stack<i32> = Stack::empty();
                assert_eq!(s.peek(), None);
                s = s.push(1);
                assert_eq!(s.peek(), Some(&1));
                s = s.push(6);
                assert_eq!(s.peek(), Some(&6));
            }

            #[test]
            fn test_stack_pop() {
                let mut s: Stack<i32> = Stack::empty();
                assert_eq!(s.peek(), None);
                s = s.push(1);
                assert_eq!(s.peek(), Some(&1));
                s = s.push(6);
                assert_eq!(s.peek(), Some(&6));
            }

            #[test]
            fn test_list_macro() {
                let l: List<i32> = list!(1i32, 2i32, 3i32);
                assert_eq!(l.to_string(), "(1i32, (2i32, (3i32, nil)))")
            }
        }
    }

    /// SVM item types
    pub enum SVMCell {
        Atom,
        ListCell(Box<List<SVMCell>>)
    }

    pub enum Atom {
        AtomUInt(usize),
        AtomSInt(isize),
        AtomFloat(f64),
        AtomStr(str)
    }

    /// SVM instruction types
    pub enum SVMInstruction {
        InstNIL,
        InstLDC(Atom),
        InstLD,
        InstLDF,
        InstJOIN,
        InstAP,
        InstRET,
        InstDUM,
        InstRAP
    }

    /// Represents a SVM machine state
    pub struct State {
        stack: Stack<SVMCell>,
        env: Stack<SVMCell>,
        control: Stack<SVMCell>,
        dump: Stack<SVMCell>
    }
}
