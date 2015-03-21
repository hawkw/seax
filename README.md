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

A Scheme interpreter/compiler targeting the Seax VM. This implementation aims to conform with the [Revised<sup>6</sup> Report on Scheme](http://www.r6rs.org) (R6RS) whenever possible, but may not be a complatible implementation. `seax-scheme` is released as a library, rather than an executable, so that it may be included in other applications which use Scheme as an embedded language.


### Seax

The Seax main crate will contain a simple command-line wrapper for invoking the Scheme interpreter, either on source code files or as a REPL, compiling Scheme programs to SVM bytecode, and executing SVM bytecode files. This is so that the individual components of the system may be written as libraries rather than as executable programs. If additional compilers targeting the SVM are developed, this main program may invoke them as well.
