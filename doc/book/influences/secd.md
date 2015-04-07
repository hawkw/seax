% Landin's SECD Machine

Although Seax has a great deal of influences, both software and academic works, it owes its' entire existence to Peter J. Landin's visionary papers [The Next 700 Programming Languages](http://fsl.cs.illinois.edu/images/e/ef/P157-landin.pdf) and [The Mechanical Evaluation of Expressions](http://comjnl.oxfordjournals.org/content/6/4/308). 

In "The Next 700 Programming Languages", published in the _Communications of the ACM in 1966, Landin describes an unimplemented programming language he calles ISWIM, for 'If you See What I Mean'. While the Lisp-inspired ISWIM has never been implemented, it has been a major influence in the design of many modern functional languages.

In "The Mechanical Evaluation of Expressions", published in _Computer Journal_ in 1964, Landin provides the first description of the SECD abstract machine, which he would later use to define the operational semantics of ISWIM. The machine described here forms the basis of the virtual machine at the core of the Seax runtime environment.

Also of great interest is Olivier Danvey's much more recent [A Rational Deconstruction of Landin's SECD Machine](http://www.brics.dk/RS/03/33/), published in _Basic Research In Computer Science_ in 2003; and Brian Graham's 1989 paper [SECD: Design Issues](http://prism.ucalgary.ca/bitstream/1880/46590/2/1989-369-31.pdf) and 1992 book [The SECD Microprocessor: A Verification Case Study](http://www.amazon.com/The-SECD-Microprocessor-Verification-International/dp/0792392450), both of which concern the implementation of the SECD machine as a hardware device.