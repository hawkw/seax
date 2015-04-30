#![feature(box_patterns,box_syntax)]
#![feature(scheme)]
#![feature(compile)]
#![feature(collections)]
extern crate rustc_serialize;
extern crate docopt;

extern crate seax_svm as svm;
extern crate seax_scheme as scheme;

#[macro_use]
extern crate log;

use docopt::Docopt;
use std::io;
use std::io::{Write, Read, BufRead,BufReader};
use svm::slist::{List,Stack};
use svm::cell::{SVMCell,Inst};
use svm::State;
use std::iter::FromIterator;
use std::error::Error;

static USAGE: &'static str = "
Usage:
    seax repl [-vd]
    seax [-vd] <bin>

Options:
    -v, --verbose   Enable verbose mode
    -d, --debug     Enable debug mode
";

#[derive(RustcDecodable)]
struct Args {
    cmd_repl: bool,
    arg_bin: String,
    flag_verbose: bool,
    flag_debug: bool,
}

mod loggers;

fn main() {
    let args: Args = Docopt::new(USAGE)
                .and_then(|d| d.decode())
                .unwrap_or_else(|e| e.exit());

    if args.flag_verbose {
        log::set_logger(|max_log_level| {
            max_log_level.set(log::LogLevelFilter::Debug);
            Box::new(loggers::DebugLogger)
        });
    } else {
        log::set_logger(|max_log_level| {
            max_log_level.set(log::LogLevelFilter::Info);
            Box::new(loggers::DefaultLogger)
        });
    };

    if args.cmd_repl {
        let mut stdin = BufReader::new(io::stdin());
        let mut stdout = io::stdout();

        print!("scheme> ");
        stdout.flush();

        for line in stdin.lines() {
            line.map_err(   |error   | String::from_str(error.description()) )
                .and_then(  |ref code| scheme::compile(code) )
                .map(       |program | svm::eval_program(program, true) )
                .map(       |result  | println!(">> {:?}", result) )
                .map_err(   |err     | error!("{}", err));
            print!("scheme> ");
            stdout.flush();
        }
    }
}
