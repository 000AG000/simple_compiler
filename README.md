# Purpose 

This is a test project to learn using vi, get better in rust and thinking about interperters and compilers.

## Language definition

This language is based on the Loop-language used in the lecture "Formale Systeme".

PROG -> STATEMENT
STATEMENT -> "let" VARIABLE_NAME [ "=" [NUMBER | VARIABLE_NAME]] | VARIABLE_NAME "=" VARIABLE_NAME  ["+"|"-" [NUMBER | VARIABLE_NAME] ] | "LOOP " VARIABLE_NAME "DO" STATEMENT "END" | STATEMENT SEPERATOR STATEMENT | "print" VARIABLE_NAME
NUMBER = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
CHARACTER = "a"|"b"|"c"|"d⅛|"e⅛|"f"|"g"|"h"|"i"|"j"|"k"|"l"|"m"|"n"|"o"|"p"|"q"|"r"|"s"|"t"|"u"|"v"|"w"|"x"|"y"|"z"|"A"|"B"|"C"|"D"|"E"|"F"|"G"|"H"|"I"|"J"|"K"|"L"|"M"|"N"|"O"|"P"|"Q"|"R"|"S"|"T"|"U"|"V"|"W"|"X"|"Y"|"Z"
VARIABLE_NAME -> (CHARACTER|NUMBER)* 
SEPERATOR -> "\n" | ";"

## Lexer ideas

When interpeters need to interpret a huge amounts of code it would be unwise to have a lexer that is build inefficient and stores all the text as strings. However, in the first approach it would be good to keep it simple.