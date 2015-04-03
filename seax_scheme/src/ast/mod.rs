use svm::cell::SVMCell;

use self::ExprNode::*;
use self::NumNode::*;
use super::ForkTable;
/// The symbol table for bound names is represented as a
/// `ForkTable` mapping `&str` (names) to `(uint,uint)` tuples,
/// representing the location in the `$e` stack storing the value
/// bound to that name.
pub type SymTable<'a>   = ForkTable<'a, &'a str, (usize,usize)>;
/// A `CompileResult` is either `Ok(SVMCell)` or `Err(&str)`.
pub type CompileResult  = Result<SVMCell, &'static str>;

static INDENT: &'static str = "\t";

/// Trait for AST nodes.
pub trait ASTNode {
    /// Compile this node to a list of SVM expressions
    fn compile(self, state: SymTable)   -> CompileResult;

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
    fn compile(self, state: SymTable)   -> CompileResult {
        match self {
            //  TODO: should some of these nodes cause a state fork?
            Root(node)          => node.compile(state),
            SExpr(node)         => node.compile(state),
            Name(node)          => node.compile(state),
            ListConst(node)     => node.compile(state),
            NumConst(node)      => node.compile(state),
            BoolConst(node)     => node.compile(state),
            CharConst(node)     => node.compile(state),
            StringConst(node)   => node.compile(state)
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
    fn compile(self, state: SymTable)   -> CompileResult {
        Err("UNINPLEMENTED")
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

    fn compile(self,state: SymTable)    -> CompileResult {
        Err("UNINPLEMENTED")
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

        for operand in self.operands.iter() {
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
    fn compile(self, state: SymTable)   -> CompileResult {
        Err("UNINPLEMENTED")
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


impl AsRef<str> for NameNode {

    /// Converts the `String` value of this node to a `&str` reference.
    ///
    /// # Examples:
    /// ```
    /// # use seax_scheme::ast::NameNode;
    /// let node = NameNode { name: "a string".to_string() };
    /// assert_eq!(node.as_ref(), "a string")
    /// ```
    fn as_ref(&self) -> &str {
        self.name.as_ref()
    }
}

impl ASTNode for NameNode {
    fn compile(self, state: SymTable)   -> CompileResult {
        Err("UNINPLEMENTED")
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
    fn compile(self, state: SymTable)   -> CompileResult {
        Err("UNINPLEMENTED")
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

    fn compile(self,state: SymTable)    -> CompileResult {
        Err("UNINPLEMENTED")
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
    fn compile(self, state: SymTable)   -> CompileResult {
        Err("UNINPLEMENTED")
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
    fn compile(self, state: SymTable)   -> CompileResult {
        Err("UNINPLEMENTED")
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

