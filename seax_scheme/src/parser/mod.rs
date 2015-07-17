extern crate parser_combinators;

use self::parser_combinators::{try, between, parser, many, many1, any_char,
    optional, hex_digit, not_followed_by, skip_many, newline,ParserExt,};
use self::parser_combinators::combinator::With;
use self::parser_combinators::primitives::{Parser, ParseResult, State};
use self::parser_combinators::char::{spaces,Spaces,digit,satisfy,string};

use super::ast::*;
use super::ast::ExprNode::*;

use std::str::FromStr;
use std::char;
use std::error::Error;

#[cfg(test)]
mod tests;

fn lex<'a, P>(p: P) -> With<Spaces<&'a str>, P>
    where P: Parser<Input=&'a str> {
    spaces().with(p)
}

#[stable(feature="parser",since="0.0.2")]
fn hex_scalar(input: State<&str>) -> ParseResult<String, &str> {
    satisfy(|c| c == 'x' || c == 'X')
        .with( many1(hex_digit()) )
        .parse_state(input)
}

/// Parser for signed integer constants.
///
/// This parses signed integer constants in decimal and hexadecimal.
///
/// TODO: add support for octal
/// TODO: add support for binary
/// TODO: add support for R6RS exponents
#[unstable(feature="parser")]
pub fn sint_const(input: State<&str>) -> ParseResult<NumNode, &str> {

    fn hex_int(input: State<&str>) -> ParseResult<isize, &str> {
        satisfy(|c| c == '#')
            .with(parser(hex_scalar)
                    .map(|x| isize::from_str_radix(x.as_ref(), 16).unwrap()) )
            .parse_state(input)
    }

    fn dec_int(input: State<&str>) -> ParseResult<isize, &str> {
        optional(satisfy(|c| c == '#')
            .and(satisfy(|c| c == 'd' || c == 'D')))
            .with(many1::<String, _>(digit())
                .map(|x| isize::from_str(x.as_ref()).unwrap() ))
            .parse_state(input)
    }

    fn signed(input: State<&str>) -> ParseResult<(Option<char>,isize), &str> {
        optional(satisfy(|c| c == '-'))
            .and(
                try(parser(hex_int))
                .or(parser(dec_int))
                )
            .parse_state(input)
    }

    parser(signed)
        .map(|x| {
            if let Some(sign) = x.0 {
                let mut s = String::new();
                s.push(sign);
                s.push('1');
                x.1 * isize::from_str(s.as_ref()).unwrap()
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
#[unstable(feature="parser")]
pub fn uint_const(input: State<&str>) -> ParseResult<NumNode, &str> {

    fn hex_uint(input: State<&str>) -> ParseResult<usize, &str> {
        satisfy(|c| c == '#')
            .with(parser(hex_scalar)
                    .map(|x| usize::from_str_radix(x.as_ref(), 16).unwrap()) )
            .parse_state(input)
    }

    fn dec_uint(input: State<&str>) -> ParseResult<usize, &str> {
        many1::<String, _>(digit())
            .map(|x|usize::from_str(x.as_ref()).unwrap() )
            .parse_state(input)
    }

    try(parser(hex_uint))
        .or(parser(dec_uint))
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
#[stable(feature="parser",since="0.0.2")]
pub fn float_const(input: State<&str>) -> ParseResult<NumNode, &str> {

    fn float_str(input: State<&str>) -> ParseResult<((String, char), String), &str> {
        many1::<String,_>(digit())
            .and(satisfy(|c| c == '.'))
            .and(many1::<String, _>(digit()))
            .parse_state(input)
    }

    parser(float_str)
        .map(|x| {
            let s = format!("{}{}{}", (x.0).0, (x.0).1, x.1);
            NumNode::FloatConst(FloatNode{
                value: f64::from_str(s.as_ref()).unwrap()
            })
        })
        .skip(optional(satisfy(|c| c == 'f' || c == 'F')))
        .parse_state(input)
}

/// Parses boolean constants.
///
/// `#t`, `#T` -> `true`
/// `#f`, `#F` -> `false`
#[stable(feature="parser",since="0.0.2")]
pub fn bool_const(input: State<&str>) -> ParseResult<BoolNode, &str> {

    fn t_const(input: State<&str>) -> ParseResult<BoolNode, &str> {
        try(satisfy(|c| c == 't' || c == 'T'))
            .map(|_| BoolNode{ value: true })
            .parse_state(input)
    }

    fn f_const(input: State<&str>) -> ParseResult<BoolNode, &str> {
        try(satisfy(|c| c == 'f' || c == 'F'))
            .map(|_| BoolNode{ value: false })
            .parse_state(input)
    }

    satisfy(|c| c == '#')
        .with(parser(t_const)
            .or(parser(f_const))
        )
        .parse_state(input)
}

/// Parses a floating-point, signed integer, or unsigned integer constant.
#[stable(feature="parser",since="0.0.2")]
pub fn number(input: State<&str>) -> ParseResult<NumNode, &str> {
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
#[stable(feature="parser",since="0.0.2")]
pub fn name(input: State<&str>) -> ParseResult<NameNode, &str> {

    fn operator(input: State<&str>) -> ParseResult<String, &str> {

        fn single_op(input: State<&str>) -> ParseResult<String, &str> {
            satisfy( |c| c == '+' || c == '-' || c == '*' || c == '/' || c == '=')
                .map(|c| format!("{}", c))
                .parse_state(input)
        }

        parser(single_op)
            .or(string("!=")
                    .or(string(">="))
                    .or(string("<="))
                    .map(String::from)
                )
            .parse_state(input)
    }

    fn ident(input: State<&str>) -> ParseResult<String, &str> {

        fn initial(input: State<&str>) -> ParseResult<char, &str> {
            satisfy(|c|
                    c.is_alphabetic()
                    // R6RS 'special initial' characters
                    || c == '!' || c == '$' || c == '%' || c == ':' || c == '^'
                    || c == '<' || c == '>' || c == '_' || c == '~' || c == '\\'
                    || c == '?'
                    )
                .parse_state(input)
        }

        fn subsequent(input: State<&str>) -> ParseResult<char, &str> {
            satisfy(|c| c.is_alphanumeric()
                    // R6RS 'special initial' characters
                    || c == '!' || c == '$' || c == '%' || c == ':' || c == '^'
                    || c == '<' || c == '>' || c == '_' || c == '~' || c == '\\'
                    || c == '?'
                    // R6RS 'special subsequent' characters
                    || c == '+' || c == '-' || c == '.' || c == '@' )
                .parse_state(input)
        }

        fn rest(input: State<&str>) -> ParseResult<String, &str> {
            many::<String, _>(parser(subsequent))
                .parse_state(input)
        }

        parser(initial)
            .and(parser(rest))
            .map(|x| format!("{}{}", x.0, x.1) )
            .parse_state(input)
    }

    try(parser(operator))
        .or(parser(ident))
        .map(NameNode::new)
        .parse_state(input)
}

/// Recognizes R<sup>6</sup>RS character constants.
///
/// Character constants begin with the delimiter `#\` and may take
/// one of three forms:
///
/// 1. single ASCII character
///     + e.g. `#\a`, `#\Q`, `#\&` etc.
/// 2. R<sup>6</sup>RS named character
///     + e.g. `#\newline`, `#\tab` etc.
///     + please consult the [Revised<sup>6</sup> Report on Scheme](http://www.r6rs.org/) for a complete listing of valid character names
/// 3. Hex scalar value
///     + delimited with the character `x`
///     + e.g. `#\x1B` etc.
#[stable(feature="parser",since="0.0.2")]
pub fn character(input: State<&str>) -> ParseResult<CharNode, &str> {

    fn newline(input: State<&str>) -> ParseResult<char, &str> {
        try(string("newline"))
            .or(try(string("linefeed")))
            .map(|_| '\n')
            .parse_state(input)
    }

    fn tab(input: State<&str>) -> ParseResult<char, &str> {
        try(string("tab")).map(|_| '\t').parse_state(input)
    }

    fn nul(input: State<&str>) -> ParseResult<char, &str> {
        try(string("nul"))
            .map(|_| '\u{0000}')
            .parse_state(input)
    }

    fn backspace(input: State<&str>) -> ParseResult<char, &str> {
        try(string("backspace"))
            .map(|_| '\u{0008}')
            .parse_state(input)
    }

    fn vtab(input: State<&str>) -> ParseResult<char, &str> {
        try(string("vtab"))
            .map(|_| '\u{000B}')
            .parse_state(input)
    }

    fn page(input: State<&str>) -> ParseResult<char, &str> {
        try(string("page"))
            .map(|_| '\u{000C}')
            .parse_state(input)
    }

    fn retn(input: State<&str>) -> ParseResult<char, &str> {
        try(string("return"))
            .map(|_| '\u{000D}')
            .parse_state(input)
    }

    fn esc(input: State<&str>) -> ParseResult<char, &str> {
        try(string("esc"))
            .map(|_| '\u{001B}')
            .parse_state(input)
    }

    fn delete(input: State<&str>) -> ParseResult<char, &str> {
        try(string("delete"))
            .map(|_| '\u{007F}')
            .parse_state(input)
    }

    fn alarm(input: State<&str>) -> ParseResult<char, &str> {
        try(string("alarm"))
            .map(|_| '\u{0007}')
            .parse_state(input)
    }

    fn space(input: State<&str>) -> ParseResult<char, &str> {
        try(string("space"))
            .map(|_| '\u{0020}')
            .parse_state(input)
    }

    fn char_name(input: State<&str>) -> ParseResult<char, &str> {
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

    fn hex_char(input: State<&str>) -> ParseResult<char, &str> {
        parser(hex_scalar)
            .map(|x| char::from_u32(
                    u32::from_str_radix(x.as_ref(),16).unwrap()
                ).unwrap() )
            .parse_state(input)
    }

    string("#\\").with(
            parser(char_name)
            .or(parser(hex_char))
            .or(parser(any_char))
        ).map(|c| CharNode { value: c})
        .parse_state(input)
}

/// Parses a R<sup>6</sup>RS single-line comment
#[unstable(feature="parser")]
pub fn line_comment(input: State<&str>) -> ParseResult<(),&str> {
    satisfy(|c| c == ';')
        .with(skip_many(satisfy(|c| c != '\n')).skip(newline()))
        .parse_state(input)
}
#[stable(feature="parser",since="0.0.2")]
pub fn string_const(input: State<&str>) -> ParseResult<StringNode, &str> {

    fn escape_char(input: State<&str>) -> ParseResult<char, &str> {
        satisfy(|c| c == '\\')
            .with( satisfy(|c|
                    c == 'a' || c == 'b' || c == 't' || c == 'n' ||
                    c == 'v' || c == 'f' || c == 'r' || c == '\\' || c == '"')
                    .map(|c| match c {
                        '"'     => '"',
                        '\\'    => '\\',
                        '/'     => '/',
                        'b'     => '\u{0008}',
                        'f'     => '\u{000c}',
                        'n'     => '\n',
                        'r'     => '\r',
                        't'     => '\t',
                        _       => panic!("the impossible just happened!")
                    }) )
            .parse_state(input)
    }

    fn string_char(input: State<&str>) -> ParseResult<char, &str> {
        satisfy(|c| c != '\\' && c!= '"')
            .or(parser(escape_char))
            .parse_state(input)
    }

    between(
        satisfy(|c| c == '"'),
        satisfy(|c| c == '"'),
        many(parser(string_char)) )
    .map(|x| StringNode { value: x })
    .parse_state(input)
}

/// Parses Scheme expressions.
#[allow(unconditional_recursion)]
#[stable(feature="parser",since="0.1.1")]
pub fn expr(input: State<&str>) -> ParseResult<ExprNode, &str> {
    fn sexpr_inner(input: State<&str>) -> ParseResult<ExprNode, &str> {
        parser(expr)
            .and(lex(many(parser(expr))))
            .map(|x| SExpr(SExprNode {
                    operator: box x.0,
                    operands: x.1
                })
            )
            .parse_state(input)
    }

    fn sexpr(input: State<&str>) -> ParseResult<ExprNode, &str> {
        between(
            satisfy(|c| c == '('),
            lex(string(")").or(string(" )"))),
            lex(parser(sexpr_inner))
        ).or(
            between(
                satisfy(|c| c == '['),
                lex(satisfy(|c| c == ']')),
                lex(parser(sexpr_inner))
            )
        ).parse_state(input)
    }

    fn list(input: State<&str>) -> ParseResult<ExprNode, &str>{
        between(
            satisfy(|c| c == '('),
            lex(string(")").or(string(" )"))),
            lex(many(parser(expr))
                .map(|x| ListConst(ListNode {
                        elements: x
                    })
                ))
        ).parse_state(input)
    }

    fn constant(input: State<&str>) -> ParseResult<ExprNode, &str>{

        try(parser(number).map(NumConst))
            .or(try(parser(character).map(CharConst)))
            .or(try(parser(string_const).map(StringConst)))
            .or(try(parser(bool_const).map(BoolConst)))
            .parse_state(input)
    }

    fn non_constant(input: State<&str>) -> ParseResult<ExprNode, &str>{
        parser(sexpr)
            .or(parser(list))
            .or(parser(name).map(Name))
            .parse_state(input)
    }

    lex(try(optional(parser(line_comment))).with(
            lex(parser(non_constant))
                .or(parser(constant))
            ))
        .parse_state(input)
}
#[unstable(feature="parser")]
pub fn parse(program: &str) -> Result<ExprNode, String> {
    parser(expr) // todo: this should build a root node instead
        .parse(program)
        .map_err(|e| { let mut s = String::new(); s.push_str(e.description()); s} )
        .map(    |x| x.0 )
}
