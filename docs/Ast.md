# Abstracy Syntax Tree
This is an ongoing compendium to see what the structure of the AST is going to look like over time.

**Abstract Syntax Tree:**
```
Node Types {
    BinaryExpr(
        BinaryExpr {
            op: BinaryOperator
            lhs: Node
            rhs: Node
        }
    )
    
    Number{
        value: i32
    }
}

```