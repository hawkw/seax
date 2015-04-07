% Seax

Seax is a multi-language runtime environment for executing computer programs, with a focus on functional languages. At the core of Seax is a virtual machine based on an implementation of the [SECD machine](https://en.wikipedia.org/wiki/SECD_machine) first described by Peter J. Landin. 

Seax was implemented by [Hawk Weisman](http://hawkweisman.me) and is released under the [MIT license](https://github.com/hawkw/seax/blob/master/LICENSE). While Seax was written primarily for educational purposes, the goal of the project is to create a runtime environment which may be used by developers for real-world projects.

Do note that Seax is currently undergoing active development, and therefore, a number of planned features may currently be incomplete or wholly unimplemented.

The Seax documentation is split into three primary sections, which can be navigated using the table of contents on the left of this page.

## Background

This section contains information regarding the motivation and goals behind Seax, and the antecedants and previous work that have informed its' implementation.

## Implementation

This section details the implementation of Seax and its components, including descriptions of their architecture and functionality. Discussions of the design choices that went into the Seax's implementation are also furnished.

## Reference

This section contains information relevant to those who wish to use Seax in their projects. Information is provided on writing Seax bytecode, writing Scheme programs using the Seax Scheme compiler, using Seax as a library for embedded program execution, and Seax as a compilation target for compiler implementors.