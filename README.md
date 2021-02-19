# Will this amount to anything at any point? Probably not

Currently studying bad READMEs and getting pretty good at it

Also kinda studying compilers so I guess thats what this is about.

## Program representation

First step is to convert the code to a sequence of assembly-esque instructions.

These sequences can be converted to higher level representations in order to be useful.

One such representation is a __CFG (Control flow graph)__
A CFG represents a program as a graph of instructions or in a more optimal form, a graph of basic blocks.
A basic block is a sequence of instructions where executing the first instructions means executing all instructions.
An edge in a CFG is a transition between 2 basic blocks, the basic blocks themselves are the nodes of the graph. There also exists
an entry and an exit block. The graph can be represented as a dictionary mapping labels of the blocks to its successors.

### Eliminating dead code through CFGs
Nodes that are not the entry block and are no successors of any other nodes are dead code. Through repeatedly applying this check and removing
found blocks a very basic dead code elimination can be done. This of course only eliminates code unreached by the flow of the program.
