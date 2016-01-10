//  Seax
//  Copyright 2016 Hawk Weisman.
//
//  Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//  http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
//  <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
//  option. This file may not be copied, modified, or distributed
//  except according to those terms.
//! Command-line application for [Seax](hawkweisman.me/seax), a VM-based
//! platform for executing programs in functional languages.

extern crate rustc_serialize;
#![crate_name = "seax"]
#![crate_type = "bin"]
#![cfg_attr(test, feature(test))]
#![feature(scheme)]
#![feature(compile)]
#![feature(decode)]
#![feature(convert)]

extern crate docopt;
extern crate regex;

extern crate seax_svm as svm;
extern crate seax_util as util;
extern crate seax_scheme as scheme;

#[macro_use] extern crate log;
#[cfg(test)] extern crate test;

use docopt::Docopt;
use regex::Regex;

use std::io;
use std::io::{Write, Read, BufRead,BufReader};
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use std::convert::AsRef;

use svm::bytecode::decode_program;

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

#[cfg(test)] mod tests;

#[allow(dead_code)]
fn main() {
    let args: Args = Docopt::new(USAGE)
                .and_then(|d| d.decode())
                .unwrap_or_else(|e| e.exit());

    let ext_re = Regex::new(r"(?P<name>.+)?(?P<ext>\.[^.]*$|$)").unwrap();

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
        let file = File::create(&PathBuf::from(args.arg_file.as_str()))
            .map_err(|error| String::from(error.description()) );
        let (name, extension) = ext_re // file name and  extension
            .captures(args.arg_file.as_ref())
            .and_then(|c| (c.name("name"), c.name("ext")) );
        let result = match extension {
            Some(".scm") => { // compile scheme
                debug!("Compiling Scheme file {}", args.arg_file);
                file.and_then( |mut file| {
                        let mut s = String::new();
                        file.read_to_string(&mut s).map(|_| s)
                            .map_err(|error| String::from(error.description()) )
                        })
                    .and_then( |ref code| scheme::compile(code) )
                },
            _ => unimplemented!()
        }.and_then(|ref insts|
            File::create(name)
                .map_err(|error| String::from(error.description()) )
                .and_then(|mut file| file.write(&[0x5E,0xCD]))
            );
        match result {
            Ok(_) => println!("Compiled program to {}", name)
        }
    } else {
        let file = File::create(&PathBuf::from(args.arg_file.as_str()))
            .map_err(|error| String::from(error.description()) );
        let extension = ext_re // filename extension
            .captures(args.arg_file.as_ref())
            .and_then(|c| c.name("ext"));
        let result = match extension {
            Some(".scm") => { // interpret scheme
                debug!("Interpreting Scheme file {}", args.arg_file);
                file.and_then( |mut file| {
                        let mut s = String::new();
                        file.read_to_string(&mut s).map(|_| s)
                            .map_err(|error| String::from(error.description()) )
                        })
                    .and_then( |ref code| scheme::compile(code) )
            },
            _ => {
                debug!("Executing binary {}", args.arg_file);
                file.and_then( |mut file| decode_program(&mut file) )
            }
        }
        .and_then( |program | svm::eval_program(program, args.flag_debug) );
        match result {
            Ok(value)   => println!("===> {:?}", value),
            Err(why)    => error!("{}", why)
        };
    }
}
