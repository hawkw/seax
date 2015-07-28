Seax
====

A virtual machine-based platform for executing programs in functional programming language.

This project is split across a number of separate Cargo crates, each with its own Git repository:

  + __Seax Command-Line Application (this repository)__

    [![Build Status](https://img.shields.io/travis/hawkw/seax/master.svg?style=flat-square)](https://travis-ci.org/hawkw/seax) [![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](https://github.com/hawkw/seax/LICENSE)

    A command-line application for compiling programs to SVM bytecode, executing SVM bytecode files, and invoking the  the Scheme interpreter, either on source code files or as a REPL. This is so that the individual components of the system may be written as libraries rather than as executable programs. If additional compilers targeting the SVM are developed, this main program may invoke them as well. This repository also contains the main documentation and issue tracker for Seax.

  + __[Seax Virtual Machine (SVM)](https://github.com/hawkw/seax_svm)__

    [![Build Status](https://img.shields.io/travis/hawkw/seax_svm/master.svg?style=flat-square)](https://travis-ci.org/hawkw/seax_svm)
    [![Coverage](https://img.shields.io/codecov/c/github/hawkw/seax_svm/master.svg?style=flat-square)](http://codecov.io/github/hawkw/seax_svm?branch=master)
    [![Latest RustDoc](https://img.shields.io/badge/rustdoc-latest-green.svg?style=flat-square)](http://hawkweisman.me/seax/api/seax_svm/)
    [![Latest SVM release](https://img.shields.io/crates/v/seax_svm.svg?style=flat-square)](https://crates.io/crates/seax_svm)
    [![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](https://github.com/hawkw/seax/LICENSE)

    The core of the project, a virtual machine for evaluating Seax bytecode programs. SVM is based on the [SECD machine](http://en.wikipedia.org/wiki/SECD_machine) described by Peter Landin in 1963. This crate contains the main SECD implementation, definitions of the SVM instruction set and cell types, and a library for encoding and decoding Seax bytecode files. SVM is distributed as a library so that it may be included in other programs.

  + __[Seax Scheme Compiler](https://github.com/hawkw/seax_scheme)__

    [![Build Status](https://img.shields.io/travis/hawkw/seax_scheme/master.svg?style=flat-square)](https://travis-ci.org/hawkw/seax_scheme)
    [![Coverage](https://img.shields.io/codecov/c/github/hawkw/seax_scheme/master.svg?style=flat-square)](http://codecov.io/github/hawkw/seax_scheme?branch=master)
    [![Latest RustDoc](https://img.shields.io/badge/rustdoc-latest-green.svg?style=flat-square)](http://hawkweisman.me/seax/api/seax_scheme/)
    [![Latest Seax Scheme release](https://img.shields.io/crates/v/seax_scheme.svg?style=flat-square)](https://crates.io/crates/seax_scheme)
    [![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](https://github.com/hawkw/seax/LICENSE)

    A Scheme interpreter/compiler targeting the Seax VM. This implementation aims to conform with the [Revised<sup>6</sup> Report on Scheme](http://www.r6rs.org) (R6RS) whenever possible, but may not be a complatible implementation. `seax-scheme` is released as a library, rather than an executable, so that it may be included in other applications which use Scheme as an embedded language.

  + __[Seax Compiler Tools](https://github.com/hawkw/seax_compiler_tools)__

    A library of general-purpose reusable code for writing compilers targeting the Seax platform. This crate includes traits for abstract syntax tree nodes for Seax programs, and an implementation of the ForkTable data structure for representing scopes and symbol tables.

Instructions
------------

### Building Seax

Seax is implemented using the Rust programming language. In order to build Seax from source, you're going to need [Cargo](http://doc.crates.io/guide.html), Rust's build automation tool. Continuous integration builds of Seax are built against the latest Rust nightly. Therefore, backwards compatibility with earlier Rust versions are not always guaranteed.

If you have Cargo and an up-to-date Rust install, you can build Seax quite easily by running `cargo build --release` from the root directory. This will build all of the Seax libraries and the application, which will be output to `target/release/seax`.

Note that this builds the fully-optimized release Seax executable, and is intended for individuals who want to use Seax. Seax developers may want to build less optimized debug executables instead.

RustDoc documentation for Seax can be built using the `cargo doc` command.

### Using Seax

Seax currently supports the following commands:

+ `seax repl` launches the Scheme interpreter in [read-eval-print loop](http://en.wikipedia.org/wiki/Read–eval–print_loop) mode
+ `seax FILE.scm` invokes the interpreter on a Scheme source code file (`.scm`)

The following flags are also supported:

+ `-v` or `--verbose` launches Seax in verbose mode. Prepare yourself for a _great deal_ of debug logging if you enable this flag.
+ `-d` or `--debug` enables debugging state dumps from SVM fatal errors. This may incur a performance penalty.

Commands for running compiled Seax bytecode files and for compiling Scheme source code to Seax bytecode files will be added when these features reach a higher level of completion.

Documentation
-------------

In addition to the RustDoc API documentation provided for each Seax library, the [Seax book](http://hawkweisman.me/seax/) contains detailed information on the design and implementation of Seax, as well as instructions on writing programs targeting the Seax platform. Note that the Seax book is currently under active development and is not yet complete.
