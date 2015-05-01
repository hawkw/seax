Seax
====

[![Build Status](https://img.shields.io/travis/hawkw/seax/master.svg?style=flat-square)](https://travis-ci.org/hawkw/seax) [![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](https://github.com/hawkw/seax/LICENSE)

A SECD virtual machine for evaluating Lisp programs.

This project consists of three primary components in separate crates:

### Seax Virtual Machine (SVM)
[![Build Status](https://img.shields.io/travis/hawkw/seax/svm-dev.svg?style=flat-square)](https://travis-ci.org/hawkw/seax)
[![Latest RustDoc](https://img.shields.io/badge/rustdoc-latest-green.svg?style=flat-square)](http://hawkweisman.me/seax/api/seax_svm/)
[![Latest SVM release](https://img.shields.io/crates/v/seax_svm.svg?style=flat-square)](https://crates.io/crates/seax_svm)

The core of the project. A virtual machine based on the [SECD machine](http://en.wikipedia.org/wiki/SECD_machine) described by Peter Landin in 1963. SVM is distributed as a library so that it may be included in other programs.

### Seax Scheme

[![Build Status](https://img.shields.io/travis/hawkw/seax/scheme-dev.svg?style=flat-square)](https://travis-ci.org/hawkw/seax)
[![Latest RustDoc](https://img.shields.io/badge/rustdoc-latest-green.svg?style=flat-square)](http://hawkweisman.me/seax/api/seax_scheme/)
[![Latest Seax Scheme release](https://img.shields.io/crates/v/seax_scheme.svg?style=flat-square)](https://crates.io/crates/seax_scheme)

A Scheme interpreter/compiler targeting the Seax VM. This implementation aims to conform with the [Revised<sup>6</sup> Report on Scheme](http://www.r6rs.org) (R6RS) whenever possible, but may not be a complatible implementation. `seax-scheme` is released as a library, rather than an executable, so that it may be included in other applications which use Scheme as an embedded language.

### Seax

The Seax main crate will contain a simple command-line wrapper for invoking the Scheme interpreter, either on source code files or as a REPL, compiling Scheme programs to SVM bytecode, and executing SVM bytecode files. This is so that the individual components of the system may be written as libraries rather than as executable programs. If additional compilers targeting the SVM are developed, this main program may invoke them as well.

Instructions
------------

## Building Seax

In order to build Seax from source, you're going to need  [Cargo](http://doc.crates.io/guide.html), Rust's build automation tool. Continuous integration builds of Seax are built against the latest Rust nightly. Therefore, backwards compatibility with earlier Rust versions are not always guaranteed.

If you have Cargo and an up-to-date Rust install, you can build Seax quite easily by running `cargo build` from the root directory. This will build all of the Seax libraries and the application, which will be output to `target/debug/seax`.

## Using Seax

Seax currently supports the following commands:

+ `seax repl` launches the Scheme interpreter in [read-eval-print loop](http://en.wikipedia.org/wiki/Read–eval–print_loop) mode
+ `seax FILE.scm` invokes the interpreter on a Scheme source code file (`.scm`)

The following flags are also supported:

+ `-v` or `--verbose` launches Seax in verbose mode. Prepare yourself for a _great deal_ of debug logging if you enable this flag.
+ `-d` or `--debug` inables debugging state dumps from SVM fatal errors. This may incur a performance penalty.

Commands for running compiled Seax bytecode files and for compiling Scheme source code to Seax bytecode files will be added when these features reach a higher level of completion.
