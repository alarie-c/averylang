# Avery Programming Language
This is a language I'm using primarily to learn about compilers in Rust. The goal is to make a language that is inherently suited to data science, data analysis, data visualization, statistics, etc.

**Timeline**
 - 8/8 - redo everything :D
 - 8/10 - basic lexer is more or less done
 - 1/16 - lexer done, parsing started and somewhat functional

**Major Projects**
- [x] Lexer 8/16
- [ ] Parser

## Lexer
Major Goals:
1. Should have minimal overhead
2. Should have simple and easy control flow
3. Match blocks are abstracted
4. Methods and helper functions return similar Option types
5. Elegant handling of EOF condition checks

## Parser
Major Goals:
1. Understandable control flow
2. Abstracted functions for parsing different kinds of statements
3. Simple and abstracted error handling