% The SECD Abstract Machine

At the core of the Seax runtime environment is an implementation of the SECD abstract machine. The SECD machine is so named for the four registers that make up the machine's architecture `$s`, the stack; `$e`, the environment stack; `$c`, the control stack; and `$d`, the dump stack. 

Each of these registers contains, at any given time, either a [`cons`](/cons-list.md) cell or the empty list (`nil`). These `car` parts of these `cons` cells may point to any of the [SVM memory cell types](svm.html#svm-primitive-data-types) --- either atoms, lists, or SVM instructions. The SECD machine treats each of these lists as a stack, dealing only with the head element of each list at any given time.

## The Stack

The `$s` (stack) register is used for temprary data storage. Not unlike the temporary registers in a register machine architecture, it stores data that is currently being manipulated. When data is loaded from the environment or from main memory, it is pushed to the stack; and function calls and instructions that require operands expect to find their arguments on the stack.

## The Control and Dump Stacks

The `%c` (control) and `$d` (dump) registers are responsible for control flow. The `$c` register plays a role similar to that of the program counter on other hardware architectures. It points to the head of a list of SVM instructions corresponding to the code of the program currently under execution. Since these instructions are stored in a list, the control stack may be permuted using the same list and stack operations as the program's data. To those readers who recall our discussion of Lisp's quality of [homoiconicity](why-lisp.html#homoiconicity), this concept should seem familiar. The SECD machine can, therefore, perfrom control-flow operations by modifying the contents of the control stack. 

For example, consider the case of function calls (a primary control-flow primitive in Lisp and other functional-programming languages). In the SECD machine, functions are stored as lists of SVM instructions and data, either on the control stack as anonymous functions or on the environment stack as functions bound to names. Function calls can then be performed qhite simply, by setting the `$c` register to point at the head `cons` cell of the list corresponding to the function being called. 

The `$d` register is used to store the current contents of the control stack so that they may be restored when control is returned to the calling function. When a function is called, the current machine state (i.e. the stack, control stack, and current environment) are bundled together into a list and pushed to the dump stack. When control is returned to the caller, its registers are restored from the dump. In this role, the `$d` register performs a portion of the functionality of the call stack in other architectures; restoring the call-site state from a stack frame after a function returns.

## The Environment Stack

The `$e` (environment) register contains named data. If the `$d` register can be said to perform half of the work commonly assigned to a call stack in other architectures (_viz._ storing the state at the call site and restoring it when a function returns), the environment stack performs the remainder of the call stack's labor: it is used to pass arguments to function calls.

The environment stack is structured as a list of lists. Each level in the list corresponds to the scope of a particular function, with the head of the stack containing the arguments passed to the function currently under execution. The names bound to those arguments are associated by the compiler or interpreter with pairs of integers corresponding to the level and index in the environment stack where that variable is stored. We can note that the environment stack is, therefore, the only one of the SECD machine's stacks which is commonly accessed through indexing rather than sequentially through stack operations.

In addition to storing function arguments, the environment stack also contains local variables to a scope. We should acknowledge that in the Lisp programming language, the `let` expression, which binds local variables within a scope, is semantically equivalent to a function call; it is simply syntactic sugar for an anonymous function. This subject will be discussed in greater detail later on. That the binding of variables local to a scope takes place on the environment stack also provides us with a limited form of stack-based memory management for these variables.