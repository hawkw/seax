
#![feature(box_syntax,box_patterns)]
extern crate seax_svm;

use seax_svm::svm::slist::Stack;
use seax_svm::svm::slist::List::{Cons,Nil};
use seax_svm::svm::cell::Atom::*;
use seax_svm::svm::cell::SVMCell::*;
use seax_svm::svm::Inst::*;

macro_rules! list(
    ( $e:expr, $($rest:expr),+ ) => ( Cons($e, Box::new(list!( $( $rest ),+ )) ));
    ( $e:expr ) => ( Cons($e, Box::new(Nil)) );
    () => ( Box::new(Nil) );
);

/// SVM integration tests.
///
/// These are based on the sample programs in Zach Allaun's Clojure SECD
/// [implementation](https://github.com/zachallaun/secd). Each example also
/// provides the source code for the equivalent Lisp program.

/// Test for simple list construction through CONS.
///
/// ```lisp
/// (cons 10 (cons 20 nil))
/// ```
#[test]
fn test_list_creation() {
    assert_eq!(
        seax_svm::svm::eval_program(list!(
            InstCell(NIL),
            InstCell(LDC), AtomCell(SInt(10)), InstCell(CONS),
            InstCell(LDC), AtomCell(SInt(20)), InstCell(CONS)
        )).peek(),
        Some(&ListCell( box list!(AtomCell(SInt(20)), AtomCell(SInt(10)))))
    );
}

/// Test for simple list construction and destructuring
///
/// ```lisp
/// (car (cons 10 (cons 20 nil)))
/// ```
#[test]
fn test_list_car() {
    assert_eq!(
        seax_svm::svm::eval_program(list!(
            InstCell(NIL),
            InstCell(LDC), AtomCell(SInt(10)), InstCell(CONS),
            InstCell(LDC), AtomCell(SInt(20)), InstCell(CONS),
            InstCell(CAR)
        )).peek(),
        Some(&AtomCell(SInt(20)))
    );
}
/// Test for simple list construction and destructuring
///
/// ```lisp
/// (cdr (cons 10 (cons 20 nil)))
/// ```
#[test]
fn test_list_cdr() {
    assert_eq!(
        seax_svm::svm::eval_program(list!(
            InstCell(NIL),
            InstCell(LDC), AtomCell(SInt(10)), InstCell(CONS),
            InstCell(LDC), AtomCell(SInt(20)), InstCell(CONS),
            InstCell(CDR)
        )).peek(),
        Some(&ListCell(box list!(AtomCell(SInt(10)))))
    );
}

/// Test for simple mathematics application
///
/// ```lisp
/// (+ 10 10)
/// ```
#[test]
fn test_simple_add() {
    assert_eq!(
        seax_svm::svm::eval_program(list!(
            InstCell(LDC), AtomCell(SInt(10)),
            InstCell(LDC), AtomCell(SInt(10)),
            InstCell(ADD)
        )).peek(),
        Some(&AtomCell(SInt(20)))
    );
}

/// Test for nested arithmetic
///
/// ```lisp
/// (- 20 (+ 5 5))
/// ```
#[test]
fn test_nested_arith() {
     assert_eq!(
        seax_svm::svm::eval_program(list!(
            InstCell(LDC), AtomCell(SInt(5)),
            InstCell(LDC), AtomCell(SInt(5)),
            InstCell(ADD),
            InstCell(LDC), AtomCell(SInt(20)),
            InstCell(SUB)
        )).peek(),
        Some(&AtomCell(SInt(10)))
    );
}


/// Test for basic branching
///
/// ```lisp
/// ((if (= 0 (- 1 1)) true false)
/// ```
#[test]
fn test_basic_branching() {
    assert_eq!(
        seax_svm::svm::eval_program(list!(
            InstCell(LDC), AtomCell(SInt(1)), InstCell(LDC), AtomCell(SInt(1)),
            InstCell(SUB),
            InstCell(LDC), AtomCell(SInt(0)),
            InstCell(EQ),
            InstCell(SEL),
                ListCell(box list!(InstCell(LDC), AtomCell(SInt(1)), InstCell(JOIN))),
                ListCell(box list!(InstCell(NIL), InstCell(JOIN)))
        )).peek(),
        Some(&AtomCell(SInt(1)))
    );
}

