#![feature(box_patterns,box_syntax)]
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
use svm::slist::{List,Stack};
use svm::cell::{SVMCell,Inst};
use svm::State;
use std::iter::FromIterator;

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

pub fn eval_program(program: &List<SVMCell>, debug: bool) -> List<SVMCell> {
    debug!("evaluating");
    let mut machine = State {
        stack:      Stack::empty(),
        env:        Stack::empty(),
        control:    program.clone(),
        dump:       Stack::empty()
    };
    debug!("made state");
    // while there are more instructions,
    while {
        machine.control.length() > 0usize &&
        machine.control.peek()!= Some(&SVMCell::InstCell(Inst::STOP))
    } {  //TODO: this is kinda heavyweight

        debug!("evaling");
        machine = machine.eval(None,debug).unwrap().0 // continue evaling
    };
    debug!("done, state: {:?}",machine);
    machine.stack
}

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
            match line {
                Ok(ref line) => {
                    debug!("before compile, line is: {}", line);
                    let program = scheme::compile(line)
                        .map(|prog: Vec<SVMCell> | {
                            debug!("compiled: {:?}",prog);
                            let result = List::from_iter(prog);
                            debug!("control stack: {:?}", result);
                            result
                         }).unwrap();
                    debug!("before eval: {:?}", program);
                    let result = eval_program(&program, true);
                    debug!("out of eval_program");
                    println!(">> {:?}", result);
                },
                Err(why) => println!("{}", why)
            }
            print!("scheme> ");
            stdout.flush();
        }
    }
}
