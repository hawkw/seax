#![feature(box_syntax)]
#[test]
fn it_works() {
}


mod SVM {

    /// Singly-linked cons list.
    ///
    /// This is used internally to represent list primitives in the machine.
    pub enum List<T> {
        Nil,
        Cons(T, Box<List<T>>)
    }

    /// Public implementation for List.
    impl<T> List<T> {

        /// Creates a new empty list
        fn new() -> List<T> {
            List::Nil
        }

        /// Prepends the given item to the list
        fn prepend(self, it: T) -> List<T> {
            List::Cons(it, box self)
        }

        /// Returns the length of the list
        fn length (&self) -> i32 {
            match *self {
                List::Cons(_, ref tail) => 1 + tail.length(),
                List::Nil => 0
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
