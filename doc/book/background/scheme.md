% The Scheme Programming Language

Scheme is one of the most commonly used members of the Lisp family, rivalling Common Lisp in popularity. It dates back to the 1970s, and was initially designed by Guy L. Steele and Gerald Sussman, two great figures in the history of programming languages.

## Minimalism

Scheme is fairly minimalist in design. It provides relatively few primitive instructions, but is designed in order to support programmers extending its syntax easily. Both the beauty and the practical advantages of this design philosophy are communicated quite powerfully by Steele himself in his 1998 talk ["Growing a Language"](https://www.youtube.com/watch?v=_ahvzDzKdB0), which is in this author's opinion the single most important and powerful lecture in the history of the study of programming languages. A Scheme interpreter or compiler defines very few of the language's fundamental forms, as a majority of them may be implemented in Scheme as 'library forms' using the language's other constructs.

## Scope 

Scheme is a [_lexically scoped_](http://en.wikipedia.org/wiki/Scope_(computer_science)#Lexical_scoping_vs._dynamic_scoping)) language. While lexical scoping is now very common, Scheme was one of the first such languages. This means that the scope of a Scheme program may be determined through analyzing the program's source text, rather than at run-time as in _dynamic scoping_. In Scheme, functions (either `lambda` expressions or named function definitions) and binding constructs (e.g. `let`, `letrec`, etc.) correspond to scopes.

## Functional Programming

Scheme is a functional programming language. It is, in fact, one of the first such languages, and was repsonsible for popularizing a number of important functional programming concepts, such as [anonymous functions](http://en.wikipedia.org/wiki/Anonymous_function) (in the form of `lambda` expressions), [continuation-passing style](http://en.wikipedia.org/wiki/Continuation), and the use of tail recursion to express repeated computations or loops.