# Literals

8/10/2024
~~Number and string literals are weird because they aren't separated into number or string until parsing even though string literals are so clearly marked with quotations.~~

Sike! I'm splitting up number and string literals in the lexer because its just so much easier.

Same stuff from before still applies...

String literals have to keep their quotations lest it fuck with the text spans and where the string literal actually is.
Number literals do not have to do this, however I still have to include the periods and underscores for similar reasons.
* Underscores so the previous issue doesn't happen and textspans are still accurate
* Periods so that floating point numbers can be accurately parsed

But because symbols/identifiers are not allowed to start with numbers, number literals must be their own thing. Anything that starts with a number will be tokenized as a number literal. This allows for some more options during parsing.

For example, I may choose to add ascii alphanumerics to be pushed to the buffer when taking number literals, allowing for unique number literal related syntax like `0usize` or `1_u32` in Rust. Another example is `float x = 3.14f` in C++.

**In summary**
Literals area bunch of bullshit with too many exceptions...
I will eventually have to add escape character support as well, I just realized.