#![feature(box_syntax)]
#[test]
fn it_works() {
}


pub mod svm {
    mod slist {
        use svm::slist::List::{Cons,Nil};
        /// Singly-linked cons list.
        ///
        /// This is used internally to represent list primitives in the machine.
        enum List<T> {
            Nil,
            Cons(T, Box<List<T>>)
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
    }

    enum Exp {
        Number(i32)
    }

    struct Engine {
        stack: Vec<Exp>
    }
}
