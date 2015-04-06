pub use slist::List::{Cons,Nil};
use std::fmt;
use std::ops::Index;

/// Convenience macro for making lists.
///
/// # Example:
///
/// ```
/// # #[macro_use] extern crate seax_svm;
/// # use seax_svm::slist;
/// # use seax_svm::slist::List::{Cons, Nil};
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
    () => ( Box::new(Nil) );
);

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
    /// use seax_svm::slist::{List,Stack};
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
    /// # use seax_svm::slist::{List,Stack};
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
    /// # use seax_svm::slist::{List,Stack};
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

    pub fn last(&self) -> &T {
        match *self {
            Cons(ref car, box Nil) => &car,
            Cons(_, ref cdr @ box Cons(_,_)) => cdr.last(),
            Nil => panic!("Last called on empty list")
        }
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
    /// # use seax_svm::slist;
    /// # use seax_svm::slist::List;
    /// # use seax_svm::slist::List::{Cons, Nil};
    /// # fn main () {
    /// let list = list!(1,2,3);
    /// let mut iter = list.iter();
    /// assert_eq!(iter.next().unwrap(), &1);
    /// assert_eq!(iter.next().unwrap(), &2);
    /// assert_eq!(iter.next().unwrap(), &3);
    /// # }
    /// ```
    /// ```
    /// # #![feature(convert)]
    /// # #[macro_use] extern crate seax_svm;
    /// # use seax_svm::slist;
    /// # use seax_svm::slist::List;
    /// # use seax_svm::slist::List::{Cons, Nil};
    /// # fn main () {
    /// let l: List<isize> = list!(1,2,3,4,5,6);
    /// let mut string = String::new();
    /// for item in l.iter() {
    ///     string.push_str((item.to_string() + ", ").as_ref());
    /// }
    /// assert_eq!(string, "1, 2, 3, 4, 5, 6, ".to_string())
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
/// # use seax_svm::slist;
/// # use seax_svm::slist::List;
/// # use seax_svm::slist::List::{Cons, Nil};
/// # fn main () {
/// let list = list!(1,2,3,4,5,6);
/// assert_eq!(list[0us], 1);
/// # }
/// ```
impl<T> Index<usize> for List<T> {
    type Output = T;

    fn index<'a>(&'a self, _index: usize) -> &'a T {
        match _index {
            0usize => match *self {
                Cons(ref car, _) => car,
                Nil => panic!("List index {} out of range", _index)
            },
            1usize => match *self {
                Cons(_, box Cons(ref cdr, _)) => cdr,
                Cons(_, box Nil) => panic!("List index {} out of range", _index),
                Nil => panic!("List index {} out of range", _index)
            },
            i if i == self.length() => self.last(),
            i if i > self.length()  => panic!("List index {:?} out of range.", _index),
            i if i > 1usize => {
                let mut it = self.iter();
                for _ in 0usize .. i{
                    it.next();
                }
                it.next().unwrap()
            },
            _ => panic!("Expected an index i such that i >= 0, got {:?}.", _index)
        }
    }
}
/// Implementation of indexing for `List<T>`.
///
/// # Examples:
/// ```
/// # #[macro_use] extern crate seax_svm;
/// # use seax_svm::slist;
/// # use seax_svm::slist::List;
/// # use seax_svm::slist::List::{Cons, Nil};
/// # fn main () {
/// let list = list!(1,2,3,4,5,6);
/// assert_eq!(list[0is], 1);
/// # }
/// ```
impl<T> Index<isize> for List<T> {
    type Output = T;

    fn index<'a>(&'a self, _index: isize) -> &'a T {
        match _index {
            0isize => match *self {
                Cons(ref car, _) => car,
                Nil => panic!("List index {} out of range", _index)
            },
            1isize => match *self {
                Cons(_, box Cons(ref cdr, _)) => cdr,
                Cons(_, box Nil) => panic!("List index {} out of range", _index),
                Nil => panic!("List index {} out of range", _index)
            },
            i if i == self.length() as isize => self.last(),
            i if i > self.length() as isize => panic!("List index {:?} out of range.", _index),
            i if i > 1isize => {
                let mut it = self.iter();
                for _ in 0isize .. i{
                    it.next();
                }
                it.next().unwrap()
            },
            _ => panic!("Expected an index i such that i >= 0, got {:?}.", _index)
        }
    }
}

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
    fn test_list_usize_indexing() {
        let l: List<isize> = list!(1,2,3,4,5,6);
        assert_eq!(l[0usize],1);
        assert_eq!(l[1usize],2);
        assert_eq!(l[2usize],3);
        assert_eq!(l[3usize],4);
        assert_eq!(l[4usize],5);
        assert_eq!(l[5usize],6);
    }

    #[test]
    fn test_list_isize_indexing() {
        let l: List<isize> = list!(1,2,3,4,5,6);
        assert_eq!(l[0isize],1);
        assert_eq!(l[1isize],2);
        assert_eq!(l[2isize],3);
        assert_eq!(l[3isize],4);
        assert_eq!(l[4isize],5);
        assert_eq!(l[5isize],6);
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
            string.push_str((item.to_string() + ", ").as_ref());
        }
        let slice: &str = string.as_ref(); // this is necessary because assert_eq! is weird
        assert_eq!(slice, "1, 2, 3, 4, 5, 6, ")
    }

}
