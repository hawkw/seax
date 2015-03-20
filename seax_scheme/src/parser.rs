extern crate "parser-combinators" as parser_combinators;
use self::parser_combinators::{between, spaces, many, many1, sep_by, alpha_num, satisfy, string,Parser, ParserExt, ParseResult};
use self::parser_combinators::primitives::{State, Stream};
use super::ast;
use super::ast::ExprNode;

pub fn name<I>(input: State<I>) -> ParseResult<ast::NameNode, I>
    where I: Stream<Item=char> {
         let ident_start = satisfy(|c|
            c.is_alphabetic()
                || c == '='
                || c == '*'
                || c == '+'
                || c == '/'
                || c == '!'
                || c == '\\'
                || c == '?');
        let ident_body = satisfy(|c|
            c.is_alphanumeric()
                || c == '='
                || c == '*'
                || c == '+'
                || c == '/'
                || c == '!'
                || c == '\\'
                || c == '?');
        ident_start
            .and(many::<Vec<_>, _>(ident_body).map(|it|
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
                (ast::NameNode{ name: s}, x.1)
            })

}

pub fn expr<I>(input: State<I>) -> ParseResult<ast::ExprNode, I>
    where I: Stream<Item=char> {
        let mut sexpr = between(
            satisfy(|c| c == '('),
            satisfy(|c| c == ')'),
            (name as fn(_) -> _)
                .and(many(expr as fn(_) -> _))
                .map(|x| {
                    ast::ExprNode::SExpr(ast::SExprNode {
                        operator: x.0,
                        operands: (x.1)
                    })
                })
                );
        sexpr
            .or((name as fn(_) -> _).map(|x| ast::ExprNode::Name(x)))
            .parse_state(input)
}

#[cfg(test)]
mod tests {
    use ::ast;
    use super::expr;
    use super::parser_combinators::{Parser, ParseResult};

    #[test]
    fn test_basic_ident() {
        assert_eq!(
            (expr as fn (_) -> _).parse("ident").unwrap(),
            (ast::ExprNode::Name(ast::NameNode { name: "ident".to_string() }), "")
            );
    }

}
