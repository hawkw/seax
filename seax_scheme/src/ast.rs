use svm::cell::SVMCell;
use svm::slist::List;

/// Trait for AST nodes.
pub trait ASTNode {
    /// Compile this node to a list of SVM expressions
    fn compile(self)                    -> Result<SVMCell, &'static str>;

    /// Pretty-print this node
    fn prettyprint(&self)               -> String { self.print_level(0usize) }

    /// Pretty-print this node at the desired indent level
    fn print_level(&self, level: usize) -> String;
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
#[derive(Clone, PartialEq,Debug)]
pub enum ExprNode {
    Root(RootNode),
    SExpr(SExprNode),
    Name(NameNode),
    List(ListNode),
    NumConst(NumNode),
    BoolConst(BoolNode),
    StringConst(StringNode),
    CharConst(CharNode),
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

/// AST node for an S-expression.
///
/// This includes function application, assignment,
/// function definition, et cetera...Scheme is not a complexl anguage.
#[derive(Clone, PartialEq,Debug)]
pub struct SExprNode {
    pub operator: NameNode,
    pub operands: Vec<ExprNode>,
}

/// AST node for a list literal
#[derive(Clone, PartialEq,Debug)]
pub struct ListNode { pub elements: Vec<ExprNode> }

/// AST node for an identifier
#[derive(Clone, PartialEq,Debug)]
pub struct NameNode { pub name: String }

/// AST node for an integer constant
#[derive(Clone, PartialEq,Debug)]
pub struct IntNode { pub value: isize }

/// AST node for an unsigned integer constant
#[derive(Clone, PartialEq,Debug)]
pub struct UIntNode { pub value: usize }

/// AST node for a floating-point constant
#[derive(Clone, PartialEq,Debug)]
pub struct FloatNode { pub value: f64 }

/// AST node for a boolean constant
#[derive(Clone, PartialEq,Debug)]
pub struct BoolNode { pub value: bool }

/// AST node for a character constant
#[derive(Clone, PartialEq,Debug)]
pub struct CharNode { pub value: char }

/// AST node for a  string constant
#[derive(Clone, PartialEq,Debug)]
pub struct StringNode { pub value: String }
