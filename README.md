# Relox

compiler/interpreter for lox-ish Programming language 

## Current State

It's work in progress but you should be able to get this locally and run `cargo run` and do basic arithmetic, String comparisons and String appending

### Scanner

The idea of the scanner is to be reused on the AST interpreter and the bytecode compiler. At the moment is fast but it's 
too procedural for my taste. I might refactor it at some point with a combinators

### Interpreter

The objective of the interpreter might be to just the REPL. I might just use the bytecode VM 
though. Will see...

- [x] Scanner
- [x] AST Parser - Recursive Descente
- [x] Basic Eval Interpreter
- [ ] Statements
- [ ] Control Flow, Functions, Etc
- [ ] Classes and Inheritance (maybe, it's not trendy anymore)

### Bytecode VM

- [x] Scanner
- [x] Rudimentary VM
- [X] Single Pass Compiler - Pratt Parser 
- [X] Single Pass Compiler - Compiler 
- [ ] Handle Variables
- [ ] Functions
- [ ] Maybe a garbage collector, I'm not sure if it will necessary with rust, yet.
- [ ] Jit Compiler
- [ ] Own String and Float low level implementation - Maybe?
- [ ] More cool stuff 

