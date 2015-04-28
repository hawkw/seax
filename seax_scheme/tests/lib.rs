#![feature(box_syntax,box_patterns)]
#![feature(compile)]
#![feature(scheme)]

/// Scheme compiler integration tests
///
/// These are based on the sample programs in Zach Allaun's Clojure SECD
/// [implementation](https://github.com/zachallaun/secd).
/// And from http://webdocs.cs.ualberta.ca/%7Eyou/courses/325/Mynotes/Fun/SECD-slides.html

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
            InstCell(LDC), AtomCell(SInt(20)), InstCell(CONS),
            InstCell(LDC), AtomCell(SInt(10)), InstCell(CONS)
        ))
    );
}

/// Test for simple list construction and destructuring
///
/// ```lisp
/// (car (cons 20 (cons 10 nil)))
/// ```
#[test]
fn  compile_list_car() {
    assert_eq!(
        scheme::compile("(car (cons 20 (cons 10 nil)))"),
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
/// (cdr (cons 20 (cons 10 nil)))
/// ```
#[test]
fn compile_list_cdr() {
    assert_eq!(
        scheme::compile("(cdr (cons 20 (cons 10 nil)))"),
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
/// (if (= 0 (- 1 1)) #t #f)
/// ```
#[test]
fn compile_basic_branching_1() {
    assert_eq!(
        scheme::compile("(if (= 0 (- 1 1)) #t #f)"),
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
}
///
/// ```lisp
/// (+ 10 (if (nil? nil) 10 20))
/// ```
#[test]
fn compile_basic_branching_2() {
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

/// Lambda
///
/// ```lisp
/// (lambda (x y) (+ x y))
/// ```
///
/// (LDF (LD (1.2) LD (1.1) + RTN))
#[test]
fn compile_lambda() {
    assert_eq!(
        scheme::compile("(lambda (x y) (+ x y))"),
        Ok(list!(
            InstCell(LDF),
            ListCell(box list!(
                InstCell(LD),
                ListCell(box list!(
                    AtomCell(UInt(1)), AtomCell(UInt(2))
                    )),
                InstCell(LD),
                ListCell(box list!(
                    AtomCell(UInt(1)), AtomCell(UInt(1))
                    )),
                InstCell(ADD),
                InstCell(RET)
            ))
        ))
    )
}

/// Lambda application
///
/// ```lisp
/// ((lambda (x y) (+ x y)) 2 3)
/// ```
///
/// (NIL LDC 3 CONS LDC 2 CONS LDF (LD (1.2) LD (1.1) + RTN) AP)
#[test]
fn compile_lambda_ap() {
    assert_eq!(
        scheme::compile("((lambda (x y) (+ x y)) 2 3)"),
        Ok(list!(
            InstCell(NIL),
            InstCell(LDC), AtomCell(SInt(3)),
            InstCell(CONS),
            InstCell(LDC), AtomCell(SInt(2)),
            InstCell(CONS),
            InstCell(LDF),
            ListCell(box list!(
                InstCell(LD), ListCell(box list!(
                    AtomCell(UInt(1)), AtomCell(UInt(2))
                    )),
                InstCell(LD), ListCell(box list!(
                    AtomCell(UInt(1)), AtomCell(UInt(1))
                    )),
                InstCell(ADD),
                InstCell(RET)
            )),
            InstCell(AP)
        ))
    )
}

/// Nested lambdas
///
/// ```lisp
/// ((lambda (z) ((lambda (x y) (+ (- x y) z)) 3 5)) 6)
/// ```
///
/// ```seax
/// (NIL LDC 6 CONS LDF
///     (NIL LDC 5 CONS LDC 3 CONS
///         LDF
///             (LD (2.1) LD (1.2) LD (1.1) SUB ADD RTN)
///         AP
///         RTN)
///     AP
/// ```
#[test]
fn compile_nested_lambda() {
    assert_eq!(
        scheme::compile("((lambda (z) ((lambda (x y) (+ (- x y) z)) 3 5)) 6)"),
        Ok(list!(
            InstCell(NIL),
            InstCell(LDC), AtomCell(SInt(6)), InstCell(CONS),
            InstCell(LDF),
            ListCell(box list!(
                InstCell(NIL),
                InstCell(LDC), AtomCell(SInt(5)), InstCell(CONS),
                InstCell(LDC), AtomCell(SInt(3)), InstCell(CONS),
                InstCell(LDF),
                ListCell(box list!(
                    InstCell(LD), ListCell(box list!(
                        AtomCell(UInt(2)),AtomCell(UInt(1))
                        )),
                    InstCell(LD), ListCell(box list!(
                        AtomCell(UInt(1)),AtomCell(UInt(2))
                        )),
                    InstCell(LD), ListCell(box list!(
                        AtomCell(UInt(1)),AtomCell(UInt(1))
                        )),
                    InstCell(SUB),
                    InstCell(ADD),
                    InstCell(RET)
                )),
                InstCell(AP),
                InstCell(RET)
            )),
            InstCell(AP)
        ))
    )
}

/// Test for the compilation of a single simple `let` binding.
///
/// ```lisp
/// (let ([x 5]) x)
/// ```
#[test]
fn compile_single_let() {
    assert_eq!(
        scheme::compile("(let ([x 5]) x)"),
        Ok(list!(
            InstCell(NIL),
            InstCell(LDC), AtomCell(SInt(5)), InstCell(CONS),
            InstCell(LDF),
            ListCell(box list!(
                InstCell(LD), ListCell(box list!(
                        AtomCell(UInt(1)),AtomCell(UInt(1))
                    )),
                InstCell(RET)
            )),
            InstCell(AP)
        ))
    );
}


/// Test for the compilation of multiple simple `let` bindings.
///
/// ```lisp
/// (let ([x 1]
///       [y 2]
///       [z 3])
///      (+ x y z))
/// ```
#[test]
fn compile_multiple_let() {
    assert_eq!(
        scheme::compile(
            "(let ([x 1]
                   [y 2]
                   [z 3])
                (+ x y z))"),
        Ok(list!(
            InstCell(NIL),
            InstCell(LDC), AtomCell(SInt(1)), InstCell(CONS),
            InstCell(LDC), AtomCell(SInt(2)), InstCell(CONS),
            InstCell(LDC), AtomCell(SInt(3)), InstCell(CONS),
            InstCell(LDF),
            ListCell(box list!(
                InstCell(LD), ListCell(box list!(
                        AtomCell(UInt(1)),AtomCell(UInt(3))
                    )),
                InstCell(LD), ListCell(box list!(
                        AtomCell(UInt(1)),AtomCell(UInt(2))
                    )),
                InstCell(ADD),
                InstCell(LD), ListCell(box list!(
                        AtomCell(UInt(1)),AtomCell(UInt(1))
                    )),
                InstCell(ADD),
                InstCell(RET)
            )),
            InstCell(AP)
        ))
    );
}

/// Test for the compilation of a `let` binding to the result of
/// an expression.
///
/// ```lisp
/// (let ([x (+ 1 1)]) x
/// ```
#[test]
fn compile_expr_let() {
    assert_eq!(
        scheme::compile("(let ([x (+ 1 1)]) x)"),
        Ok(list!(
            InstCell(NIL),
            InstCell(LDC), AtomCell(SInt(1)),
            InstCell(LDC), AtomCell(SInt(1)),
            InstCell(ADD),
            InstCell(CONS),
            InstCell(LDF),
            ListCell(box list!(
                InstCell(LD), ListCell(box list!(
                        AtomCell(UInt(1)),AtomCell(UInt(1))
                    )),
                InstCell(RET)
            )),
            InstCell(AP)
        ))
    );
}
/*
/// Test for the compilation of a `let` binding with name shadowing
///
/// ```lisp
/// (let ([x 1])
///     (let ([x 2]) x)
///     )
/// ```
#[test]
fn compile_name_shadowing_let() {
    assert_eq!(
        scheme::compile("(let ([x 1])
                            (let ([x 2]) x) )"),
        Ok(list!(
            InstCell(NIL),
            InstCell(LDC), AtomCell(SInt(1)),
            InstCell(CONS),
            InstCell(LDF),
            ListCell(box list!(
                InstCell(NIL),
                InstCell(LDC), AtomCell(SInt(2)),
                InstCell(CONS),
                InstCell(LDF),
                ListCell(box list!(
                    InstCell(LD), ListCell(box list!(
                            AtomCell(UInt(1)),AtomCell(UInt(1))
                        )),
                InstCell(RET)
                )),
            InstCell(AP),
            InstCell(RET)
            )),
            InstCell(AP)
        ))
    );
}*/


