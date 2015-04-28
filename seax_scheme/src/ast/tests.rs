use super::*;
use super::NumNode::*;
use super::ExprNode::*;

use svm::cell::Atom::*;
use svm::cell::Inst::*;
use svm::cell::SVMCell::*;

use svm::slist::List::{Cons,Nil};

#[test]
fn test_compile_add() {
    let ast = SExprNode {
        operator: box Name(NameNode { name: "+".to_string() }),
        operands: vec![
            NumConst(IntConst(IntNode{ value: 1isize })),
            NumConst(IntConst(IntNode{ value: 2isize }))
        ]
    };
    assert_eq!(
        ast.compile(&SymTable::new()),
        Ok(vec![
            InstCell(LDC), AtomCell(SInt(2)),
            InstCell(LDC), AtomCell(SInt(1)),
            InstCell(ADD)
        ])
    )
}

#[test]
fn test_compile_sub() {
    let ast = SExprNode {
        operator: box Name(NameNode { name: "-".to_string() }),
        operands: vec![
            NumConst(UIntConst(UIntNode{ value: 9usize })),
            NumConst(UIntConst(UIntNode{ value: 9usize })),
            NumConst(UIntConst(UIntNode{ value: 9usize }))
        ]
    };
    assert_eq!(
        ast.compile(&SymTable::new()),
        Ok(vec![
            InstCell(LDC), AtomCell(UInt(9)),
            InstCell(LDC), AtomCell(UInt(9)),
            InstCell(SUB),
            InstCell(LDC), AtomCell(UInt(9)),
            InstCell(SUB)
        ])
    )
}

#[test]
fn test_compile_div() {
    let ast = SExprNode {
        operator: box Name(NameNode { name: "/".to_string() }),
        operands: vec![
            NumConst(IntConst(IntNode{ value: 1isize })),
            NumConst(IntConst(IntNode{ value: 2isize })),
            NumConst(IntConst(IntNode{ value: 3isize })),
            NumConst(IntConst(IntNode{ value: 4isize }))
        ]
    };
    assert_eq!(
        ast.compile(&SymTable::new()),
        Ok(vec![
            InstCell(LDC), AtomCell(SInt(4isize)),
            InstCell(LDC), AtomCell(SInt(3isize)),
            InstCell(DIV),
            InstCell(LDC), AtomCell(SInt(2isize)),
            InstCell(DIV),
            InstCell(LDC), AtomCell(SInt(1isize)),
            InstCell(DIV)

        ])
    )
}

#[test]
fn test_compile_mul() {
    let ast = SExprNode {
        operator: box Name(NameNode { name: "*".to_string() }),
        operands: vec![
            NumConst(FloatConst(FloatNode{ value: 1f64 })),
            NumConst(FloatConst(FloatNode{ value: 2f64 })),
            NumConst(FloatConst(FloatNode{ value: 3f64 })),
            NumConst(FloatConst(FloatNode{ value: 4f64 }))
        ]
    };
    assert_eq!(
        ast.compile(&SymTable::new()),
        Ok(vec![
            InstCell(LDC), AtomCell(Float(4f64)),
            InstCell(LDC), AtomCell(Float(3f64)),
            InstCell(MUL),
            InstCell(LDC), AtomCell(Float(2f64)),
            InstCell(MUL),
            InstCell(LDC), AtomCell(Float(1f64)),
            InstCell(MUL)
        ])
    )
}

#[test]
fn test_compile_nested_sexpr() {
    let ast = SExprNode {
        operator: box Name(NameNode { name: "+".to_string() }),
        operands: vec![
            NumConst(IntConst(IntNode{ value: 4isize })),
            SExpr(SExprNode {
                operator: box Name(NameNode { name: "-".to_string() }),
                operands: vec![
                    NumConst(IntConst(IntNode{ value: 1isize })),
                    NumConst(IntConst(IntNode{ value: 2isize })),
                    NumConst(IntConst(IntNode{ value: 3isize }))
                ]
            })
        ]
    };
    assert_eq!(
        ast.compile(&SymTable::new()),
        Ok(vec![
            InstCell(LDC), AtomCell(SInt(3)),
            InstCell(LDC), AtomCell(SInt(2)),
            InstCell(SUB),
            InstCell(LDC), AtomCell(SInt(1)),
            InstCell(SUB),
            InstCell(LDC), AtomCell(SInt(4)),
            InstCell(ADD)
        ])
    )
}

#[test]
fn test_compile_gte() {
    let ast = SExprNode {
        operator: box Name(NameNode { name: ">=".to_string() }),
        operands: vec![
            NumConst(FloatConst(FloatNode{ value: 1f64 })),
            NumConst(IntConst(IntNode{ value: 2isize })),
        ]
    };
    assert_eq!(
        ast.compile(&SymTable::new()),
        Ok(vec![
            InstCell(LDC), AtomCell(SInt(2isize)),
            InstCell(LDC), AtomCell(Float(1f64)),
            InstCell(GTE)
        ])
    )
}

#[test]
fn test_compile_lte() {
    let ast = SExprNode {
        operator: box Name(NameNode { name: "<=".to_string() }),
        operands: vec![
            NumConst(UIntConst(UIntNode{ value: 3usize })),
            NumConst(IntConst(IntNode{ value: 2isize })),
        ]
    };
    assert_eq!(
        ast.compile(&SymTable::new()),
        Ok(vec![
            InstCell(LDC), AtomCell(SInt(2isize)),
            InstCell(LDC), AtomCell(UInt(3usize)),
            InstCell(LTE)
        ])
    )
}

#[test]
fn test_compile_string() {
    assert_eq!(
        StringNode{ value: "a string".to_string() }
            .compile(&SymTable::new()),
        Ok(vec![ListCell(box list!(
            AtomCell(Char('a')),
            AtomCell(Char(' ')),
            AtomCell(Char('s')),
            AtomCell(Char('t')),
            AtomCell(Char('r')),
            AtomCell(Char('i')),
            AtomCell(Char('n')),
            AtomCell(Char('g'))
            ))])
        )
}

