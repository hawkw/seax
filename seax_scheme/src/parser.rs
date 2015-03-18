extern crate "parser-combinators" as parser_combinators;
use self::parser_combinators::{between, spaces, many, many1, sep_by, satisfy, string,Parser, ParserExt, ParseResult};
use self::parser_combinators::primitives::{State, Stream};
use super::ast;
use super::ast::ExprNode;


pub fn expr<I>(input: State<I>) -> ParseResult<ast::ExprNode, I>
    where I: Stream<Item=char> {

        let spaces = spaces();
        let name = many1(satisfy(|c| c.is_alphabetic()));
        /* // TODO: fix
        let sexpr = between(
            satisfy(|c| c == '('),
            satisfy(|c| c == ')'),
            expr as fn (_) -> _
            );*/
        spaces.clone().with(
             name.map(|it: String| ast::ExprNode::Name(ast::NameNode{ name: it}))
             //.or(sexpr.map(|it| ast::ExprNode::SExpr(ast::SExprNode())))
         ).parse_state(input)
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
