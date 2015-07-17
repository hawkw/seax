use svm::cell::SVMCell;
use svm::cell::Atom::*;
use svm::cell::Inst::*;
use svm::cell::SVMCell::*;
use svm::slist::{List,Stack};
use svm::slist::List::{Cons,Nil};

use self::ExprNode::*;
use self::NumNode::*;
use super::ForkTable;

use std::fmt;
use std::fmt::Write;
use std::iter::FromIterator;
use std::convert::Into;
use std::hash::Hash;

#[cfg(test)]
mod tests;

/// The symbol table for bound names is represented as a
/// `ForkTable` mapping `&str` (names) to `(uint,uint)` tuples,
/// representing the location in the `$e` stack storing the value
/// bound to that name.
#[stable(feature = "forktable", since = "0.0.6")]
pub type SymTable<'a> = ForkTable<'a, &'a str, (usize,usize)>;

/// A `CompileResult` is either `Ok(SVMCell)` or `Err(&str)`
#[stable(feature = "compile", since = "0.0.3")]
pub type CompileResult = Result<Vec<SVMCell>, String>;

static INDENT: &'static str = "    ";

/// Trait for a symbol table
#[stable(feature = "compile",since = "0.1.0")]
pub trait Scope<K> where K: Eq + Hash {
    /// Bind a name to a scope.
    ///
    /// Returnsthe indices for that name in the SVM environment.
    #[stable(feature = "compile",since = "0.1.0")]
    fn bind(&mut self, name: K, lvl: usize) -> (usize,usize);
    /// Look up a name against a scope.
    ///
    /// Returns the indices for that name in the SVM environment,
    /// or None if that name is unbound.
    #[stable(feature = "compile",since = "0.1.0")]
    fn lookup(&self, name: &K)              -> Option<(usize,usize)>;
}

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
    fn prettyprint(&self)               -> String { self.print_level(0) }

    /// Pretty-print this node at the desired indent level
    #[stable(feature = "ast", since = "0.0.2")]
    fn print_level(&self, level: usize) -> String;
}

impl fmt::Debug for ASTNode {
    #[stable(feature = "ast", since = "0.0.4")]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.prettyprint())
    }
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
#[derive(Clone, PartialEq)]
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

impl fmt::Debug for ExprNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.prettyprint())
    }
}

#[derive(Clone, PartialEq)]
#[stable(feature = "ast", since = "0.0.2")]
pub enum NumNode {
    #[stable(feature = "ast", since = "0.0.2")]
    IntConst(IntNode),
    #[stable(feature = "ast", since = "0.0.2")]
    UIntConst(UIntNode),
    #[stable(feature = "ast", since = "0.0.2")]
    FloatConst(FloatNode)
}

impl fmt::Debug for NumNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.prettyprint())
    }
}

/// AST node for the root of a program's AST
#[derive(Clone, PartialEq)]
#[stable(feature = "ast", since = "0.0.2")]
pub struct RootNode { pub exprs: Vec<ExprNode> }

impl fmt::Debug for RootNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.prettyprint())
    }
}

impl ASTNode for RootNode {
    #[unstable(feature="compile")]
    #[allow(unused_variables)]
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
#[derive(Clone, PartialEq)]
#[stable(feature = "ast", since = "0.0.2")]
pub struct SExprNode {
    #[stable(feature = "ast", since = "0.0.2")]
    pub operator: Box<ExprNode>,
    #[stable(feature = "ast", since = "0.0.2")]
    pub operands: Vec<ExprNode>,
}

impl ASTNode for SExprNode {

    /// Compile method for S-expression nodes.
    ///
    /// Since almost all of Scheme syntax is S-expressions,
    /// that means that this is responsible for compiling approximately
    /// ... everything.
    ///
    /// Abandon all hope, ye who enter here.
    #[unstable(feature="compile")]
    fn compile<'a>(&'a self, state: &'a SymTable<'a>) -> CompileResult {
        // TODO: break this monster apart into sub-functions
        // because this is a wretched abomination of cyclomatic complexity
        match self.operator {
            box Name(ref node) => match node.name.as_ref() {
                "if" => match self.operands.as_slice() {
                    [ref condition,ref true_case,ref false_case] => {
                        let mut result = Vec::new();

                        result.push_all(&try!(condition.compile(state)));
                        result.push(InstCell(SEL));

                        let mut false_code = try!(false_case.compile(state));
                        false_code.push(InstCell(JOIN));

                        let mut true_code = try!(true_case.compile(state));
                        true_code.push(InstCell(JOIN));

                        result.push(ListCell(box List::from_iter(true_code)));
                        result.push(ListCell(box List::from_iter(false_code)));

                        Ok(result)
                    },
                    _ => Err("[error]: malformed if expression".to_string())
                },
                "lambda" => match self.operands.as_slice() {
                    [SExpr(SExprNode{
                            operator: box Name(ref param_a),
                            operands: ref param_bs}), SExpr(ref body)] => {
                        let mut sym = state.fork(); // fork the symbol table
                        let depth = self.depth(); // cache the depth for binds

                        sym.bind(param_a.name.as_ref(),depth);

                        for b in param_bs {
                            if let &Name(ref node) = b {
                                sym.bind(node.name.as_ref(),depth);
                            } // todo: make errors otherwise
                        }

                        let mut result = Vec::new();
                        let mut func = try!(body.compile(&sym));
                        func.push(InstCell(RET));

                        result.push_all(&vec![
                            InstCell(LDF),
                            ListCell(box List::from_iter(func))
                        ]);

                        Ok(result)
                    },
                    _ => Err("[error]: malformed lambda expression".to_string())
                },
                "let" => match self.operands.as_slice() {
                    [SExpr(SExprNode{
                        operator: box SExpr(ref param_a),
                        operands: ref param_bs}), ref body_exp] => {

                        let mut sym = state.fork();
                        let mut result = Vec::new();
                        let depth = self.depth();

                        result.push(InstCell(NIL));

                        (match param_a {
                            &SExprNode{
                                operator: box Name(ref node),
                                operands: ref param_body
                            } => {

                                sym.bind(node.name.as_ref(),depth);

                                for exp in param_body {
                                    result.push_all(&try!(exp.compile(&sym)));
                                }

                                result.push(InstCell(CONS));

                                Ok(result)
                            },
                            _ => Err("[error]: malformed let expression".to_string())
                        }).and_then(|mut result: Vec<SVMCell> | {
                            for param_b in param_bs {
                                if let &SExpr(SExprNode{
                                    operator: box Name(ref node),
                                    operands: ref param_body
                                }) = param_b {

                                    sym.bind(node.name.as_ref(),depth);

                                    for ref exp in param_body {
                                        result.push_all(&try!(exp.compile(&sym)));
                                    }

                                    result.push(InstCell(CONS));
                                }
                            }
                            Ok(result)
                        }).and_then(|mut result: Vec<SVMCell> | {

                            let mut body_code = Vec::new();
                            body_code.push_all(&try!(body_exp.compile(&sym)));
                            body_code.push(InstCell(RET));

                            result.push_all(&[
                                InstCell(LDF),
                                ListCell(box List::from_iter(body_code)),
                                InstCell(AP)
                            ]);

                            Ok(result)

                        })
                    },
                    _ => Err(format!("[error]: malformed let expression:\n{:?}",self))
                },
                _ => { // TODO: this is basically a duplicate of the general case
                       // I feel bad for doing it this way but nothing else worked
                    let ref op = self.operator;
                    let mut result = Vec::new();
                    match self.operands {
                        ref other if other.len() == 1 => {
                            result.push_all( &try!(other[0].compile(state)) );
                            result.push_all( &try!(op.compile(state)) );
                        },
                        _       => {
                            let mut it = self.operands.iter().rev();
                            // TODO: can thsi be represented with a reduce/fold?
                            result.push_all(&try!(
                                it.next().unwrap().compile(state)));
                            for ref operand in it {
                                result.push_all(&try!(operand.compile(state)));
                                result.push_all(&try!(op.compile(state)));
                            }
                        }
                    }
                    Ok(result)
                }
            },
            box ref op  => {
                let mut result = Vec::new();
                match self.operands {
                    ref other if other.len() == 1 => { // just an optimization
                        result.push(InstCell(NIL));
                        result.push_all( &try!(other[0].compile(state)) );
                        result.push(InstCell(CONS));
                        result.push_all( &try!(op.compile(state)) );
                        result.push(InstCell(AP));
                    },
                    _       => {
                        let mut it = self.operands.iter().rev();
                        // TODO: can thsi be represented with a reduce/fold?
                        result.push(InstCell(NIL));
                        result.push_all(&try!(
                            it.next().unwrap().compile(state)));
                        for ref operand in it {
                            result.push(InstCell(CONS));
                            result.push_all(&try!(operand.compile(state)));
                            result.push(InstCell(CONS));
                            result.push_all(&try!(op.compile(state)));
                            result.push(InstCell(AP));
                        }
                    }
                }
                Ok(result)
            }
        }

    }

    #[stable(feature = "ast", since = "0.0.6")]
    fn print_level(&self, level: usize) -> String {
        let mut tab = String::new();
        for _ in 0 .. level { tab.push_str(INDENT); };

        let mut result = String::new();
        write!(&mut result, "{}S-Expression:\n",tab).unwrap();
        tab.push_str(INDENT);

        // op
        write!(&mut result, "{}Operator:\n{}", tab,
            self.operator.print_level(level + 2)
        ).unwrap();

        for ref operand in self.operands.iter() {
            write!(&mut result, "{}Operand: \n{}", tab,
                operand.print_level(level + 2)
            ).unwrap();
        };
        result
    }

}

impl SExprNode {
    /// Tests to see if this node is a binding expression
    #[stable(feature = "compile",since = "0.1.1")]
    fn is_bind(&self) -> bool {
        // I wish I didn't have to do it this way, there's an apparent
        // Rust issue (rust-lang/rust#23762) that makes it impossible
        // to use `as_ref()` in a pattern guard.
        match *self.operator {
            // If you understand why it must be this way, then you have
            // achieved true enlightenment.
            Name(ref node)  => match node.name.as_ref() {
                "lambda" | "let" | "letrec" => true,
                _                           => false
            },
            _               => false
        }
    }
    /// Returns the depth of a nested lambda expression.
    ///
    /// This is used for generating indices into the VM's environment.
    /// Since the environment is treated as a stack, it is necessary to
    /// produce compile-time estimates of the size of a call stack for a
    /// closure with nested clsures, since we want to associate the names bound
    /// in the top level of the closure with the lowest level of the stack at
    /// evaluation time.
    #[stable(feature = "compile",since = "0.1.0")]
    fn depth(&self)     -> usize {
            self.operands.iter().fold(
                match *self.operator {
                    SExpr(ref node) => node.depth(),
                    Name(_)         => if self.is_bind() {1} else {0},
                    _               => 0
                },
            |acc, op| acc + match op {
                &SExpr(ref node)    => node.depth(),
                _                   => 0
            } )
    }
}

#[stable(feature = "ast", since = "0.0.4")]
impl fmt::Debug for SExprNode {
    #[stable(feature = "ast", since = "0.0.2")]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.prettyprint())
    }
}

/// AST node for a list literal
#[derive(Clone, PartialEq)]
#[stable(feature = "ast", since = "0.0.2")]
pub struct ListNode { pub elements: Vec<ExprNode> }
#[stable(feature = "ast", since = "0.0.4")]
impl fmt::Debug for ListNode {
    #[stable(feature = "ast", since = "0.0.4")]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.prettyprint())
    }
}

impl ASTNode for ListNode {
    #[unstable(feature="compile")]
    #[allow(unused_variables)]
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
            write!(&mut result, "{}{}\n", tab, elem.print_level(level + 1))
                .unwrap();
        };
        result
    }

}

/// AST node for an identifier
#[derive(Clone, PartialEq)]
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
            | "local-declare" | "using-syntax" | "define-structure" | "car"
            | "cdr" | "cons" | "nil" | "nil?" | "atom?" => true,
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
#[stable(feature = "ast", since = "0.0.4")]
impl fmt::Debug for NameNode {
    #[stable(feature = "ast", since = "0.0.4")]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.prettyprint())
    }
}

impl ASTNode for NameNode {
    #[unstable(feature="compile")]
    fn compile<'a>(&'a self, state: &'a SymTable<'a>) -> CompileResult {
        match self.name.as_ref() {
            "cons"  => Ok(vec![InstCell(CONS)]),
            "car"   => Ok(vec![InstCell(CAR)]),
            "cdr"   => Ok(vec![InstCell(CDR)]),
            "nil"   => Ok(vec![InstCell(NIL)]),
            "nil?"  => Ok(vec![InstCell(NULL)]),
            "atom?" => Ok(vec![InstCell(ATOM)]),
            "+"     => Ok(vec![InstCell(ADD)]),
            "-"     => Ok(vec![InstCell(SUB)]),
            "*"     => Ok(vec![InstCell(MUL)]),
            "/"     => Ok(vec![InstCell(DIV)]),
            "%"     => Ok(vec![InstCell(MOD)]),
            "="     => Ok(vec![InstCell(EQ)]),
            ">"     => Ok(vec![InstCell(GT)]),
            ">="    => Ok(vec![InstCell(GTE)]),
            "<"     => Ok(vec![InstCell(LT)]),
            "<="    => Ok(vec![InstCell(LTE)]),
            ref name => match state.lookup(&name) {
                Some((lvl,idx)) => Ok(vec![
                    InstCell(LD),
                    ListCell(box list!(
                        AtomCell(UInt(lvl)),
                        AtomCell(UInt(idx)))
                    )]),
                None => Err(format!("[error] Unknown identifier `{}`", name))
            }
        }
    }

    #[stable(feature = "ast", since = "0.0.2")]
    fn print_level(&self, level: usize) -> String {
        let mut tab = String::new();
        for _ in 0 .. level {tab.push_str(INDENT)};

        format!("{}Name: {}\n", tab, self.name)
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
    #[allow(unused_variables)]
    fn compile<'a>(&'a self, state: &'a SymTable<'a>) -> CompileResult {
       match *self {
            UIntConst(ref node)    =>
                Ok(vec![InstCell(LDC),AtomCell(UInt(node.value))]),
            IntConst(ref node)     =>
                Ok(vec![InstCell(LDC),AtomCell(SInt(node.value))]),
            FloatConst(ref node)   =>
                Ok(vec![InstCell(LDC),AtomCell(Float(node.value))])
       }
    }

    #[stable(feature = "ast", since = "0.0.2")]
    fn print_level(&self, level: usize) -> String {
        let mut tab = String::new();
        for _ in 0 .. level {tab.push_str(INDENT);};

        let mut result = String::new();

        write!(&mut result, "{}Number:\n", tab).unwrap();

        match *self {
            UIntConst(ref node)  => write!(&mut result, "{}u\n", node.value)
                .unwrap(),
            IntConst(ref node)   => write!(&mut result, "{}\n", node.value)
                .unwrap(),
            FloatConst(ref node) => write!(&mut result, "{}f\n", node.value)
                .unwrap()
        };

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
    #[stable(feature="compile", since="0.0.6")]
    #[allow(unused_variables)]
    fn compile<'a>(&'a self,state:  &'a SymTable)    -> CompileResult {
        match self.value {
            true    => Ok(vec![InstCell(LDC), AtomCell(SInt(1))]),
            false   => Ok(vec![InstCell(NIL)])
        }
    }

    #[stable(feature = "ast", since = "0.0.2")]
    fn print_level(&self, level: usize) -> String {
        let mut tab = String::new();
        for _ in 0 .. level {tab.push_str(INDENT);};

        format!("{}Boolean: {}\n", tab, self.value)
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
    #[stable(feature="compile", since="0.0.7")]
    #[allow(unused_variables)]
    fn compile<'a>(&'a self, state: &'a SymTable<'a>) -> CompileResult {
        Ok(vec![AtomCell(Char(self.value))])
    }
    #[stable(feature = "ast", since = "0.0.2")]
    fn print_level(&self, level: usize) -> String {
        let mut tab = String::new();
        for _ in 0 .. level {tab.push_str(INDENT);};

        format!("{}Character: \'{}\'\n", tab, self.value)
    }
}


/// AST node for a  string constant
#[derive(Clone, PartialEq,Debug)]
#[stable(feature = "ast", since = "0.0.2")]
pub struct StringNode { pub value: String }

impl ASTNode for StringNode {
    /// Method to compile a String.
    ///
    /// For now, this compiles strings into lists of characters.
    /// Eventually this may change.
    #[unstable(feature="compile")]
    #[allow(unused_variables)]
    fn compile<'a>(&'a self, state: &'a SymTable<'a>) -> CompileResult {
        let chars: Vec<u8> = self.value.clone().into();
        Ok(vec![
            ListCell(box List::from_iter(
                chars.into_iter().map(|c| AtomCell(Char(c as char)))
                )) ])
    }
    #[stable(feature = "ast", since = "0.0.2")]
    #[allow(unused_variables)]
    fn print_level(&self, level: usize) -> String {
        format!("String: \"{}\"\n", self.value)
    }
}
