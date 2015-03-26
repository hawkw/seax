#![crate_name = "seax_scheme"]
#![crate_type = "lib"]
#![feature(core)]

#[macro_use]
extern crate "seax_svm" as svm;

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

