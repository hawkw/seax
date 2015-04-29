% The Cons List

In order to understand both the Lisp programming language and the SECD machine, it is necessary to understand the singly-linked list, or `cons` list. 

## The Cons Cell

The `cons` list is one of the most conceptually simple data structures possible. It consists of one fundamental function, the eponymous `cons`, short for <i>cons</i>truct. The `cons` function creates memory objects, called _`cons` cells_ or _pairs_, which contain two values or pointers.

Typically, a `cons` cell is written using the 'dotted pair' notation `(a . b)`, where `a` refers to the first element, or `car`, and `b` refers to the second element, or `cdr`. These (admittedly rather arcane) names are acronyms, meaning '<i>C</i>ontents of the <i>A</i>ddress part of the <i>R</i>egister number' and '<i>C</i>ontents of the <i>D</i>ecrement part of the <i>R</i>egister number', which refer to hardware instructions on the [IBM 704](http://en.wikipedia.org/wiki/IBM_704) computers on which Lisp was first implemented in the late 1950s. Although these hardware instructions are now lost to the mists of time, their names are now enshrined in computing lore thanks to Lisp. We will, therefore, continue to use this notation.

## The Singly-Linked List

Despite its' simplicity, the `cons` cell may be used to implement a variety of data structures, such as lists and trees. We are concerned primarily with the singly-linked list.

In order to implement a singly-linked list using the `cons` cell, we simply define a list of items as being either of the following:

+ A `cons` cell whose `car` is a pointer to an item and whose tail is a pointer to a list
+ The empty list, called `nil`

In programming languages which support algebraic data types, defining such a structure is almost trivially easy. In Rust, it looks something like this:

```rust
pub enum List<T> {
    /// Cons cell containing a `T` and a link to the tail
    Cons(T, Box<List<T>>),
    /// The empty list.
    Nil
}
```

Do note that the Rust implementation is somewhat more complicated due to Rust's memory management and typing discipline. In Scheme, we can say, even more simply:

```scheme
(define (cons a b) (lambda (x) (x a b)))
```

Therefore, by repeatedly `cons`ing 