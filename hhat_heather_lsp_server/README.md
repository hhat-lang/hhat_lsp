# H-hat's Heather dialect LSP server

This directory holds the crate of LSP (language server protocol) server for the Heather dialect. It is implemented in Rust and should be good enough to LSP clients to use it, for any other language implementation or IDE/code editor that supports LSP.


## Code organization


Parser folder contains:
- Grammar: the grammar for each kind of file
- AST
- Lexer
- Parser
- 