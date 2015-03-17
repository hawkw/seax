use seax_svm::svm::cell::SVMCell;
use seax_svm::slist::List;

/// Trait for AST nodes.
pub trait ASTNode {
    /// Compile this node to a list of SVM expressions
    fn compile(self)                        -> Result<SVMCell, &'static str>;

    /// Pretty-print this node
    fn prettyprint(&self)                   -> String {
        prettyprint_level(&self, 0usize)
    }

    /// Pretty-print this node at the desired indent level
    fn print_level(&self, level: usize)     -> String;
}

/// Expression
///
/// All Seax Scheme expressions are one of the following
///  + Nested S-Expressions
///  + Identifiers
///  + Lists
///  + Numbers
///     - signed int
///     - unsigned int
///     - floating-point
///  + Characters
///  + Strings
///  TODO: implement the entire Scheme 'numeric tower'
///  TODO: macros should happen
#[deriving(Clone, PartialEq)]
pub enum ExprNode {
    Root(RootNode),
    SExpr(SExprNode),
    Name(NameNode),
    List(ListNode),
    IntConst(IntNode),
    UIntConst(UIntNode),
    FloatConst(FloatNode),
    BoolConst(BoolNode),
    StringConst(StringNode),
    CharConst(CharNode),
}

/// AST node for the root of a program's AST
#[deriving(Clone, PartialEq)]
pub struct RootNode { pub exprs: Vec<ExprNode> }

/// AST node for an S-expression.
///
/// This includes function application, assignment,
/// function definition, et cetera...Scheme is not a complexl anguage.
#[deriving(Clone, PartialEq)]
pub struct SExprNode {
    pub operator: NameNode,
    pub operands: Vec<ExprNode>,
}

/// AST node for a list literal
#[deriving(Clone, PartialEq)]
pub struct ListNode { pub elements: Vec<ExprNode> }

/// AST node for an identifier
#[deriving(Clone, PartialEq)]
pub struct NameNode { pub name: String }

/// AST node for an integer constant
#[deriving(Clone, PartialEq)]
pub struct IntNode { pub value: isize }

/// AST node for an unsigned integer constant
#[deriving(Clone, PartialEq)]
pub struct UIntNode { pub value: usize }

/// AST node for a floating-point constant
#[deriving(Clone, PartialEq)]
pub struct FloatNode { pub value: f64 }

/// AST node for a boolean constant
#[deriving(Clone, PartialEq)]
pub struct BoolNode { pub value: bool }

/// AST node for a character constant
#[deriving(Clone, PartialEq)]
pub struct CharNode { pub value: char }

