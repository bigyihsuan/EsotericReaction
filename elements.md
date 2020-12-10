# Elements

This file outlines the elements of Esoteric Reaction.

## Number Operations

Element  | Arg Types | Return Types | Notes
-|-|-|-
`O` | - | Num | Push 0.
`H` | Num, Num | Num | Addition
`S` | Num, Num | Num | Subtraction.
`F` | Num, Num | Num | Multiplication.
`D` | Num, Num | Num | Division. (Deuterium)
`N` | Num, Num | Num | Negation.
`P` | Num, Num | Num | Exponentiation.
`Cl` | Num | Num | Increment.
`Br` | Num | Num | Decrement.

## Logical Operators

Logical operators pop 2 elements and returns either 0 (falsy) or 1 (truthy).
For `He`, `Ar`, `Kr`, `Xe`, `Rn`, always returns 0 if the 2 elements are of different types, while `Ne` always returns 1.

Element  | Arg Types | Return Types | Notes
-|-|-|-
`He` | Any a, Any b | Num | `a == b`
`Ne` | Any a, Any b | Num | `a != b`
`Ar` | Any a, Any b | Num | `a > b`
`Kr` | Any a, Any b | Num | `a >= b`
`Xe` | Any a, Any b | Num | `a < b`
`Rn` | Any a, Any b | Num | `a <= b`

## List Operations

Element | Arg Types | Return Types | Notes
-|-|-|-
`Li` | - | List | Push an empty list.
`La` | List | Any, ... | De-list-ify a list into its elements.
`Ac` | List, Any | List | Append an element to the list.
`He` | List | Any, List | Push the head of a list and then the rest.
`Ta` | List | Any, List | Push the tail of a list and then the rest.

## Stack Operations

Element | Arg Types | Return Types | Notes
-|-|-|-
`Po` | Any | - | Pop.
`Dy` | Any | Any, Any | Duplicate.
`Sb` | Any a, Any b | Any b, Any a | Swap.
`Ra` | Any a, Any b, Any c | Any c, Any a, Any b | Rotate top 3 elements upwards.
`Rb` | Any a, Any b, Any c | Any b, Any c, Any a | Rotate top 3 elements downwards.
`Si` | - | Num | Size of the stack.

## Higher-Order Functions

Element | Arg Types | Return Types | Notes
-|-|-|-
`Fl` | - | Fun | Function. Push the next term as code.
`In` | Fun | - | Interpret.
`F` | Fun, List | List | Filter. Applies f on all elements of l. Returns a list where the result of f is truthy.
`Fe` | Fun, List | List | For Each/Map. Returns a list with f applied on l.
`Re` | Fun f, Any a, List l | Num | Reduce left-to-right, starting at a on l.
`Rf` | Fun f, Any a, List l | Num | Reduce right-to-left, starting at a on l.

## Control Flow Functions

Element | Arg Types | Return Types | Notes
-|-|-|-
`I` | Any cond, Fun then, Fun else | Fun | If-Then-Else. Returns code based on the truthiness of cond. then if truthy, else if falsy.

## I/O

I/O is handled by `light` and `heat`. Light works with bytes, heat works with stack elements.

Element | Arg Types | Return Types | Notes
-|-|-|-
`light` | - | Num | Push 1 byte from STDIN.
`heat` | Any | - | Pop and print.
