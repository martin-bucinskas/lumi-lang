grammar asm;

// Lexer rules
ALPHA: [a-zA-Z]+;
SEMICOLON: ';';
NEWLINE: '\r'? '\n';
WS: [ \t\r\n]+ -> skip; // skip whitespace

// Parser rules
opcode: ALPHA+;
comment: SEMICOLON .*? NEWLINE?;
