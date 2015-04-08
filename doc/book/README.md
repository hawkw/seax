% Seax

#### Seax Virtual Machine (SVM)

[![Build Status](https://img.shields.io/travis/hawkw/seax/svm-dev.svg?style=flat-square)](https://travis-ci.org/hawkw/seax)
[![Latest RustDoc](https://img.shields.io/badge/rustdoc-latest-green.svg?style=flat-square)](http://hawkweisman.me/seax/api/seax_svm/)
[![Latest SVM release](https://img.shields.io/crates/v/seax_svm.svg?style=flat-square)](https://crates.io/crates/seax_svm)

#### Seax Scheme Compiler 

[![Build Status](https://img.shields.io/travis/hawkw/seax/scheme-dev.svg?style=flat-square)](https://travis-ci.org/hawkw/seax)
[![Latest RustDoc](https://img.shields.io/badge/rustdoc-latest-green.svg?style=flat-square)](http://hawkweisman.me/seax/api/seax_scheme/)
[![Latest Seax Scheme release](https://img.shields.io/crates/v/seax_scheme.svg?style=flat-square)](https://crates.io/crates/seax_svm)

Seax is a multi-language runtime environment for executing computer programs, with a focus on functional languages. At the core of Seax is a virtual machine based on an implementation of the [SECD machine](https://en.wikipedia.org/wiki/SECD_machine) first described by Peter J. Landin. 

Seax was implemented by [Hawk Weisman](http://hawkweisman.me) and is released under the [MIT license](https://github.com/hawkw/seax/blob/master/LICENSE). While Seax was written primarily for educational purposes, the goal of the project is to create a runtime environment which may be used by developers for real-world projects.

Do note that Seax is currently undergoing active development, and therefore, a number of planned features may currently be incomplete or wholly unimplemented.

The Seax documentation is split into three primary sections, which can be navigated using the table of contents on the left of this page.

<h2 class="section-header"><a href="background/index.html">Background</a></h2>

This section contains information regarding the motivation and goals behind Seax, and the antecedants and previous work that have informed its' implementation.

<h2 class="section-header"><a href="implementation">Implementation</a></h2>

This section details the implementation of Seax and its components, including descriptions of their architecture and functionality. Discussions of the design choices that went into the Seax's implementation are also furnished.

## Reference

This section contains information relevant to those who wish to use Seax in their projects. Information is provided on writing Seax bytecode, writing Scheme programs using the Seax Scheme compiler, using Seax as a library for embedded program execution, and Seax as a compilation target for compiler implementors.