#![crate_name = "seax_scheme"]
#![crate_type = "lib"]
#![feature(convert,core,box_syntax,box_patterns)]

#[macro_use]
extern crate seax_svm as svm;

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
/// An associative map data structure for representing scopes.
///
/// A `ForkTable` functions similarly to a standard associative map
/// data structure (such as a `HashMap`), but with the ability to
/// fork children off of each level of the map. If a key exists in any
/// of a child's parents, the child will 'pass through' that key. If a
/// new value is bound to a key in a child level, that child will overwrite
/// the previous entry with the new one, but the previous `key` -> `value`
/// mapping will remain in the level it is defined. This means that the parent
/// level will still provide the previous value for that key.
///
/// This is an implementation of the ForkTable data structure for
/// representing scopes. The ForkTable was initially described by
/// Max Clive. This implemention is based primarily by the Scala
/// reference implementation written by Hawk Weisman for the Decaf
/// compiler, which is available [here](https://github.com/hawkw/decaf/blob/master/src/main/scala/com/meteorcode/common/ForkTable.scala).
pub use self::forktab::ForkTable;

