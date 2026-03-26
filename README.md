# Purpose 

This is a test project to learn using vi, get better in rust and thinking about interperters and compilers.

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
    TERM (("+" | "-") TERM)*

TERM ->
    NUMBER | IDENT

SEPERATOR -> ("\n" | ";")



## Lexer ideas

When interpeters need to interpret a huge amounts of code it would be unwise to have a lexer that is build inefficient and stores all the text as strings. However, in the first approach it would be good to keep it simple.

## Parser ideas

Make the language simple so you only need a minimal number of peeks forward