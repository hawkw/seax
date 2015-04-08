#![feature(box_syntax,box_patterns)]
#![feature(compile)]
#![feature(scheme)]

/// Scheme compiler integration tests
///
/// These are based on the sample programs in Zach Allaun's Clojure SECD
/// [implementation](https://github.com/zachallaun/secd).

#[macro_use]
extern crate seax_svm as svm;
extern crate seax_scheme as scheme;

use svm::slist::List::{Cons,Nil};
use svm::cell::Atom::*;
use svm::cell::SVMCell::*;
use svm::Inst::*;


/// Test for simple list construction through CONS.
///
/// ```lisp
/// (cons 10 (cons 20 nil))
/// ```
#[test]
fn compile_list_creation() {
    assert_eq!(
        scheme::compile("(cons 10 (cons 20 nil))"),
        Ok(list!(
            InstCell(NIL),
            InstCell(LDC), AtomCell(SInt(10)), InstCell(CONS),
            InstCell(LDC), AtomCell(SInt(20)), InstCell(CONS)
        ))
    );
}

/// Test for simple list construction and destructuring
///
/// ```lisp
/// (car (cons 10 (cons 20 nil)))
/// ```
#[test]
fn  compile_list_car() {
    assert_eq!(
        scheme::compile("(car (cons 10 (cons 20 nil)))"),
        Ok(list!(
            InstCell(NIL),
            InstCell(LDC), AtomCell(SInt(10)), InstCell(CONS),
            InstCell(LDC), AtomCell(SInt(20)), InstCell(CONS),
            InstCell(CAR)
        ))
    );
}

/// Test for simple list construction and destructuring
///
/// ```lisp
/// (cdr (cons 10 (cons 20 nil)))
/// ```
#[test]
fn compile_list_cdr() {
    assert_eq!(
        scheme::compile("(cdr (cons 10 (cons 20 nil)))"),
        Ok(list!(
            InstCell(NIL),
            InstCell(LDC), AtomCell(SInt(10)), InstCell(CONS),
            InstCell(LDC), AtomCell(SInt(20)), InstCell(CONS),
            InstCell(CDR)
        ))
    );
}

/// Test for simple mathematics application
///
/// ```lisp
/// (+ 10 10)
/// ```
#[test]
fn compile_simple_add(){
    assert_eq!(
        scheme::compile("(+ 10 10)"),
        Ok(list!(
            InstCell(LDC), AtomCell(SInt(10)),
            InstCell(LDC), AtomCell(SInt(10)),
            InstCell(ADD)
        ))
    );
}

/// Test for nested arithmetic
///
/// ```lisp
/// (- 20 (+ 5 5))
/// ```
#[test]
fn compile_nested_arith() {
     assert_eq!(
        scheme::compile("(- 20 (+ 5 5))"),
        Ok(list!(
            InstCell(LDC), AtomCell(SInt(5)),
            InstCell(LDC), AtomCell(SInt(5)),
            InstCell(ADD),
            InstCell(LDC), AtomCell(SInt(20)),
            InstCell(SUB)
        ))
    );
}


/// Tests for basic branching
///
/// ```lisp
/// ((if (= 0 (- 1 1)) #t #f)
/// ```
///
/// ```lisp
/// (+ 10 (if (nil? nil) 10 20))
/// ```
#[test]
fn compile_basic_branching() {
    assert_eq!(
        scheme::compile("((if (= 0 (- 1 1)) #t #f)"),
        Ok(list!(
            InstCell(LDC), AtomCell(SInt(1)), InstCell(LDC), AtomCell(SInt(1)),
            InstCell(SUB),
            InstCell(LDC), AtomCell(SInt(0)),
            InstCell(EQ),
            InstCell(SEL),
                ListCell(box list!(InstCell(LDC), AtomCell(SInt(1)), InstCell(JOIN))),
                ListCell(box list!(InstCell(NIL), InstCell(JOIN))
            )
        ))
    );
    assert_eq!(
        scheme::compile("(+ 10 (if (nil? nil) 10 20))"),
        Ok(list!(
            InstCell(NIL), InstCell(NULL),
            InstCell(SEL),
                ListCell(box list!(InstCell(LDC), AtomCell(SInt(10)), InstCell(JOIN))),
                ListCell(box list!(InstCell(LDC), AtomCell(SInt(20)), InstCell(JOIN))),
            InstCell(LDC), AtomCell(SInt(10)),
            InstCell(ADD)
        ))
    );
}

