% Why SECD and Lisp? 

> "Lisp is Lego for adults."
> 
> &mdash; [Paul Graham](https://twitter.com/paulg/status/588511912331055104)

The SECD machine is deeply linked with the [Lisp](http://en.wikipedia.org/wiki/Lisp_(programming_language)) family of programming languages. Lisp is among the oldest programming languages still in use today; originating in 1958, it predates C by 14 years. Lisp's syntax is significantly different from most popular programming languages, and many programmers consider it arcane or confusing. 

Why, then, focus on Lisp and on an abstract machine inspired by Lisp, when other languages are newer and more popular?

In anthropology, the [Sapir-Whorf hypothesis](http://en.wikipedia.org/wiki/Linguistic_relativity) suggests that the syntactical structure of a human language effects the thought patterns of the individuals who speak and write it. While not all linguists and anthropologists agree that this is true of natural languages, I for one feel that it is certainly true of _programming_ languages. The syntax of a programming language shapes the cognitive structures of programmers using that language, and teaches its own ways of solving programming problems.

<span id="homoiconicity">A major quality that sets Lisps apart from most other languages is that they exhibits a property known as _[homoiconicity](http://en.wikipedia.org/wiki/Homoiconicity)_.</span> Homoiconicity means that the structure of the source code of a Lisp program is identical with the structure of the abstract syntax tree (AST) produced for that program by a Lisp compiler. This means, therefore, that the code of a Lisp program may be treated as data by that program; making self-modification (reflection) simple. 

It is interesting to note that this same idea of storing instructions and data in the same way is essentially the core of the [Von Neumann architecture](http://en.wikipedia.org/wiki/Von_Neumann_architecture) upon which all computers in use today are based. This is, I think, a very important observation. It implies a way of thinking about programs and their execution that helps the programmer to better understand the machine. If we suppose the Sapir-Whorf hypothesis mentioned earlier, we could perhaps come to the conclusion that programming in Lisp might make us better programmers.

I am most certainly not the first to make the claim that Lisp makes us better programmers. In his essay [_How To Become a Hacker_](http://www.catb.org/esr/faqs/hacker-howto.html), Eric S. Raymond tells us that "LISP is worth learning for a different reason — the profound enlightenment experience you will have when you finally get it. That experience will make you a better programmer for the rest of your days, even if you never actually use LISP itself a lot." The enlightenment Raymond mentions, I think, comes from the viewing of code as data and data as code, the idea that makes modern computing possible.

Another excellent perspective on the benefits of Lisp comes from the typographer Matthew Butterick, author of the books _Practical Typography_ and _Typography for Lawyers_. Butterick, not a programmer by trade, taught himself Racket (a member of the Scheme family of Lisps) in order to create a new typesetting system called [Pollen](http://pollenpub.com). His [explanation](http://practicaltypography.com/why-racket-why-lisp.html) of why he chose Lisp, from the perspective of a relative outsider to computer science, speaks well to the language's strengths and cognitive benefits.