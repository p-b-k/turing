Math Fun Project
================

Including a Turning Machine and some Lambda Caclulus tools.

Building
------------------------------------------------
Standard rust build from root.  
`$ cargo build` puts the built files in target/debug.  

Running
------------------------------------------------
There are a few different executables in the debug folder, some of them for turning machines and some
of them for lambda caclulus.

**Turning Machine Executables**

1. **symbolic**
Creates an accept function languages, based on a logic file. Read stdin and determines if the input
is a member of the defined lanaguage

2. **tcalc**
Takes two integers as parameters and performs an arithmatic operation on them and prints the result.
The default operation is addition (adder.tm), but you can also specify other operations with the
`--logic` flag.

Currently only mulitiplication (multiplier.tm) and addition (adder.tm) are supported.

**Lambda Calculus Executables**
1. **lcacl** Simple too for performing operations on lambda expressions.
Currently only pretty-prints the expression but more is coming soon.

2. **lambda-shell** A repl for working with lambda expressions.
