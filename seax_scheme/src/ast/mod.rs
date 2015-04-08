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
pub type SymTable<'a>   = ForkTable<'a, &'a str, (usize,usize)>;
/// A `CompileResult` is either `Ok(SVMCell)` or `Err(&str)`.
pub type CompileResult  = Result<Vec<SVMCell>, String>;

static INDENT: &'static str = "\t";

/// Trait for AST nodes.
pub trait ASTNode {
    /// Compile this node to a list of SVM expressions
    fn compile<'a>(&'a self,
                   state: &'a SymTable<'a>
                   )                    -> CompileResult;

    /// Pretty-print this node
    fn prettyprint(&self)               -> String { self.print_level(0usize) }

    /// Pretty-print this node at the desired indent level
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
pub enum ExprNode {
    Root(RootNode),
    SExpr(SExprNode),
    Name(NameNode),
    ListConst(ListNode),
    NumConst(NumNode),
    BoolConst(BoolNode),
    StringConst(StringNode),
    CharConst(CharNode),
}

impl ASTNode for ExprNode {
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
pub enum NumNode {
    IntConst(IntNode),
    UIntConst(UIntNode),
    FloatConst(FloatNode)
}

/// AST node for the root of a program's AST
#[derive(Clone, PartialEq,Debug)]
pub struct RootNode { pub exprs: Vec<ExprNode> }

impl ASTNode for RootNode {
    fn compile<'a>(&'a self, state: &'a SymTable<'a>) -> CompileResult {
        Err("UNINPLEMENTED".to_string())
    }
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
pub struct SExprNode {
    pub operator: NameNode,
    pub operands: Vec<ExprNode>,
}

impl ASTNode for SExprNode {

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
pub struct ListNode { pub elements: Vec<ExprNode> }

impl ASTNode for ListNode {
    fn compile<'a>(&'a self, state: &SymTable<'a>) -> CompileResult {
        Err("UNINPLEMENTED".to_string())
    }
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
pub struct NameNode { pub name: String }

impl NameNode {
    /// Returns true if this is a keyword
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
    fn is_arith(&self) -> bool {
      match self.name.as_ref() {
         "+" | "-" | "*" | "/" | "%" => true,
         _ => false
      }
   }
    /// Returns true if this is a comparison operator
    fn is_cmp(&self) -> bool {
      match self.name.as_ref() {
         "=" | "!=" | ">" | "<" | ">=" | "<=" => true,
         _ => false
      }
   }
}

impl ASTNode for NameNode {
    fn compile<'a>(&'a self, state: &'a SymTable<'a>) -> CompileResult {
        Err("UNINPLEMENTED".to_string())
    }
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
pub struct IntNode { pub value: isize }

impl ASTNode for NumNode {
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
pub struct UIntNode { pub value: usize }

/// AST node for a floating-point constant
#[derive(Clone, PartialEq,Debug)]
pub struct FloatNode { pub value: f64 }

/// AST node for a boolean constant
#[derive(Clone, PartialEq,Debug)]
pub struct BoolNode { pub value: bool }

impl ASTNode for BoolNode {

    fn compile<'a>(&'a self,state:  &'a SymTable)    -> CompileResult {
        Err("UNINPLEMENTED".to_string())
    }

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
pub struct CharNode { pub value: char }

impl ASTNode for CharNode {
    fn compile<'a>(&'a self, state: &'a SymTable<'a>) -> CompileResult {
        Err("UNINPLEMENTED".to_string())
    }
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
pub struct StringNode { pub value: String }

impl ASTNode for StringNode {
    fn compile<'a>(&'a self, state: &'a SymTable<'a>) -> CompileResult {
        Err("UNINPLEMENTED".to_string())
    }
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

