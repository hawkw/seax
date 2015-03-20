#![crate_name = "seax_scheme"]
#![crate_type = "lib"]
#![feature(core)]

#[macro_use]
extern crate "seax_svm" as svm;

pub mod ast;
pub mod parser;