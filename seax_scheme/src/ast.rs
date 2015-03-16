extern crate seax_svm;

use seax_svm::svm::cell::SVMCell;

/// Trait for AST nodes.
pub trait SyntaxTree {
    /// Compile this node to a list of SVM expressions
    fn compile(self)                    -> Vec<SVMCell>;

    /// Pretty-print this node at the desired indent level
    fn prettyprint(&self, level: usize) -> String;
}

/// Expression
///
/// All Seax Scheme expressions are one of the following
///  + Nested S-Expressions
///  + Identifiers
///  + Numbers
///     - signed int
///     - unsigned int
///     - float
///  + Characters
///  + Strings
pub enum Expr {
    SExpr(List<Expr>),
    Ident(String),
    SInt(isize),
    UInt(usize),
    Float(f64),
    Char(char),
    String(String)
}