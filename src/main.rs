#![feature(scheme)]
#![feature(compile)]
#![feature(convert)]
extern crate rustc_serialize;
extern crate docopt;

extern crate seax_svm as svm;
extern crate seax_scheme as scheme;

use docopt::Docopt;
use std::io;
use std::io::{Read, BufRead};

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


fn main() {
    let args: Args = Docopt::new(USAGE)
                .and_then(|d| d.decode())
                .unwrap_or_else(|e| e.exit());
    if args.cmd_repl {
        let mut console = io::stdin();
        let lock = console.lock();
        for line in lock.lines() {
            println!(">");
            match line {
                Ok(line) => { scheme::compile(line.as_ref())
                                .and_then(  |p| Ok(svm::eval_program(p, args.flag_debug)) )
                                .map(       |r| println!("{}", r) )
                                .map_err(   |e| println!("{}", e) ); },
                Err(why) => println!("{}", why)

            }
        }
    }
}
