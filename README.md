# Atomic

An esolang using chemical "formulas" as code. Functional stack-based language, aka instructions can be data and be pushed onto the stack.

A single atomic element (C, O, U, Fe, etc) represents an operation. A numbered element (`H_2`, `H_2O`, `C0_2`) represents a different operation, based on its preceeding element. A numbered molecule (`3H`,`3H_2O`, etc) represents that molecule operating that number of times.

Yes, imbalanced and impossible equations are possible.

```
reagents [eq] products
A + B -> C + heat
```

Each line of equations applies the operation in the direction of the arrow. Whatever value is on 

`A + B` represents the two input arguments `A` and `B`.

`[eq]` can be one of four symobls:

* `=` for assignment and definition
* `->` for one-way equation
* `<->` for two-way equation
* `<=>` for equillibrium equation

Parentheses signifies multi-element names. `A(BC)` is `A` and `BC`.

Comments are signified by a semicolon `; Comment`.

Heat and light are "side" effects. Light is the function IO. As a reagent it is a list consisting of the function arguments. As a product, it is a list containing all output values.
Heat represents STDIO. As a reagent, it is a list of characters of a single line of STDIN. As a 