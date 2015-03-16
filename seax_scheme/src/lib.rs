#[macro_use]
extern crate nom;

use nom::{IResult,Needed,FlatMapOpt,line_ending,not_line_ending, space, alphanumeric, multispace};
use nom::IResult::*;

use std::str;
use std::collections::HashMap;