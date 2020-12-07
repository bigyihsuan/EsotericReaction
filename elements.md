# Elements

This file outlines the elements of Esoteric Reaction.

## Number Operations

Element  | Arg Types | Return Types | Notes
-|-|-|-
`O` | - | Num | Push 0.
`O_N` | - | Num | Push N.
`H` | List | Num | Push the sum of all elements and subelements in the list.
`H_N` | Num, ... | Num | Push the sum of the next N elements.
`S` | List | Num | Push the difference of all elements and subelements in the list, from left to right.
`S_N` | Num, ... | Num | Push the difference of the next N elements, from left to right.
`N` | Num | Num | Negate.
`N_N` | Num | Num | Negate the next N elements.
`Mg` | List | Num | Push the product of all elements and subelements in the list.
`Mg_N` | Num, ... | Num | Push the product of the next N elements.
`D` | List | Num | Push the quotient of all elements and subelements in the list, from left to right. (Deuterium)
`D_N` | Num, ... | Num | Push the quotient of the next N elements, from left to right. (Deuterium)

## List Operations

Element | Arg Types | Return Types | Notes
-|-|-|-
`Li` | - | List | Push an empty list.
`Li_N` | Any, ... | List | List-ify the next N elements into a list.
`La` | List | Any, ... | De-list-ify a list into its elements.
`Ac` | List, Any | List | Append an element to the list.
`Ac_N` | List, Any, ... | List | Append N elements to the list (in stack order).
`He` | List | Any, List | Push the head of a list and then the rest.
`He_N` | List | List, List | Push the first N elements of a list and then the rest.
`Ta` | List | Any, List | Push the tail of a list and then the rest.
`Ta_N` | List | List, List | Push the last N elements of a list and then the rest.

## Stack Operations

Element | Arg Types | Return Types | Notes
-|-|-|-
`Po` | - | - | Pop. Use `nPo` to discard `n` elements.
`Dy` | Any | Any, Any | Duplicate. Use `nDy` to duplicate an element `n` times.
`Sb` | Any a, Any b | Any b, Any a | Swap.
`Ra` | Any a, Any b, Any c | Any c, Any a, Any b | Rotate top 3 elements downwards.

## Higher-Order Functions

Element | Arg Types | Return Types | Notes
-|-|-|-
`Fl` | - | Fun | Function. Push the next term as code.
`Fl_N` | - | Fun | Function. Push the next N terms as code.
`In` | Fun | - | Interpret.
`F` | Fun, List | List | Filter. Applies f on all elements of l. Returns a list where the result of f is truthy.
`Fe` | Fun, List | List | For Each/Map. Returns a list with f applied on l.

## Control Flow Functions

Element | Arg Types | Return Types | Notes
-|-|-|-
`I` | Any cond, Fun then, Fun else | Fun | If-Then-Else. Returns the branch based on the truthiness of cond.

## I/O

I/O is handled by `light` and `heat`.

Element | Arg Types | Return Types | Notes
-|-|-|-
`light` | - | Num | Pushes 1 byte from STDIN.
`light_N` | - | List | Pushes N bytes from STDIN as a list.
`heat` | Any | - | Pops 1 element and prints to STDOUT.
`heat_N` | Any, N | - | Pops N elements and prints to STDOUT.
