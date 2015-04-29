% The SECD Abstract Machine

At the core of the Seax runtime environment is an implementation of the SECD abstract machine. The SECD machine is so named for the four registers that make up the machine's architecture `$s`, the stack; `$e`, the environment stack; `$c`, the control stack; and `$d`, the dump stack. 

Each of these registers contains, at any given time, either a [`cons`](/cons-list.md) cell or the empty list (`nil`). These `car` parts of these `cons` cells may point to any of the [SVM memory cell types](svm.html#svm-primitive-data-types) --- either atoms, lists, or SVM instructions. The SECD machine treats each of these lists as a stack, dealing only with the head element of each list at any given time.

## The Stack

The `$s` (stack) register is used for temprary data storage. Not unlike the temporary registers in a register machine architecture, it stores data that is currently being manipulated. When data is loaded from the environment or from main memory, it is pushed to the stack; and function calls and instructions that require operands expect to find their arguments on the stack.

## The Control and Dump Stacks

The `%c` (control) and `$d` (dump) registers are responsible for control flow. The `$c` register plays a role similar to that of the program counter on other hardware architectures. It points to the head of a list of SVM instructions corresponding to the code of the program currently under execution. Since these instructions are stored in a list, the control stack may be permuted using the same list and stack operations as the program's data. To those readers who recall our discussion of Lisp's quality of [homoiconicity](why-lisp.html#homoiconicity), this concept should seem familiar. The SECD machine can, therefore, perfrom control-flow operations by modifying the contents of the control stack. 

For example, consider the case of function calls (a primary control-flow primitive in Lisp and other functional-programming languages). In the SECD machine, functions are stored as lists of SVM instructions and data, either on the control stack as anonymous functions or on the environment stack as functions bound to names. Function calls can then be performed qhite simply, by setting the `$c` register to point at the head `cons` cell of the list corresponding to the function being called. 

The `$d` register is used to store the current contents of the control stack so that they may be restored when control is returned to the calling function. When a function is called, the current machine state (i.e. the stack, control stack, and current environment) are bundled together into a list and pushed to the dump stack. When control is returned to the caller, its registers are restored from the dump.
