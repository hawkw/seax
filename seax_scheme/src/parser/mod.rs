extern crate "parser-combinators" as parser_combinators;

use self::parser_combinators::{try, between, spaces, string, parser, many, many1, digit, any_char, optional, hex_digit, not_followed_by, satisfy, Parser, ParserExt, ParseResult};
use self::parser_combinators::primitives::{State, Stream};

use super::ast::*;
use super::ast::ExprNode::*;

use std::str::FromStr;
use std::num::FromStrRadix;
use std::char;

/// Parser for signed integer constants.
///
/// This parses signed integer constants in decimal and hexadecimal.
///
/// TODO: add support for octal
/// TODO: add support for binary
/// TODO: add support for R6RS exponents
fn sint_const<I>(input: State<I>) -> ParseResult<NumNode, I>
    where I: Stream<Item=char> {

        fn hex_string<I>(input: State<I>) -> ParseResult<isize, I>
            where I: Stream<Item=char> {
                (satisfy(|c| c == '#')
                    .and(satisfy(|c| c == 'x' || c == 'X')))
                    .with(many1::<Vec<_>, _>(hex_digit()))
                    .map(|x| {
                        isize::from_str_radix(
                            x.iter()
                             .fold(
                                String::new(),
                                |mut s: String, i| { s.push(*i); s })
                             .as_slice(),
                        16).unwrap()
                    }).parse_state(input)
        }


        fn dec_string<I>(input: State<I>) -> ParseResult<isize, I>
            where I: Stream<Item=char> {
                optional(satisfy(|c| c == '#')
                    .and(satisfy(|c| c == 'd' || c == 'D')))
                .with(many1::<Vec<_>, _>(digit())
                    .map(|x| isize::from_str(x.iter().fold(
                        String::new(), |mut s: String, i| { s.push(*i); s })
                        .as_slice()
                    ).unwrap()
                    )).parse_state(input)
            }

        optional(satisfy(|c| c == '-'))
            .and(
                try(parser(hex_string))
                .or(parser(dec_string))
                )
            .map(|x| {
                if let Some(sign) = x.0 {
                    let mut s = String::new();
                    s.push(sign);
                    s.push('1');
                    x.1 * isize::from_str(s.as_slice()).unwrap()
                } else {
                    x.1
                }
                })
            .skip(not_followed_by(satisfy(|c|
                c == 'u' || c == 'U' || c == '.' || c == 'f' || c == 'F')
            ))
            .map(|x: isize| NumNode::IntConst(IntNode{value: x}))
            .parse_state(input)
}
/// Parser for unsigned integer constants.
///
/// This parses unssigned integer constants in decimal and hexadecimal.
///
/// TODO: add support for octal
/// TODO: add support for binary
/// TODO: add support for R6RS exponents
fn uint_const<I>(input: State<I>) -> ParseResult<NumNode, I>
    where I: Stream<Item=char> {


        fn hex_string<I>(input: State<I>) -> ParseResult<usize, I>
            where I: Stream<Item=char> {
            (satisfy(|c| c == '#')
                .and(satisfy(|c| c == 'x' || c == 'X')))
                .with(many1::<Vec<_>, _>(hex_digit()))
                .map(|x| usize::from_str_radix(
                        x.iter()
                         .fold(
                            String::new(),
                            |mut s: String, i| { s.push(*i); s })
                         .as_slice(),
                    16).unwrap()
                ).parse_state(input)
        }

        try(parser(hex_string))
        .or( many1::<Vec<_>, _>(digit())
            .map(|x|usize::from_str(x.iter().fold(
                String::new(), |mut s: String, i| { s.push(*i); s })
                .as_slice()
            ).unwrap())
        )
        .skip(satisfy(|c| c == 'u' || c == 'U'))
        .map(|x: usize| NumNode::UIntConst(UIntNode{value: x}))
        .parse_state(input)
}

/// Parser for floating-point constants.
///
/// This parses floating-point constants. Currently, this parser
/// recognizes numbers with decimal points as floating point, followed
/// by an optional `f` or `F`. Numbers with `f`s but no decimal points,
/// i.e. `1F`, are currently not recognized. While this form of number
/// is not specified by R6RS, I'd like to support it anyway as it's
/// a common form for floating-point numbers. Priority: low.
fn float_const<I>(input: State<I>) -> ParseResult<NumNode, I>
    where I: Stream<Item=char> {
        many1::<Vec<_>, _>(digit())
            .and(satisfy(|c| c == '.'))
            .and(many1::<Vec<_>, _>(digit()))
            .map(|x| {
                let mut s = String::new();
                for i in (x.0).0.iter() { s.push(*i); } ;
                s.push((x.0).1);

                for i in x.1.iter() { s.push(*i); };
                NumNode::FloatConst(FloatNode{
                    value: f64::from_str(s.as_slice()).unwrap()
                })
            })
            .skip(optional(satisfy(|c| c == 'f' || c == 'F')))
            .parse_state(input)
}

/// Parses boolean constants.
///
/// Note that this parser recognizes the strings `"true"` and `"false"`
/// as true and false. While this is not specified in R6RS, the use of
/// these tokens is common enough in other programming languages that
/// I've decided that Seax Scheme should support it as well. This may
/// be removed in a future version if it causes unforseen compatibility
/// issues.
///
/// `#t`, `#T`, or `true`  -> `true`
/// `#f`, `#F`, or `false` -> `false`
pub fn bool_const<I>(input: State<I>) -> ParseResult<BoolNode, I>
    where I: Stream<Item=char> {
        let t_const = try(string("#t"))
            .or(try(string("#T")))
            .or(try(string("true")))
            .map(|_| BoolNode{ value: true });
        let f_const = try(string("#f"))
            .or(try(string("#F")))
            .or(try(string("false")))
            .map(|_| BoolNode{ value: false });
        t_const
            .or(f_const)
            .parse_state(input)
    }

/// Parses a floating-point, signed integer, or unsigned integer constant.
pub fn number<I>(input: State<I>) -> ParseResult<NumNode, I>
    where I: Stream<Item=char> {
        try(parser(sint_const))
            .or(try(parser(uint_const)))
            .or(try(parser(float_const)))
            .parse_state(input)
}

/// Parser for valid R6RS identifiers.
///
/// An identifier may begin with an alphabetic character or
/// one of the following special characters `!`, `$`, `&`, `:`, `^`,
/// `<`, `>`, `_`,`~`,`\`, or `?`. Subsequent characters may also include
/// numbers or the special characters `+`, `-`, `.`, and `@`.
///
/// Essentially, this parser recognizes the regular expression
/// `[a-zA-Z!\$%:\^<>_~\\\?][a-zA-Z0-9!\$%:\^<>_~\\\?\+\-\.@]*`.
///
/// For more information, consult the
/// [R6RS](http://www.r6rs.org/final/html/r6rs/r6rs-Z-H-7.html).
pub fn name<I>(input: State<I>) -> ParseResult<NameNode, I>
    where I: Stream<Item=char> {

        fn initial<I>(input: State<I>) -> ParseResult<char, I>
            where I: Stream<Item=char> {
                satisfy(|c|
                    c.is_alphabetic()
                    // R6RS 'special initial' characters
                    || c == '!' || c == '$' || c == '%' || c == ':' || c == '^'
                    || c == '<' || c == '>' || c == '_' || c == '~' || c == '\\'
                    || c == '?'
                ).parse_state(input)
            }

        fn subsequent<I>(input: State<I>) -> ParseResult<char, I>
            where I: Stream<Item=char> {

            satisfy(|c|
                c.is_alphanumeric()
                    // R6RS 'special initial' characters
                    || c == '!' || c == '$' || c == '%' || c == ':' || c == '^'
                    || c == '<' || c == '>' || c == '_' || c == '~' || c == '\\'
                    || c == '?'
                    // R6RS 'special subsequent' characters
                    || c == '+' || c == '-' || c == '.' || c == '@'
                ).parse_state(input)
            }

        fn rest<I>(input: State<I>) -> ParseResult<String, I>
            where I: Stream<Item=char> {
                many::<Vec<_>, _>(parser(subsequent))
                    .map(|it|
                        it.iter().fold(
                            String::new(),
                            |mut s: String, i| {
                                s.push(*i);
                                s
                            })
                        )
                    .parse_state(input)
            }

        parser(initial)
            .and(parser(rest))
            .parse_state(input)
            .map(|x| {
                let mut s = String::new();
                s.push((x.0).0);
                s.push_str(&(x.0).1);
                (NameNode{ name: s}, x.1)
            })
}

pub fn character<I> (input: State<I>) -> ParseResult<CharNode, I>
    where I: Stream<Item=char> {

        fn newline<I> (input: State<I>) -> ParseResult<char, I>
            where I: Stream<Item=char> {
                try(string("newline"))
                    .or(try(string("linefeed")))
                    .map(|_| '\n')
                    .parse_state(input)
            }

        fn tab<I> (input: State<I>) -> ParseResult<char, I>
            where I: Stream<Item=char> {
                try(string("tab")).map(|_| '\t').parse_state(input)
            }

        fn nul<I> (input: State<I>) -> ParseResult<char, I>
            where I: Stream<Item=char> {
                try(string("nul"))
                    .map(|_| char::from_u32(0x0000).unwrap())
                    .parse_state(input)
            }

        fn backspace<I> (input: State<I>) -> ParseResult<char, I>
            where I: Stream<Item=char> {
                try(string("backspace"))
                    .map(|_| char::from_u32(0x0008).unwrap())
                    .parse_state(input)
            }

        fn vtab<I> (input: State<I>) -> ParseResult<char, I>
            where I: Stream<Item=char> {
                try(string("vtab"))
                    .map(|_| char::from_u32(0x000B).unwrap())
                    .parse_state(input)
            }

        fn page<I> (input: State<I>) -> ParseResult<char, I>
            where I: Stream<Item=char> {
                try(string("page"))
                    .map(|_| char::from_u32(0x000C).unwrap())
                    .parse_state(input)
            }

        fn retn<I> (input: State<I>) -> ParseResult<char, I>
            where I: Stream<Item=char> {
                try(string("return"))
                    .map(|_| char::from_u32(0x000D).unwrap())
                    .parse_state(input)
            }

        fn esc<I> (input: State<I>) -> ParseResult<char, I>
            where I: Stream<Item=char> {
                try(string("esc"))
                    .map(|_| char::from_u32(0x001B).unwrap())
                    .parse_state(input)
            }

        fn delete<I> (input: State<I>) -> ParseResult<char, I>
            where I: Stream<Item=char> {
                try(string("delete"))
                    .map(|_| char::from_u32(0x007F).unwrap())
                    .parse_state(input)
            }

        fn alarm<I> (input: State<I>) -> ParseResult<char, I>
            where I: Stream<Item=char> {
                try(string("alarm"))
                    .map(|_| char::from_u32(0x0007).unwrap())
                    .parse_state(input)
            }

        fn space<I> (input: State<I>) -> ParseResult<char, I>
            where I: Stream<Item=char> {
                try(string("space"))
                    .map(|_| char::from_u32(0x0020).unwrap())
                    .parse_state(input)
            }

        fn char_name<I> (input: State<I>) -> ParseResult<char, I>
            where I: Stream<Item=char> {
                parser(newline)
                    .or(parser(tab))
                    .or(parser(vtab))
                    .or(parser(backspace))
                    .or(parser(nul))
                    .or(parser(page))
                    .or(parser(retn))
                    .or(parser(esc))
                    .or(parser(delete))
                    .or(parser(alarm))
                    .or(parser(space))
                    .parse_state(input)
            }

        fn hex_char<I> (input: State<I>) -> ParseResult<char, I>
            where I: Stream<Item=char> {
                satisfy(|c| c == 'x')
                    .with(many1::<Vec<_>, _>(hex_digit()))
                    .map(|x| {
                        char::from_u32(
                            u32::from_str_radix(
                                x.iter()
                                 .fold(
                                    String::new(),
                                    |mut s: String, i| { s.push(*i); s })
                                 .as_slice(),
                            16).unwrap()
                        ).unwrap()
                    })
                    .parse_state(input)
            }

        string("#\\")
            .with(
                parser(char_name)
                .or(parser(hex_char))
                .or(parser(any_char))
            ).map(|c| CharNode { value: c})
            .parse_state(input)

}

/// Parses Scheme expressions.
#[allow(unconditional_recursion)]
pub fn expr<I>(input: State<I>) -> ParseResult<ExprNode, I>
    where I: Stream<Item=char> {
        let spaces = spaces();

        fn sexpr<I>(input: State<I>) -> ParseResult<ExprNode, I>
            where I: Stream<Item=char> {
                between(
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
                ).parse_state(input)
            }

        fn list<I>(input: State<I>) -> ParseResult<ExprNode, I>
            where I: Stream<Item=char> {
                between(
                    satisfy(|c| c == '('),
                    satisfy(|c| c == ')'),
                    many(parser(expr))
                        .map(|x| {
                            ListConst(ListNode {
                                elements: x
                            })
                        })
                ).parse_state(input)
            }

        spaces.clone().with(
            try(parser(sexpr))
                .or(try(parser(list)))
                .or(try(parser(name).map(Name)))
                .or(try(parser(number).map(NumConst)))
            ).parse_state(input)
}

#[cfg(test)]
mod tests;