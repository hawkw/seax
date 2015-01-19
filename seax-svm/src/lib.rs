#[test]
fn it_works() {
}


mod SVM {
    /// Singly-linked cons list.
    ///
    /// This is used internally to represent list primitives in the machine.
    pub enum List<T> {
        Cons(T ~List<T>),
        Nil
    }

    /// Public implementation for List.
    pub impl List<T> {

        pub fn new() -> List<T> {
            Nil
        }

    }

    enum Exp {
        Number(int)
    }

    struct Engine {
        stack: Vec<Exp>
    }
}
