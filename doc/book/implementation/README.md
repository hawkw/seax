% Implementation

Seax consists of two core components: the Seax Virtual Machine (SVM) and the the Seax Scheme compiler. In order to allow Seax to be used embedded in other projects, both of these components have been written as libraries rather than as applications. Therefore, a simple driver application to allow users to invoke Seax functionality from the command line must also be provided.