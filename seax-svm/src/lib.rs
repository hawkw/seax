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

        fn new() -> List<T> {
            List::Nil
        }

    }

    enum Exp {
        Number(i32)
    }

    struct Engine {
        stack: Vec<Exp>
    }
}
