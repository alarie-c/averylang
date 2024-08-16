# Avery Syntax
Avery syntax is designed to be familiar and promote type safety and ensure the deliberate and intentional manipulation of data.

**EVERYTHING HERE IS TENTATIVE AND TEMPORARY**

**Declaring Variables**
Variables (values) are immutable by default. To make one, declare the variable name, optionally a type, and it's value.
```
x = 10
x int = 10
```

To make a value mutable, put the `mut` keyword at the beginning of the declaration.
```
mut y = 5
mut y int 5
```

Re-assigning or mutating variables requires the reassignment operator (`->`).
```
mut i = 0
i -> 1
```

Variables that are not initialized with the `=` operator must have a type declared, otherwise there will be an `unknown identifier` error. The variable will then be initialized with the default value for the type.

Abstract types like classes, enums, etc. cannot be used in non-assigned initialization because they have no default value, unless otherwise implemented.
```
mut a int // Creates a mutable integer variable "a"
b str // Creates an immutable string value "b"
```

Know that despite being assigned a default value, these variables are not "assigned" and but have a value given to them via the assignment operator before being able to be read or mutated.
```
mut a int
$ a
```
*The above is invalid code. Variables that have not been assigned a value cannot be read*

```
mut a int
a -> 5
$ a
```
*The above code is valid and prints `5` upon execution*

