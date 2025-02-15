# Pandi C Compiler

## TODOs
- Comments are not handled during lexing, because the preprocessor removes them
  - Buuut, if I lex the preprocessed file, the token locations will be all over the place
  - I'd need my own preprocessor anyways, so I'll have to write one
