#![feature(scheme)]
#![feature(compile)]
extern crate rustc_serialize;
extern crate docopt;

extern crate seax_svm as svm;
extern crate seax_scheme as scheme;

#[macro_use]
extern crate log;

use docopt::Docopt;
use std::io;
use std::io::{Write, Read, BufRead,BufReader};

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

    if args.flag_debug {
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
        for line in stdin.lines() {
            match line {
                Ok(ref line) => {

                    debug!("before compile, line is: {}", line);
                    scheme::compile(line)
                                .and_then(  |p| Ok(svm::eval_program(p, true) )  )
                                .map(       |r| println!(">> {}", r) )
                                .map_err(   |e| println!(">> {}", e) ); },
                Err(why) => println!("{}", why)
            }
            print!("scheme> ");
        }
    }
}
