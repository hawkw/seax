use super::*;
use super::NumNode::*;
use super::ExprNode::*;
use super::SymTable::*;

use svm::cell::SVMCell;
use svm::cell::Atom::*;
use svm::cell::Inst::*;
use svm::cell::SVMCell::*;

#[test]
fn test_compile_add() {
    let ast = SExprNode {
        operator: NameNode { name: "+".to_string() },
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
        operator: NameNode { name: "-".to_string() },
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
        operator: NameNode { name: "/".to_string() },
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
        operator: NameNode { name: "*".to_string() },
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