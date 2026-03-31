# Purpose 

This is a test project to learn using vim, get better in rust and thinking about interpreters compilers.

## Language definition

This language is based on the Loop-language used in the lecture "Formale Systeme".

PROGRAM     -> STATEMENT_LIST

STATEMENT_LIST -> STATEMENT (SEPARATOR STATEMENT)*

STATEMENT ->
    LET IDENT ("=" EXPRESSION)?
  | IDENT "=" EXPRESSION
  | LOOP IDENT DO STATEMENT_LIST END // Loop does the STATEMENT_LIST the number of times the identifier evaluates to
  | PRINT IDENT
  | EMPTY

EXPRESSION ->
    TERM (("+" | "-") TERM)* // will be calculated from left to right saturated subtraction

TERM ->
    NUMBER | IDENT

SEPARATOR -> ("\n" | ";")


Nodes:
Variables are only unsized integers
Variables have to be declared with let before using them
This is ASCII only


## Build

```bash
cargo build --release
```

## How to use
```bash
simple_interpreter -p path_to_file
```

with logging:
```bash
RUST_LOG=debug simple_interpreter -p path_to_file 
```


## Example file

A file cloud look like this:
```
let x = 3;
LOOP x DO
    print x;
END
```

The output is the following

3
3
3

## Project structure

The projects consists of four parts:
- main cli tool for basic usage
- lexer
- parser
- interpreter

### Lexer

Simple Lexer that only accepts,get_token_kind_clone ASCII as input.

### Parser 

Recursion based parser. The Parser includes also the little semantic analysis needed for this easy language definition.

### Interpreter

It became a runtime execution working with a stack of execution frames.