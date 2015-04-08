use svm::cell::SVMCell;
use svm::cell::Atom::*;
use svm::cell::Inst::*;
use svm::cell::SVMCell::*;
use svm::slist::List::{Cons,Nil};

use self::ExprNode::*;
use self::NumNode::*;
use super::ForkTable;

#[cfg(test)]
mod tests;

/// The symbol table for bound names is represented as a
/// `ForkTable` mapping `&str` (names) to `(uint,uint)` tuples,
/// representing the location in the `$e` stack storing the value
/// bound to that name.

#[stable(feature = "forktable", since = "0.0.3")]
pub type SymTable<'a>   = ForkTable<'a, &'a str, (usize,usize)>;
/// A `CompileResult` is either `Ok(SVMCell)` or `Err(&str)`
#[stable(feature = "compile", since = "0.0.3")]
pub type CompileResult  = Result<Vec<SVMCell>, String>;

static INDENT: &'static str = "\t";

/// Trait for AST nodes.
#[stable(feature = "ast", since = "0.0.2")]
pub trait ASTNode {
    /// Compile this node to a list of SVM expressions
    #[unstable(feature="compile")]
    fn compile<'a>(&'a self,
                   state: &'a SymTable<'a>
                   )                    -> CompileResult;

    /// Pretty-print this node
    #[stable(feature = "ast", since = "0.0.2")]
    fn prettyprint(&self)               -> String { self.print_level(0usize) }

    /// Pretty-print this node at the desired indent level
    #[stable(feature = "ast", since = "0.0.2")]
    fn print_level(&self, level: usize) -> String;
}

/// Expression
///
/// All Seax Scheme expressions are one of the following
///
///  + Nested S-Expressions
///  + Identifiers
///  + Lists
///  + Numbers
///     - signed int
///     - unsigned int
///     - floating-point
///  + Characters
///  + Strings
///
///  TODO: implement the entire Scheme 'numeric tower'
///  TODO: macros should happen
///  TODO: figure out quasiquote somehow.
#[derive(Clone, PartialEq,Debug)]
#[stable(feature = "ast", since = "0.0.2")]
pub enum ExprNode {
    #[stable(feature = "ast", since = "0.0.2")]
    Root(RootNode),
    #[stable(feature = "ast", since = "0.0.2")]
    SExpr(SExprNode),
    #[stable(feature = "ast", since = "0.0.2")]
    Name(NameNode),
    #[stable(feature = "ast", since = "0.0.2")]
    ListConst(ListNode),
    #[stable(feature = "ast", since = "0.0.2")]
    NumConst(NumNode),
    #[stable(feature = "ast", since = "0.0.2")]
    BoolConst(BoolNode),
    #[stable(feature = "ast", since = "0.0.2")]
    StringConst(StringNode),
    #[stable(feature = "ast", since = "0.0.2")]
    CharConst(CharNode),
}

impl ASTNode for ExprNode {

    #[stable(feature = "compile", since = "0.0.3")]
    fn compile<'a>(&'a self, state: &'a SymTable<'a>) -> CompileResult {
        match *self {
            //  TODO: should some of these nodes cause a state fork?
            Root(ref node)          => node.compile(state),
            SExpr(ref node)         => node.compile(state),
            Name(ref node)          => node.compile(state),
            ListConst(ref node)     => node.compile(state),
            NumConst(ref node)      => node.compile(state),
            BoolConst(ref node)     => node.compile(state),
            CharConst(ref node)     => node.compile(state),
            StringConst(ref node)   => node.compile(state)
        }
    }

    #[stable(feature = "ast", since = "0.0.2")]
    fn print_level(&self, level: usize) -> String {
        match *self {
            Root(ref node)          => node.print_level(level),
            SExpr(ref node)         => node.print_level(level),
            Name(ref node)          => node.print_level(level),
            ListConst(ref node)     => node.print_level(level),
            NumConst(ref node)      => node.print_level(level),
            BoolConst(ref node)     => node.print_level(level),
            CharConst(ref node)     => node.print_level(level),
            StringConst(ref node)   => node.print_level(level)
        }
    }
}


#[derive(Clone, PartialEq,Debug)]
#[stable(feature = "ast", since = "0.0.2")]
pub enum NumNode {
    #[stable(feature = "ast", since = "0.0.2")]
    IntConst(IntNode),
    #[stable(feature = "ast", since = "0.0.2")]
    UIntConst(UIntNode),
    #[stable(feature = "ast", since = "0.0.2")]
    FloatConst(FloatNode)
}

/// AST node for the root of a program's AST
#[derive(Clone, PartialEq,Debug)]
#[stable(feature = "ast", since = "0.0.2")]
pub struct RootNode { pub exprs: Vec<ExprNode> }

impl ASTNode for RootNode {
    #[unstable(feature="compile")]
    fn compile<'a>(&'a self, state: &'a SymTable<'a>) -> CompileResult {
        Err("UNINPLEMENTED".to_string())
    }

    #[stable(feature = "ast", since = "0.0.2")]
    fn print_level(&self, level: usize) -> String {
        self.exprs
            .iter()
            .fold(
                String::new(),
                |mut s, i| {
                    s.push_str(i.print_level(level + 1).as_ref());
                    s
                })
    }

}

/// AST node for an S-expression.
///
/// This includes function application, assignment,
/// function definition, et cetera...Scheme is not a complexl anguage.
#[derive(Clone, PartialEq,Debug)]
#[stable(feature = "ast", since = "0.0.2")]
pub struct SExprNode {
    #[stable(feature = "ast", since = "0.0.2")]
    pub operator: NameNode,
    #[stable(feature = "ast", since = "0.0.2")]
    pub operands: Vec<ExprNode>,
}

impl ASTNode for SExprNode {
    #[unstable(feature="compile")]
    fn compile<'a>(&'a self, state: &'a SymTable<'a>) -> CompileResult {
        match self.operator {
            ref op if op.is_arith() || op.is_cmp() => {
                let instruction = match op.name.as_ref() {
                    "+"  => ADD,
                    "-"  => SUB,
                    "*"  => MUL,
                    "/"  => DIV,
                    "%"  => MOD,
                    "="  => EQ,
                    ">"  => GT,
                    ">=" => GTE,
                    "<"  => LT,
                    "<=" => LTE,
                    // TODO:  floating-point
                    // TODO: figure out how to handle "!=" -> "EQ + Invert"
                    _   => panic!( "Something impossible happened!")
                        // this never happens, barring force majeure
                };
                // TODO: optimize if constants
                let mut result = Vec::new();
                let mut it = self.operands.iter().rev();
                // TODO: can thsi be represented with a reduce/fold?
                result.push_all(try!(
                    it.next().unwrap().compile(state)).as_slice());
                for ref operand in it {
                    result.push_all(try!(operand.compile(state)).as_slice());
                    result.push(InstCell(instruction));
                }
                Ok(result)
            },
            ref op if op.is_kw()    => unimplemented!(),
            ref op                  => match state.get(&op.name.as_ref()) {
                Some(&(x,y)) => Ok(vec!(
                    // TODO: finish
                    InstCell(LD),
                    ListCell(box list!(
                        AtomCell(UInt(x)),
                        AtomCell(UInt(y))
                        )),
                    InstCell(LDF)
                    )),
                None         => Err(format!("[error] Unknown identifier `{}`", op.name))
            }
        }
        /*let ref token = self.operator.name;
        match token.as_ref() {
            "let" => unimplemented!(),
            ref name if state.chain_contains_key(name) => unimplemented!(),
            name => Err("[error] Unknown identifier")
        }*/
    }

    #[stable(feature = "ast", since = "0.0.2")]
    fn print_level(&self, level: usize) -> String {
        let mut tab = String::new();
        for _ in 0 .. level { tab.push_str(INDENT); };

        let mut result = String::new();
        result.push_str("S-Expression:\n");
        tab.push_str(INDENT);

        // op
        result.push_str(tab.as_ref());
        result.push_str("Operator:\n");
        result.push_str(self.operator.print_level(level + 1).as_ref());
        result.push('\n');

        for ref operand in self.operands.iter() {
            result.push_str(tab.as_ref());
            result.push_str("Operand: \n");
            result.push_str(operand.print_level(level + 1).as_ref());
            result.push('\n');
        };
        result
    }

}

/// AST node for a list literal
#[derive(Clone, PartialEq,Debug)]
#[stable(feature = "ast", since = "0.0.2")]
pub struct ListNode { pub elements: Vec<ExprNode> }

impl ASTNode for ListNode {
    #[unstable(feature="compile")]
    fn compile<'a>(&'a self, state: &SymTable<'a>) -> CompileResult {
        Err("UNINPLEMENTED".to_string())
    }

    #[stable(feature = "ast", since = "0.0.2")]
    fn print_level(&self, level: usize) -> String {
        let mut tab = String::new();
        for _ in 0 .. level {tab.push_str(INDENT); };

        let mut result = String::new();
        result.push_str("List:\n");
        tab.push_str(INDENT);

        for elem in self.elements.iter() {
            result.push_str(tab.as_ref());
            result.push_str(elem.print_level(level + 1).as_ref());
            result.push('\n');
        };
        result
    }

}

/// AST node for an identifier
#[derive(Clone, PartialEq,Debug)]
#[stable(feature = "ast", since = "0.0.2")]
pub struct NameNode { pub name: String }

impl NameNode {
    /// Returns true if this is a keyword
    #[stable(feature = "ast", since = "0.0.3")]
    fn is_kw(&self) -> bool {
        match self.name.as_ref() {
            "access" | "define-syntax" | "macro"  | "and"  | "delay"
            | "make-environment" | "begin"  | "do"| "named-lambda"
            | "bkpt" | "fluid-let" | "or" | "case" | "if" | "quasiquote"
            | "cond" | "in-package" | "quote" | "cons-stream" | "lambda"
            | "scode-quote" | "declare" | "let" | "sequence" | "default-object?"
            | "let*" | "set!" | "define" | "let-syntax" | "the-environment"
            | "define-integrable" | "letrec" | "unassigned?" | "define-macro"
            | "local-declare" | "using-syntax" | "define-structure" => true,
            _ => false
        }
    }
    /// Returns true if this is an arithmetic operator
    #[stable(feature = "ast", since = "0.0.3")]
    fn is_arith(&self) -> bool {
      match self.name.as_ref() {
         "+" | "-" | "*" | "/" | "%" => true,
         _ => false
      }
   }
    /// Returns true if this is a comparison operator
    #[stable(feature = "ast", since = "0.0.3")]
    fn is_cmp(&self) -> bool {
      match self.name.as_ref() {
         "=" | "!=" | ">" | "<" | ">=" | "<=" => true,
         _ => false
      }
   }

   #[stable(feature = "ast", since = "0.0.4")]
   pub fn new(name: String) -> Self { NameNode {name: name} }
}

impl ASTNode for NameNode {
    #[unstable(feature="compile")]
    fn compile<'a>(&'a self, state: &'a SymTable<'a>) -> CompileResult {
        Err("UNINPLEMENTED".to_string())
    }

    #[stable(feature = "ast", since = "0.0.2")]
    fn print_level(&self, level: usize) -> String {
        let mut tab = String::new();
        for _ in 0 .. level {tab.push_str(INDENT)};

        let mut result = String::new();
        result.push_str(tab.as_ref());
        result.push_str("Name: ");
        result.push_str(self.name.as_ref());
        result.push_str("\n");

        result
    }

}

/// AST node for an integer constant
#[derive(Clone, PartialEq,Debug)]
#[stable(feature = "ast", since = "0.0.2")]
pub struct IntNode {
    #[stable(feature = "ast", since = "0.0.2")]
    pub value: isize
}

impl ASTNode for NumNode {
    #[stable(feature="compile",since="0.0.3")]
    fn compile<'a>(&'a self, state: &'a SymTable<'a>) -> CompileResult {
       match *self {
            NumNode::UIntConst(ref node)    => Ok(
                    vec![InstCell(LDC),AtomCell(UInt(node.value))]
                ),
            NumNode::IntConst(ref node)     => Ok(
                    vec![InstCell(LDC),AtomCell(SInt(node.value))]
                ),
            NumNode::FloatConst(ref node)   => Ok(
                    vec![InstCell(LDC),AtomCell(Float(node.value))]
                )
       }
    }

    #[stable(feature = "ast", since = "0.0.2")]
    fn print_level(&self, level: usize) -> String {
        let mut tab = String::new();
        for _ in 0 .. level {tab.push_str(INDENT);};

        let mut result = String::new();

        result.push_str(tab.as_ref());
        result.push_str("Number: ");

        match *self {
            NumNode::UIntConst(ref node) => {
                result.push_str(format!("{}u", node.value).as_ref());
                result.push_str("\n");
            },
            NumNode::IntConst(ref node) => {
                result.push_str(format!("{}", node.value).as_ref());
                result.push_str("\n");
            },
            NumNode::FloatConst(ref node) => {
                result.push_str(format!("{}f", node.value).as_ref());
                result.push_str("\n");
            }
        }
        result
    }
}

/// AST node for an unsigned integer constant
#[derive(Clone, PartialEq,Debug)]
#[stable(feature = "ast", since = "0.0.2")]
pub struct UIntNode {
    #[stable(feature = "ast", since = "0.0.2")]
    pub value: usize
}

/// AST node for a floating-point constant
#[derive(Clone, PartialEq,Debug)]
#[stable(feature = "ast", since = "0.0.2")]
pub struct FloatNode {
    #[stable(feature = "ast", since = "0.0.2")]
    pub value: f64
}

/// AST node for a boolean constant
#[derive(Clone, PartialEq,Debug)]
#[stable(feature = "ast", since = "0.0.2")]
pub struct BoolNode {
    #[stable(feature = "ast", since = "0.0.2")]
    pub value: bool
}

impl ASTNode for BoolNode {
    #[unstable(feature="compile")]
    fn compile<'a>(&'a self,state:  &'a SymTable)    -> CompileResult {
        Err("UNINPLEMENTED".to_string())
    }

    #[stable(feature = "ast", since = "0.0.2")]
    fn print_level(&self, level: usize) -> String {
        let mut tab = String::new();
        for _ in 0 .. level {tab.push_str(INDENT);};

        let mut result = String::new();

        result.push_str(tab.as_ref());
        result.push_str("Boolean: ");
        result.push_str(format!("{}", self.value).as_ref());
        result.push_str("\n");
        result
    }
}


/// AST node for a character constant
#[derive(Clone, PartialEq,Debug)]
#[stable(feature = "ast", since = "0.0.2")]
pub struct CharNode {
    #[stable(feature = "ast", since = "0.0.2")]
    pub value: char
}

impl ASTNode for CharNode {
    #[unstable(feature="compile")]
    fn compile<'a>(&'a self, state: &'a SymTable<'a>) -> CompileResult {
        Err("UNINPLEMENTED".to_string())
    }
    #[stable(feature = "ast", since = "0.0.2")]
    fn print_level(&self, level: usize) -> String {
        let mut tab = String::new();
        for _ in 0 .. level {tab.push_str(INDENT);};

        let mut result = String::new();

        result.push_str("Character: \'");
        result.push(self.value);
        result.push_str("\'\n");
        result
    }
}


/// AST node for a  string constant
#[derive(Clone, PartialEq,Debug)]
#[stable(feature = "ast", since = "0.0.2")]
pub struct StringNode { pub value: String }

impl ASTNode for StringNode {
    #[unstable(feature="compile")]
    fn compile<'a>(&'a self, state: &'a SymTable<'a>) -> CompileResult {
        Err("UNINPLEMENTED".to_string())
    }
    #[stable(feature = "ast", since = "0.0.2")]
    fn print_level(&self, level: usize) -> String {
        let mut tab = String::new();
        for _ in 0 .. level {tab.push_str(INDENT);};

        let mut result = String::new();

        result.push_str("String: \"");
        result.push_str(self.value.as_ref());
        result.push_str("\"\n");
        result
    }
}

