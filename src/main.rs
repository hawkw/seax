#![feature(box_patterns,box_syntax)]
#![feature(scheme)]
#![feature(compile)]
#![feature(collections)]
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
use std::io::{Write, BufRead,BufReader};
use std::error::Error;

static USAGE: &'static str = "
Usage:
    seax repl [-vd]
    seax [-vd] <file>

Options:
    -v, --verbose   Enable verbose mode
    -d, --debug     Enable debug mode
";

#[derive(RustcDecodable)]
struct Args {
    cmd_repl: bool,
    arg_file: String,
    flag_verbose: bool,
    flag_debug: bool,
}

mod loggers;

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
            match line.map_err(|error| String::from_str(error.description()) )
                .and_then(  |ref code| scheme::compile(code) )
                .and_then(  |program | svm::eval_program(program, args.flag_debug) ) {
                    Ok(result)  => println!(">> {:?}",result),
                    Err(why)    => error!("{}", why)
                };
            print!("scheme> ");
            let _ = stdout.flush();
        }
    } else {
        match ext_re
            .captures(args.arg_file.as_ref())
            .and_then(|c| c.name("ext")) {
            Some(".scm")   => {
                debug!("Interpreting Scheme file {}", args.arg_file);
                unimplemented!()
                }, // interpret scheme
            _              => {
                debug!("Executing binary {}", args.arg_file);
                unimplemented!()
            } // bin, figure out
        }
    }
}
