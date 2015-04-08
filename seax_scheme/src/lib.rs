#![crate_name = "seax_scheme"]
#![crate_type = "lib"]
#![feature(convert,core,box_syntax,box_patterns,slice_patterns,collections)]

//! Library for compiling Scheme programs to Seax SVM bytecode.
//!
//! This Scheme implementation is based on the Scheme programming
//! language described in the
//! [Revised<sup>6</sup> Report on Scheme](http://www.r6rs.org/)
//! (R<sup>6</sup>RS). Any cases in which Seax Scheme differs from
//! R<sup>6</sup>RS Scheme are documented in the RustDoc for the
//! Seax Scheme library and in the Seax Scheme language manual.
//! Do note, however, that some inconsistencies between Seax Scheme
//! and R<sup>6</sup>RS may be the result of unimplemented features in
//! Seax Scheme.

#[macro_use]
extern crate seax_svm as svm;

/// Contains the Scheme abstract syntax tree (AST).
///
/// The AST stores the semantic structure of a parsed Scheme
/// program, and is responsible for compiling those programs
/// to SVM bytecode instructions, performing semantic analysis
/// (as necessary), and (eventually) for optimizing programs.
pub mod ast;

/// Contains the Scheme parser.
///
/// This parser is based on the
/// [Scheme grammar](final/html/r6rs/r6rs-Z-H-7.html) given in the
/// [Revised<sup>6</sup> Report on Scheme](http://www.r6rs.org/) (R<sup>6</sup>RS).
/// Any deviations from the R6RS standard, especially those with an impact
/// on the valid programs accepted by the parser, will be noted in the
/// parser's RustDoc.
pub mod parser;

mod forktab;

pub use self::forktab::ForkTable;

use svm::slist::{List,Stack};
use svm::cell::SVMCell;

use self::ast::{ASTNode,ExprNode};


/// Compile a Scheme program into a list of SVM cells (a control stack)
pub fn compile(program: &str) -> Result<List<SVMCell>, String> {
    parser::parse(program)
        .and_then(|tree: ExprNode      | tree.compile(&ForkTable::new()) )
        .map(     |insts: Vec<SVMCell> | insts.into_iter().rev().fold(List::new(),
                  |st: List<SVMCell>, i| st.push(i)
            ))
}