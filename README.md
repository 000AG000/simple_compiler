# Purpose 

This is a test project to learn using vim, get better in rust and thinking about interperters and compilers.

## Language definition

This language is based on the Loop-language used in the lecture "Formale Systeme".

PROGRAM     -> STATEMENT_LIST

STATEMENT_LIST -> STATEMENT (SEPARATOR STATEMENT)*

STATEMENT ->
    LET IDENT ("=" EXPRESSION)?
  | IDENT "=" EXPRESSION
  | LOOP IDENT DO STATEMENT_LIST END
  | PRINT IDENT
  | EMPTY

EXPRESSION ->
    TERM (("+" | "-") TERM)* // will be calculated from left to right

TERM ->
    NUMBER | IDENT

SEPERATOR -> ("\n" | ";")

## How to use
```bash
simple_interpreter -p path_to_file
```

## Project structure

The projects consists of four parts:
- main cli tool for basic usage
- lexer
- paser
- interpreter

### Lexer

Simple Lexer that only accepting ASCII as input.

### Parser 

Recursion based parser. The Parser includes also the little sementic analysis needed for this easy language definition.

### Interpreter

It became a runtime execution working with a stack of execution frames.