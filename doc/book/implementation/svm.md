% Seax Virtual Machine

## SVM Primitive Data Types

A cell in the Seax virtual machine's memory may consist of any of the following types:

+ **Atom**: any of the following SVM [atomic types](/api/seax_svm/cell/enum.Atom.html):
    * `UInt`: an unsigned machine-size integer
    * `SInt`: a signed machine-size integer
    * `Float`: a 64-bit double-precision floating-point number
    * `Char`: a UTF-8 character
+ **Instruction**: any of the Seax Virtual Machine [instructions](/api/seax_svm/cell/enum.Inst.html)
+ **List**: a [`cons` cell](cons-list.html#the-cons-cell) or the empty list (`nil`)