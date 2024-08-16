# Parser
The parser is inspired by some articles on AST generation and Pratt parising I've seen. It's a huge amalgamation of recursive functions.

Because of all of this goofing around is happening a bunch of methods keep messing about with the the current token we're matching at any given time.

Thus, the methods that call on other things (e.g. the controlling methods like parse_binary_expr) that are higher level than some of the smaller, more irrelevant ones, set an offset in the parser.

The parser has a member `offset` to denote how far it needs to jump ahead when done parsing an expression. This is so we can skip about freely without having to worry about keeping track of where and when and how we need to update the index. Helper methods never (never!) update the `idx`, they only update `self.current` using operations on `idx` in the `self.src.get()` method call. This means `idx` stays the same no matter what until it is mutated by `parse(&mut self)`

### Example below:
```
Tokens {
    Number(2)
    Plus,
    Number(4)
}
```
And the parser does:
```
Number(2) => Do nothing, don't create leaf nodes without a branch node first
Plus => Create a binary expression {
    Go back 1 to find a leaf node. Expect a number (otherwise crash, lol!)
    Go forward 1 to find a lead node. Expect a number (otherwise crash again)
    Return the binary expression
} 
Number(4) => Lead node, don't care, skip to the next one
EOF => Do nothin and break the parse loop because skipping will put us out of bounds
```
