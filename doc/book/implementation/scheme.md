% Seax Scheme

The Seax Scheme [crate](http://hawkweisman.me/seax/api/seax_scheme/index.html) contains a library for generating SVM instructions from source code written in the Scheme programming language, [as discussed previously](background/scheme.md). 

The Scheme langauge is described fully in the [Revised<sup>6</sup> Report on the Algorithmic Language Scheme](http://www.r6rs.org), or R<sup>6</sup>RS. The Seax Scheme compiler aims to be a fully R<sup>6</sup>RS-compliant implementation of Scheme. However, it is important to note that, as Seax is still undergoing active development, a number of language features described in R<sup>6</sup>RS are not currently implemented. Instead, complete R<sup>6</sup>RS-compliance is the goal which will mark Seax Scheme as being feature-complete.

## Parsing

The first step in any compiler or interpreter is to parse the source code and convert it from text into a format understnadable by the compiler. The Seax Scheme parser does this using a technique called [combinator parsing](http://en.wikipedia.org/wiki/Parser_combinator), a method of implementing recursive-descent parsers based on the functional programming concept of higher-order functions.

Unlike other commonly used methods of implementing compiler front-ends, combinator parsing has the significant advantage of not requiring the use of code-generation software such as `yacc` or [`bison`](https://www.gnu.org/software/bison/). Instead, parsers may be written 'by hand', with relative ease.

How does this approach work? In combinator parsing, a _parser_ is defined as any function which takes a character or string of characters as input, and accepts or rejects that string as being a symbol of a certain type. A _parser combinator_ is then a higher-order function that takes as input two or more parsers and returns a third parser function that combines the input parsers according to some rule. Common parser combinators include a repetition combinator that parses one or more repetitions of a character or string, a disjunction combinator which parses either one character or string or another, and a sequential combinator that parses one character or string followed by another. With a robust set of combinators, parsers for a wide variety of languages may be implemented with relative simplicity. 

Seax Scheme utilizes a Rust parser-combinators [library](https://github.com/Marwes/parser-combinators) created by Markus Westerlind for this purpose. Readers interested in further details of how the Seax Scheme parser operates may wish to consult the [API documentation](http://hawkweisman.me/seax/api/seax_scheme/parser/index.html) for the parsing module.

## Analysis and Compilation

The Scheme parser outputs an [abstract syntax tree](http://hawkweisman.me/seax/api/seax_scheme/ast/index.html), or AST, representing the structure of the program. This AST consists of a series of [nodes](http://hawkweisman.me/seax/api/seax_scheme/ast/trait.ASTNode.html) of different types representing each potential expression in a Seax Scheme program. Each of these nodes must implement a [`compile`](http://hawkweisman.me/seax/api/seax_scheme/ast/trait.ASTNode.html#tymethod.compile) method for transforming the node into a list of SVM instructions. The compile methods for AST nodes with children typically call the compile methods of those nodes as appropriate, descending through the tree to the leaf nodes.

To assist in compilation, a [symbol table](http://hawkweisman.me/seax/api/seax_scheme/ast/type.SymTable.html) is passed from each parent node to its child nodes. This table associates the names of symbols with the ordered pairs of indices into the VM's [environment stack](secd.html#the-environment-stack) to which those names are bound. 

This symbol table is represented through the use of a data structure called a [`ForkTable`](http://hawkweisman.me/seax/api/seax_scheme/struct.ForkTable.html), which functions similarly to a standard associative map data structure (such as a `HashMap`), but with the added ability to fork children off of each level of the map. If a key exists in any of a child's parents, the child will 'pass through' that key. If a new value is bound to a key in a child level, that child will overwrite the previous entry with the new one, but the previous key &rarr; value mapping will remain in the level it is defined. This means that the parent level will still provide the previous value for that key. This allows the `ForkTable` to represent scoped associative mappings with name shadowing, such as the scopes in a Seax Scheme program. The `ForkTable` implementation used in Seax Scheme is based on a [Scala implementation](https://github.com/hawkw/decaf/blob/master/src/main/scala/com/meteorcode/common/ForkTable.scala) written by the author and his colleague [Max Clive](http://arcticlight.me) for a compiler for the Decaf programming language. 

## Optimization

The Seax Scheme compiler does not currently support any signficant compile-time optimizations, as these features are planned for future Seax releases. When compile-time optimizations are added, they will likely be implemented in a similar form to the current compilation method, by adding to the trait for AST nodes an additional method with the signature `fn optimize(self) -> ASTNode`. This would allow AST nodes to transform themselves to optimized representations during a similar descent as the `compile` method. The transformed tree could then be compiled.

