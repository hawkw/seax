extern crate "parser-combinators" as parser_combinators;
use self::parser_combinators::{between, spaces, many, satisfy, Parser, ParserExt, ParseResult};
use self::parser_combinators::primitives::{State, Stream};
use super::ast::*;
use super::ast::ExprNode::*;

pub fn name<I>(input: State<I>) -> ParseResult<NameNode, I>
    where I: Stream<Item=char> {
         let initial = satisfy(|c|
            c.is_alphabetic()
                || c == '='
                || c == '*'
                || c == '+'
                || c == '/'
                || c == '!'
                || c == '\\'
                || c == '?');
        let subsequent = satisfy(|c|
            c.is_alphanumeric()
                || c == '='
                || c == '*'
                || c == '+'
                || c == '/'
                || c == '!'
                || c == '\\'
                || c == '?');
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
            (name as fn(_) -> _)
                .and(many(expr as fn(_) -> _))
                .map(|x| {
                    SExpr(SExprNode {
                        operator: x.0,
                        operands: (x.1)
                    })
                })
                );
        spaces.clone().with(
            sexpr
                .or((name as fn(_) -> _).map(Name))
            ).parse_state(input)
}

#[cfg(test)]
mod tests {
    use ::ast::*;
    use ::ast::ExprNode::*;
    use super::expr;
    use super::parser_combinators::Parser;

    #[test]
    fn test_basic_ident() {
        assert_eq!(
            (expr as fn (_) -> _).parse("ident").unwrap(),
            (Name(NameNode { name: "ident".to_string() }), "")
            );
    }

        #[test]
    fn test_basic_sexpr() {
        assert_eq!(
            (expr as fn (_) -> _).parse("(ident arg1 arg2)").unwrap(),
            (SExpr(SExprNode {
                operator: NameNode { name: "ident".to_string() },
                operands: vec![
                    Name(NameNode { name: "arg1".to_string() }),
                    Name(NameNode { name: "arg2".to_string() })
                ]
            }), "")
            );
    }

}
