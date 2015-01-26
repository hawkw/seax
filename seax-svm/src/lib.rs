#![crate_id = "seax-svm"]
#![crate_type="lib"]
#![feature(box_syntax)]

/// Contains the Seax Virtual Machine (SVM) and miscellaneous
/// support code.
pub mod svm {
    use svm::slist::List;
    use svm::slist::Stack;
    use std::iter::IteratorExt;
    use std::fmt;

    /// Singly-linked list and stack implementations. `List<T>` is a
    /// singly-linked cons list with boxed items. `Stack<T>` is basically
    /// just a struct containing a boxed pointer to the head of a list,
    /// and some methods.
    pub mod slist {

        use svm::slist::List::{Cons,Nil};
        use std::fmt;

        /// A stack implementation wrapping a `List<T>`
        ///
        /// This is essentially just a struct containing a boxed pointer
        /// to the head of a `List<T>`, so that when items are pushed to or
        /// popped from the stack, the pointer is changed to point to the
        /// new head item. There may be saner ways of doing this.
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

            /// Returns the length of the stack. This just calls
            /// `List::length()` on the wrapped list.
            pub fn length(&self) -> isize {
                self.head.length()
            }
        }

        /// Singly-linked cons list.
        ///
        /// This is used internally to represent list primitives in the
        /// machine.
        #[deriving(Show)]
        #[deriving(PartialEq)]
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

            /// Prepends the given item to the list, returning the new head item.
            pub fn prepend(self, it: T) -> List<T> {
                Cons(it, box self)
            }

            /// Returns the length of the list.
            pub fn length (&self) -> isize {
                match *self {
                    Cons(_, ref tail) => 1 + tail.length(),
                    Nil => 0
                }
            }
        }

        impl<T> List<T> where T: fmt::Show {
            /// Return a string representation of the list
            fn to_string(&self) -> String {
                match *self {
                    Cons(ref head, ref tail) => format!("({:?}, {})", head, tail.to_string()),
                    Nil => format!("nil")
                }
            }
        }

        impl<T> fmt::Show for List<T> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.to_string)
            }
        }


        /// Convenience macro for making lists.
        ///
        /// Usage: `list!(1i32, 2i32, 3i32);` expands to
        /// `Cons(1i32, Box::new(Cons(2i32, Box::new(Cons(3i32, Box::new(Nil))))));`.
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
                assert_eq!(l.to_string(), "(1i32, (2i32, (3i32, nil)))");
            }

            #[test]
            fn test_stack_length() {
                let full_stack: Stack<i32> = Stack::new(list!(1i32, 2i32, 3i32));
                let empty_stack: Stack<i32> = Stack::empty();
                assert_eq!(full_stack.length(), 3);
                assert_eq!(empty_stack.length(), 0);
            }

            #[test]
            fn test_stack_peek() {
                let full_stack: Stack<i32> = Stack::new(list!(1i32, 2i32, 3i32));
                let empty_stack: Stack<i32> = Stack::empty();
                assert_eq!(full_stack.peek(), Some(&1));
                assert_eq!(empty_stack.peek(), None);
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
                assert_eq!(s.peek(), None); // TODO: implement
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
    #[deriving(Show)]
    #[deriving(PartialEq)]
    pub enum SVMCell {
        Atom,
        ListCell(Box<List<SVMCell>>)
    }

    pub enum Atom {
        AtomUInt(usize),
        AtomSInt(isize),
        AtomFloat(f64),
        AtomStr(String)
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
        stack: Stack<SVMCell>,
        env: Stack<SVMCell>,
        control: Stack<SVMCell>,
        dump: Stack<SVMCell>
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
                SVMInstruction::InstNIL => State {
                    stack: self.stack.push(SVMCell::ListCell(box List::new())),
                    env: self.env,
                    control: self.control,
                    dump: self.dump
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
        use super::State;
        use super::SVMInstruction;
        use super::slist::List::{Cons,Nil};

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
            assert_eq!(state.stack.peek(), Some(&Nil));
        }
    }

}
