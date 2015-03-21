use ::ast::*;
use ::ast::ExprNode::*;
use super::*;
use super::parser_combinators::{Parser,parser};

#[test]
fn test_basic_ident() {
    assert_eq!(
        parser(expr).parse("ident").unwrap(),
        (Name(NameNode { name: "ident".to_string() }), "")
        );
}

#[test]
fn test_basic_sexpr() {
    assert_eq!(
        parser(expr).parse("(ident arg1 arg2)").unwrap(),
        (SExpr(SExprNode {
            operator: NameNode { name: "ident".to_string() },
            operands: vec![
                Name(NameNode { name: "arg1".to_string() }),
                Name(NameNode { name: "arg2".to_string() })
            ]
        }), "")
        );
}

#[test]
fn test_parse_sint_pos() {
    assert_eq!(
        parser(number).parse("1234"),
        Ok((NumNode::IntConst(IntNode { value: 1234isize }), ""))
        );
}

#[test]
fn test_parse_sint_neg() {
    assert_eq!(
        parser(number).parse("-1234"),
        Ok((NumNode::IntConst(IntNode { value: -1234isize }), ""))
        );
}

#[test]
fn test_parse_sint_hex() {
    assert_eq!(
        parser(number).parse("0x0ff"),
        Ok((NumNode::IntConst(IntNode { value: 0x0ffisize }), ""))
        );
    assert_eq!(
        parser(number).parse("0X0FF"),
        Ok((NumNode::IntConst(IntNode { value: 0x0ffisize }), ""))
        );
}
/* // Currently unsupported
#[test]
fn test_parse_sint_bin_upper() {
    assert_eq!(
        parser(number).parse("0B01"),
        Ok((NumNode::IntConst(IntNode { value: 0b01isize }), ""))
        );
    assert_eq!(
        parser(number).parse("0b01"),
        Ok((NumNode::IntConst(IntNode { value: 0b01isize }), ""))
        );
}*/

#[test]
fn test_parse_uint() {
    assert_eq!(
        parser(number).parse("1234u"),
        Ok((NumNode::UIntConst(UIntNode { value: 1234usize }), ""))
        );
    assert_eq!(
        parser(number).parse("4321U"),
        Ok((NumNode::UIntConst(UIntNode { value: 4321usize }), ""))
        );
}

#[test]
fn test_parse_uint_hex() {
    assert_eq!(
        parser(number).parse("0x0ffu"),
        Ok((NumNode::UIntConst(UIntNode { value: 0x0ffusize }), ""))
        );
    assert_eq!(
        parser(number).parse("0X0FFu"),
        Ok((NumNode::UIntConst(UIntNode { value: 0x0ffusize }), ""))
        );
}

#[test]
fn test_parse_float() {
    assert_eq!(
        parser(number).parse("1.0").unwrap(),
        (NumNode::FloatConst(FloatNode { value: 1.0f64 }), "")
        );/* // Unsupported
    assert_eq!(
        parser(number).parse("1f").unwrap(),
        (NumNode::FloatConst(FloatNode { value: 1.0f64 }), "")
        );
    assert_eq!(
        parser(number).parse("22.2222").unwrap(),
        (NumNode::FloatConst(FloatNode { value: 22.2222f64 }), "")
        );
    assert_eq!(
        parser(number).parse("22.2222f").unwrap(),
        (NumNode::FloatConst(FloatNode { value: 22.2222f64 }), "")
        );*/
}
