extern crate "parser-combinators" as parser_combinators;
use self::parser_combinators::{between, spaces, many1, sep_by, satisfy, Parser, ParserExt, ParseResult};
use self::parser_combinators::primitives::{State, Stream};
use ast;


fn expr<I>(input: State<I>) -> ParseResult(ast::ExprNode, I)
    where I: Stream<Item=char> {
        unimplemented!()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_basic_ident() {
        assert_eq!(
            (expr as fn (_) -> _).parse("ident"),
            Ok((ast::NameNode { name: "ident" }, ""))
            );
    }

}