#![feature(box_syntax)]
#![feature(macro_rules)]

pub mod svm {

    use svm::slist::List;

    mod slist {

        use svm::slist::List::{Cons,Nil};
        use std::fmt::Show;

        pub struct Stack<T> {
            head: Box<List<T>>
        }

        impl<T> Stack<T> {
            fn push(mut self, it: T) -> Stack<T> {
                Stack { head: Box::new(self.head.prepend(it)) }
            }

            fn peek(&self) -> Option<&T> {
                match *self.head {
                    Nil => None,
                    Cons(ref it,_) => Some(it)
                }
            }

            fn empty() -> Stack<T> {
                Stack { head: box Nil }
            }

            fn new(l: List<T>) -> Stack<T> {
                Stack { head: box l }
            }
        }

        /// Singly-linked cons list.
        ///
        /// This is used internally to represent list primitives in the machine.
        #[deriving(Show)]
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
            ( $e:expr, $($rest:expr),+ ) => ( (Cons($e, Box::new(list!( $( $rest ),+ )) )));
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

    enum Exp {
        Number(i32)
    }

    struct Engine {
        stack: List<Exp>
    }
}
