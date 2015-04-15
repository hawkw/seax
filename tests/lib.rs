#![feature(box_syntax,box_patterns)]
#![feature(compile)]
#![feature(scheme)]

#[macro_use]
extern crate seax_svm as svm;
extern crate seax_scheme as scheme;

use svm::slist::Stack;
use svm::slist::List::{Cons,Nil};
use svm::cell::Atom::*;
use svm::cell::SVMCell::*;

/// Test for simple list construction through CONS.
///
/// ```lisp
/// (cons 10 (cons 20 nil))
/// ==> (10 . 20)
/// ```
#[test]
fn run_list_construction() {
    assert_eq!(
        svm::eval_program(scheme::compile("(cons 10 (cons 20 nil))").unwrap(), true).peek(),
        Some(&ListCell( box list!(AtomCell(SInt(10)), AtomCell(SInt(20))) ))
    )
}


/// Test for simple list construction and deconstruction
///
/// ```lisp
/// (car (cons 20 (cons 10 nil)))
/// ==> 20
/// ```
#[test]
fn run_list_car() {
    assert_eq!(
        svm::eval_program(scheme::compile("(car (cons 20 (cons 10 nil)))").unwrap(), true).peek(),
        Some(&AtomCell(SInt(20)))
    )
}

/// Test for simple list construction and destructuring
///
/// ```lisp
/// (cdr (cons 20 (cons 10 nil)))
/// ==> (10)
/// ```
#[test]
fn run_list_cdr() {
    assert_eq!(
        svm::eval_program(scheme::compile("(cdr (cons 20 (cons 10 nil)))").unwrap(), true).peek(),
        Some(&ListCell(box list!(AtomCell(SInt(10)))))
    )
}

/// Test for simple mathematics application
///
/// ```lisp
/// (+ 10 10)
/// ==> 20
/// ```
#[test]
fn run_simple_add() {
    assert_eq!(
        svm::eval_program(scheme::compile("(+ 10 10)").unwrap(), true).peek(),
        Some(&AtomCell(SInt(20)))
    )
}

/// Test for nested arithmetic
///
/// ```lisp
/// (- 20 (+ 5 5))
/// ==> 10
/// ```
#[test]
fn run_nested_arith() {
    assert_eq!(
        svm::eval_program(scheme::compile("(- 20 (+ 5 5))").unwrap(), true).peek(),
        Some(&AtomCell(SInt(10)))
    )
}

/// Test for basic branching with `if` expressions.
///
/// ```lisp
/// ((if (= 0 (- 1 1)) #t #f)
/// ==> #t
/// ```
#[test]
fn run_basic_branching_1() {
    assert_eq!(
        svm::eval_program(scheme::compile("(if (= 0 (- 1 1)) #t #f)").unwrap(), true).peek(),
        Some(&AtomCell(SInt(1)))
    )
}

/// Test for basic branching with `if` expressions.
///
/// ```lisp
/// (+ 10 (if (nil? nil) 10 20))
/// ==> 20
/// ```
#[test]
fn run_basic_branching_2() {
    assert_eq!(
        svm::eval_program(scheme::compile("(+ 10 (if (nil? nil) 10 20))").unwrap(), true).peek(),
        Some(&AtomCell(SInt(20)))
    )
}

/// Test for applying a lambda expression
///
/// ```lisp
/// ((lambda (x y) (+ x y)) 2 3)
/// ==> 5
/// ```
#[test]
fn run_lambda_ap() {
    assert_eq!(
        svm::eval_program(scheme::compile("((lambda (x y) (+ x y)) 2 3)").unwrap(), true).peek(),
        Some(&AtomCell(SInt(5)))
    )
}

/// Test for applying an expression with nested lambdas
///
/// ```lisp
/// ((lambda (z) ((lambda (x y) (+ (- x y) z)) 3 5)) 6)
/// ==> 4
/// ```
#[test]
fn run_nested_lambda() {
    assert_eq!(
        svm::eval_program(scheme::compile("((lambda (z) ((lambda (x y) (+ (- x y) z)) 3 5)) 6)").unwrap(), true).peek(),
        Some(&AtomCell(SInt(4)))
    )
}