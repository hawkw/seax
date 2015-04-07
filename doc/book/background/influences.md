% Influences

> "If I have seen further, it is by standing on the shoulders of giants."
>
> &mdash; Sir Isaac Newton, letter to Robert Hooke, February 5, 1676

### The SECD Machine

Although Seax has a great deal of influences, both software and academic works, it owes its' entire existence to Peter J. Landin's visionary papers [The Next 700 Programming Languages](http://fsl.cs.illinois.edu/images/e/ef/P157-landin.pdf) and [The Mechanical Evaluation of Expressions](http://comjnl.oxfordjournals.org/content/6/4/308). 

In "The Next 700 Programming Languages", published in the _Communications of the ACM in 1966, Landin describes an unimplemented programming language he calles ISWIM, for 'If you See What I Mean'. While the Lisp-inspired ISWIM has never been implemented, it has been a major influence in the design of many modern functional languages.

In "The Mechanical Evaluation of Expressions", published in _Computer Journal_ in 1964, Landin provides the first description of the SECD abstract machine, which he would later use to define the operational semantics of ISWIM. The machine described here forms the basis of the virtual machine at the core of the Seax runtime environment.

Also of great interest is Olivier Danvey's much more recent [A Rational Deconstruction of Landin's SECD Machine](http://www.brics.dk/RS/03/33/), published in _Basic Research In Computer Science_ in 2003; and Brian Graham's 1989 paper [SECD: Design Issues](http://prism.ucalgary.ca/bitstream/1880/46590/2/1989-369-31.pdf) and 1992 book [The SECD Microprocessor: A Verification Case Study](http://www.amazon.com/The-SECD-Microprocessor-Verification-International/dp/0792392450), both of which concern the implementation of the SECD machine as a hardware device.

### Lambda the Ultimate CPU Architecture

While not directly related to the SECD machine, something would be seriously amiss if I neglected to mention Sussman and Steele's [Design of LISP-based Processors, or SCHEME: A Dielectric LISP, or Finite Memories Considered Harmful, or LAMBDA: The Ultimate Opcode](http://repository.readscheme.org/ftp/papers/ai-lab-pubs/AIM-514.pdf), MIT AI Memo No. 514. The very idea of functions as the core primitives of a hardware architecture can likely be traced back to this paper. All of Sussman and Steele's [similarly-named papers](http://library.readscheme.org/page1.html) are major sources of influence and motivation for Seax.

Also worthy of looking into are the MIT [Lisp Machines](http://en.wikipedia.org/wiki/Lisp_machine) built in the 1970s and 80s. These computers, which were designed to run Lisp programs language as efficiently as possible, used a stack-based CPU architecture called CADR, with many similarities to the SECD machine. The CADR computer architecture is described in [MIT AI Memo No. 528](ftp://publications.ai.mit.edu/ai-publications/pdf/AIM-528.pdf), by Thomas F. Knight, Jr., David Moon, Jack Holloway, and Guy L. Steele, Jr.

The Lisp machines were very far ahead of their time, with the distinction of being among the earliest computers to boast high-resolution bitmapped graphics, window-based graphical user interfaces, mice, and automatic memory management. While these beautiful computers are no longer manufactured, hopefully the Seax VM will help to 'keep the torch alive' for functional-programming-based computer architectures.