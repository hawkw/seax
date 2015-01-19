#![feature(box_syntax)]
#[test]
fn it_works() {
}


pub mod svm {
    mod slist {

        use svm::slist::List::{Cons,Nil};
        use std::fmt::Show;

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
            fn new() -> List<T> {
                Nil
            }

            /// Prepends the given item to the list
            fn prepend(self, it: T) -> List<T> {
                Cons(it, box self)
            }

            /// Returns the length of the list
            fn length (&self) -> i32 {
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

        #[cfg(test)]
        mod tests {
            use super::List;
            use super::List::{Cons,Nil};

            #[test]
            fn test_length() {
                let l: List<i32> = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
                assert_eq!(l.length(), 3);
            }

             #[test]
            fn test_stringify() {
                let l: List<i32> = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
                assert_eq!(l.to_string(), "(1i32, (2i32, (3i32, nil)))");
            }
        }
    }

    enum Exp {
        Number(i32)
    }

    struct Engine {
        stack: Vec<Exp>
    }
}
