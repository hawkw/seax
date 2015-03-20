extern crate "parser-combinators" as parser_combinators;
use self::parser_combinators::{between, spaces, parser, many, many1, digit, optional, hex_digit, not_followed_by, satisfy, Parser, ParserExt, ParseResult};
use self::parser_combinators::primitives::{State, Stream};
use super::ast::*;
use super::ast::ExprNode::*;

pub fn number<I>(input: State<I>) -> ParseResult<NumNode, I>
    where I: Stream<Item=char> {
        let signed_int = optional(satisfy(|c| c == '-'))
            .and(many1::<Vec<_>, _>(digit())
                .or(satisfy(|c| c == '0')
                    .with(satisfy(|c| c == 'x' || c == 'X'))
                    .with(many1::<Vec<_>, _>(hex_digit()))
                    )
                )
            .map(|x| {
                let mut s = String::new();
                if let Some(sign) = x.0 { s.push(sign) };
                x.1.iter()
                    .fold(s, |mut s: String, i| { s.push(*i); s })
                    .parse::<isize>()
                    .unwrap()
                });

        signed_int
            .map(|x: isize| NumNode::IntConst(IntNode{value: x}))
            .parse_state(input)
}

pub fn name<I>(input: State<I>) -> ParseResult<NameNode, I>
    where I: Stream<Item=char> {
         let initial = satisfy(|c|
            c.is_alphabetic()
                // R6RS 'special initial' characters
                || c == '!' || c == '$' || c == '%' || c == ':' || c == '^'
                || c == '<' || c == '>' || c == '_' || c == '~' || c == '\\'
                || c == '?');
        let subsequent = satisfy(|c|
            c.is_alphanumeric()
                // R6RS 'special initial' characters
                || c == '!' || c == '$' || c == '%' || c == ':' || c == '^'
                || c == '<' || c == '>' || c == '_' || c == '~' || c == '\\'
                || c == '?'
                // R6RS 'special subsequent' characters
                || c == '+' || c == '-' || c == '.' || c == '@');
        initial
            .and(many::<Vec<_>, _>(subsequent).map(|it|
                it.iter().fold(
                    String::new(),
                    |mut s: String, i| {
                        s.push(*i);
                        s
                    }
                    )
                ))
            .parse_state(input)
            .map(|x| {
                let mut s = String::new();
                s.push((x.0).0);
                s.push_str(&(x.0).1);
                (NameNode{ name: s}, x.1)
            })
}

pub fn expr<I>(input: State<I>) -> ParseResult<ExprNode, I>
    where I: Stream<Item=char> {
        let spaces = spaces();
        let sexpr = between(
            satisfy(|c| c == '('),
            satisfy(|c| c == ')'),
            parser(name)
                .and(many(parser(expr)))
                .map(|x| {
                    SExpr(SExprNode {
                        operator: x.0,
                        operands: x.1
                    })
                })
            );
        spaces.clone().with(
            sexpr
                .or(parser(name).map(Name))
            ).parse_state(input)
}

#[cfg(test)]
mod tests {
    use ::ast::*;
    use ::ast::ExprNode::*;
    use super::{expr, number};
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
    fn test_parse_number() {
        assert_eq!(
            parser(number).parse("1234").unwrap(),
            (NumNode::IntConst(IntNode { value: 1234isize }), "")
            );
        assert_eq!(
            parser(number).parse("-1234").unwrap(),
            (NumNode::IntConst(IntNode { value: -1234isize }), "")
            );
        assert_eq!(
            parser(number).parse("1234u").unwrap(),
            (NumNode::UIntConst(UIntNode { value: 1234usize }), "")
            );
        assert_eq!(
            parser(number).parse("1.0").unwrap(),
            (NumNode::FloatConst(FloatNode { value: 1.0f64 }), "")
            );
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
            );
    }

}
