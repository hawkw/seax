use ::ast::*;
use ::ast::ExprNode::*;
use super::*;
use super::parser_combinators::{Parser,parser};
use std::char;

#[test]
fn test_basic_ident() {
    assert_eq!(
        parser(expr).parse("ident"),
        Ok((Name(NameNode { name: "ident".to_string() }), ""))
        );
    assert_eq!(
        parser(expr).parse("a"),
        Ok((Name(NameNode { name: "a".to_string() }), ""))
        );
    assert_eq!(
        parser(expr).parse("ident_With\\special!Chars:~-+"),
        Ok((Name(NameNode { name: "ident_With\\special!Chars:~-+".to_string() }), ""))
        );
}

#[test]
fn test_basic_sexpr() {
    assert_eq!(
        parser(expr).parse("(ident arg1 arg2)"),
        Ok((SExpr(SExprNode {
            operator: NameNode { name: "ident".to_string() },
            operands: vec![
                Name(NameNode { name: "arg1".to_string() }),
                Name(NameNode { name: "arg2".to_string() })
            ]
        }), ""))
        );
}

#[test]
fn test_lex_sint_pos() {
    assert_eq!(
        parser(number).parse("1234"),
        Ok((NumNode::IntConst(IntNode { value: 1234isize }), ""))
        );
    assert_eq!(
        parser(number).parse("#d1234"),
        Ok((NumNode::IntConst(IntNode { value: 1234isize }), ""))
        );
    assert_eq!(
        parser(number).parse("#D1234"),
        Ok((NumNode::IntConst(IntNode { value: 1234isize }), ""))
        );
}

#[test]
fn test_lex_sint_neg() {
    assert_eq!(
        parser(number).parse("-1234"),
        Ok((NumNode::IntConst(IntNode { value: -1234isize }), ""))
        );
}

#[test]
fn test_lex_sint_hex() {
    assert_eq!(
        parser(number).parse("#x0ff"),
        Ok((NumNode::IntConst(IntNode { value: 0x0ffisize }), ""))
        );
    assert_eq!(
        parser(number).parse("#X0FF"),
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
fn test_lex_uint() {
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
fn test_lex_uint_hex() {
    assert_eq!(
        parser(number).parse("#x0ffu"),
        Ok((NumNode::UIntConst(UIntNode { value: 0x0ffusize }), ""))
        );
    assert_eq!(
        parser(number).parse("#X0FFu"),
        Ok((NumNode::UIntConst(UIntNode { value: 0x0ffusize }), ""))
        );
}

#[test]
fn test_lex_float() {
    assert_eq!(
        parser(number).parse("1.0"),
        Ok((NumNode::FloatConst(FloatNode { value: 1.0f64 }), ""))
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

#[test]
fn test_lex_bool() {
    assert_eq!(
        parser(bool_const).parse("#t"),
        Ok((BoolNode { value: true}, ""))
        );
    assert_eq!(
        parser(bool_const).parse("#T"),
        Ok((BoolNode { value: true}, ""))
        );
    assert_eq!(
        parser(bool_const).parse("true"),
        Ok((BoolNode { value: true}, ""))
        );
    assert_eq!(
        parser(bool_const).parse("#f"),
        Ok((BoolNode { value: false}, ""))
        );
    assert_eq!(
        parser(bool_const).parse("#F"),
        Ok((BoolNode { value: false}, ""))
        );
    assert_eq!(
        parser(bool_const).parse("false"),
        Ok((BoolNode { value: false}, ""))
        );
}

#[test]
fn test_lex_char() {
    assert_eq!(
        parser(character).parse("#\\c"),
        Ok((CharNode { value: 'c'}, ""))
        );
    assert_eq!(
        parser(character).parse("#\\A"),
        Ok((CharNode { value: 'A'}, ""))
        );
    assert_eq!(
        parser(character).parse("#\\tab"),
        Ok((CharNode { value: '\t'}, ""))
        );
    assert_eq!(
        parser(character).parse("#\\newline"),
        Ok((CharNode { value: '\n'}, ""))
        );
    assert_eq!(
        parser(character).parse("#\\nul"),
        Ok((CharNode { value: char::from_u32(0x0000).unwrap()}, ""))
        );
    assert_eq!(
        parser(character).parse("#\\backspace"),
        Ok((CharNode { value: char::from_u32(0x0008).unwrap()}, ""))
        );
    assert_eq!(
        parser(character).parse("#\\vtab"),
        Ok((CharNode { value: char::from_u32(0x000B).unwrap()}, ""))
        );
    assert_eq!(
        parser(character).parse("#\\page"),
        Ok((CharNode { value: char::from_u32(0x000C).unwrap()}, ""))
        );
    assert_eq!(
        parser(character).parse("#\\return"),
        Ok((CharNode { value: char::from_u32(0x000D).unwrap()}, ""))
        );
    assert_eq!(
        parser(character).parse("#\\esc"),
        Ok((CharNode { value: char::from_u32(0x001B).unwrap()}, ""))
        );
    assert_eq!(
        parser(character).parse("#\\delete"),
        Ok((CharNode { value: char::from_u32(0x007F).unwrap()}, ""))
        );
    assert_eq!(
        parser(character).parse("#\\alarm"),
        Ok((CharNode { value: char::from_u32(0x0007).unwrap()}, ""))
        );
    assert_eq!(
        parser(character).parse("#\\linefeed"),
        Ok((CharNode { value: '\n'}, ""))
        );
    assert_eq!(
        parser(character).parse("#\\space"),
        Ok((CharNode { value: ' '}, ""))
        );
    assert_eq!(
        parser(character).parse("#\\x0020"),
        Ok((CharNode { value: ' '}, ""))
        );
    assert_eq!(
        parser(character).parse("#\\x001B"),
        Ok((CharNode { value: char::from_u32(0x001B).unwrap()}, ""))
        );
}

#[test]
fn test_lex_string() {
    assert_eq!(
        parser(string_const).parse("\"a string\""),
        Ok((StringNode { value: "a string".to_string() }, ""))
    );
    assert_eq!(
        parser(string_const).parse("\"a string with a\\ttab\""),
        Ok((StringNode { value: "a string with a\ttab".to_string() },""))
    );
    assert_eq!(
        parser(string_const).parse("\"a string with an \\\"escaped\\\" quote\""),
        Ok((StringNode { value: "a string with an \"escaped\" quote".to_string() },""))
    );
    assert_eq!(
        parser(string_const).parse("\"the\\\\worst string ever\\\"\""),
        Ok((StringNode { value: "the\\worst string ever\"".to_string() }, ""))
    );
}
