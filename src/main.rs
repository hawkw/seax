#![feature(box_patterns,box_syntax)]
#![feature(scheme)]
#![feature(compile)]
#![feature(convert)]

//! Seax
//! ----
//!
//! Command-line application for [Seax](hawkweisman.me/seax), a VM-based
//! platform for executing programs in functional languages.

extern crate rustc_serialize;
extern crate docopt;
extern crate regex;

extern crate seax_svm as svm;
extern crate seax_scheme as scheme;

#[macro_use]
extern crate log;

use docopt::Docopt;
use regex::Regex;

use std::io;
use std::io::{Write, Read, BufRead,BufReader};
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use std::convert::AsRef;

#[allow(dead_code)]
static USAGE: &'static str = "
Usage:
    seax repl [-vd]
    seax [-vd] <file>
    seax compile [-vd] file

Options:
    -v, --verbose   Enable verbose mode
    -d, --debug     Enable debug mode
";

#[derive(RustcDecodable)]
#[allow(dead_code)]
struct Args {
    cmd_repl: bool,
    cmd_compile: bool,
    arg_file: String,
    flag_verbose: bool,
    flag_debug: bool,
}

mod loggers;

#[allow(dead_code)]
fn main() {
    let args: Args = Docopt::new(USAGE)
                .and_then(|d| d.decode())
                .unwrap_or_else(|e| e.exit());

    let ext_re = Regex::new(r".+?(?P<ext>\.[^.]*$|$)").unwrap();

    if args.flag_verbose {
        let _ = log::set_logger(|max_log_level| {
            max_log_level.set(log::LogLevelFilter::Debug);
            Box::new(loggers::DebugLogger)
        });
    } else {
        let _ = log::set_logger(|max_log_level| {
            max_log_level.set(log::LogLevelFilter::Info);
            Box::new(loggers::DefaultLogger)
        });
    };

    if args.cmd_repl {
        let stdin = BufReader::new(io::stdin());
        let mut stdout = io::stdout();

        print!("scheme> ");
        let _ = stdout.flush();

        for line in stdin.lines() {
            match line.map_err(|error| String::from(error.description()) )
                .and_then(  |ref code| scheme::compile(code) )
                .and_then(  |program | svm::eval_program(program, args.flag_debug) ) {
                    Ok(result)  => println!("===> {:?}",result),
                    Err(why)    => error!("{}", why)
                };
            print!("scheme> ");
            let _ = stdout.flush();
        }
    } else if args.cmd_compile {
        unimplemented!()
    } else {
        match ext_re
            .captures(args.arg_file.as_ref())
            .and_then(|c| c.name("ext")) {
            Some(".scm")   => { // interpret scheme
                debug!("Interpreting Scheme file {}", args.arg_file);
                let path = PathBuf::from(args.arg_file.as_str());
                match File::open(&path)
                    .map_err(|error    | String::from(error.description()) )
                    .and_then(|mut file| {
                        let mut s = String::new();
                        file.read_to_string(&mut s).map(|_| s)
                            .map_err(|error| String::from(error.description()) ) })
                    .and_then(  |ref code| scheme::compile(code) )
                    .and_then(  |program | svm::eval_program(program, args.flag_debug) ) {
                        Ok(result)  => println!("===> {:?}",result),
                        Err(why)    => error!("{}", why)
                };
            },
            _              => {
                debug!("Executing binary {}", args.arg_file);
                unimplemented!()
            } // bin, figure out
        }
    }
}
