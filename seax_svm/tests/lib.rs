#![feature(box_syntax,box_patterns)]

#[macro_use]
extern crate seax_svm as svm;

use svm::slist::Stack;
use svm::slist::List::{Cons,Nil};
use svm::cell::Atom::*;
use svm::cell::SVMCell::*;
use svm::Inst::*;

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
        svm::eval_program(list!(
            InstCell(NIL),
            InstCell(LDC), AtomCell(SInt(20)), InstCell(CONS),
            InstCell(LDC), AtomCell(SInt(10)), InstCell(CONS)
        ), true).peek(),
        Some(&ListCell( box list!(AtomCell(SInt(10)), AtomCell(SInt(20)))))
    );
}

/// Test for simple list construction and destructuring
///
/// ```lisp
/// (car (cons 20 (cons 10 nil)))
/// ```
#[test]
fn test_list_car() {
    assert_eq!(
        svm::eval_program(list!(
            InstCell(NIL),
            InstCell(LDC), AtomCell(SInt(10)), InstCell(CONS),
            InstCell(LDC), AtomCell(SInt(20)), InstCell(CONS),
            InstCell(CAR)
        ), true).peek(),
        Some(&AtomCell(SInt(20)))
    );
}
/// Test for simple list construction and destructuring
///
/// ```lisp
/// (cdr (cons 20 (cons 10 nil)))
/// ```
#[test]
fn test_list_cdr() {
    assert_eq!(
        svm::eval_program(list!(
            InstCell(NIL),
            InstCell(LDC), AtomCell(SInt(10)), InstCell(CONS),
            InstCell(LDC), AtomCell(SInt(20)), InstCell(CONS),
            InstCell(CDR)
        ), true).peek(),
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
        svm::eval_program(list!(
            InstCell(LDC), AtomCell(SInt(10)),
            InstCell(LDC), AtomCell(SInt(10)),
            InstCell(ADD)
        ), true).peek(),
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
        svm::eval_program(list!(
            InstCell(LDC), AtomCell(SInt(5)),
            InstCell(LDC), AtomCell(SInt(5)),
            InstCell(ADD),
            InstCell(LDC), AtomCell(SInt(20)),
            InstCell(SUB)
        ), true).peek(),
        Some(&AtomCell(SInt(10)))
    );
}


/// Tests for basic branching
///
/// ```lisp
/// ((if (= 0 (- 1 1)) true false)
/// ```
///
/// ```lisp
/// (+ 10 (if (nil? nil) 10 20))
/// ```
#[test]
fn test_basic_branching() {
    assert_eq!(
        svm::eval_program(list!(
            InstCell(LDC), AtomCell(SInt(1)), InstCell(LDC), AtomCell(SInt(1)),
            InstCell(SUB),
            InstCell(LDC), AtomCell(SInt(0)),
            InstCell(EQ),
            InstCell(SEL),
                ListCell(box list!(InstCell(LDC), AtomCell(SInt(1)), InstCell(JOIN))),
                ListCell(box list!(InstCell(NIL), InstCell(JOIN))
            )
        ), true).peek(),
        Some(&AtomCell(SInt(1)))
    );
    assert_eq!(
        svm::eval_program(list!(
            InstCell(NIL), InstCell(NULL),
            InstCell(SEL),
                ListCell(box list!(InstCell(LDC), AtomCell(SInt(10)), InstCell(JOIN))),
                ListCell(box list!(InstCell(LDC), AtomCell(SInt(20)), InstCell(JOIN))),
            InstCell(LDC), AtomCell(SInt(10)),
            InstCell(ADD)
        ), true).peek(),
        Some(&AtomCell(SInt(20)))
    );
}

