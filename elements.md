# Elements

This file outlines the elements of Esoteric Reaction.

```lisp
; A represents the n-th arg
; Element + args -> result
O -> 1
O_n -> n

H -> 0
H + A -> A1
H_n + A1 + A2 + ... + An -> A1 + A2 + ... + An ; add

S -> 0
S + A -> -A1
S_n + A1 + A2 + ... + An -> A1 + A2 + ... + An ; sub

Li -> []
Li_1 + A -> [A]
Li_n + A1 + A2 + ... + An -> [A1, A2, ... , An] ; list
```

Element | Name | Returns | Subscript `n` | Notes
-|-|-|-|-|-
`O` | One | 1 | `n` |
`H` | Add | 0 | `t1 + t2 + ... + tn` |
`S` | Sub | 0 | `t1 - t2 - ... - tn` |
`Li` | List | `[]` | `[t1, t2, ... , tn]` | Called with no args returns `[]`. Subscript `Li_1` allows 1-element lists.
`N` | Neg | `-arg` | `-t1, -t2, ... , -tn` | Can operate on lists, negating all elements in the list.
`Fe` | For Each | `[]` | `[f(t1), f(t2), ... , f(tn)]` | Applies the leftmost arg on all other args. `Fe_1 + arg -> arg`.

Element | Name | Reagent | Product | Subscript `n`
-|-|-|-|-
`light` | STDIO | Return 1 line of STDIN as a "string" | Print the result to STDOUT |
`heat` | Equation Pointer | Return the current equation pointer | Store the result to the equation pointer |

## Number Operations

Element  | Arg Types | Return Types | Notes
-|-|-|-
`O` | - | Num | Push 0.
`O_N` | - | Num, N | Push N.

## List Operations

Element | Arg Types | Return Types | Notes
-|-|-|-
`Li` | - | List | Push an empty list.
`Li_N` | - | List | Pop N times, push a list containing them.

## Stack Operations

Element | Arg Types | Return Types | Notes
-|-|-|-
`P` | Num n | - | Pop and discard n elements.

## Higher-Order Functions

Element | Arg Types | Return Types | Notes
-|-|-|-
`F` | Fun f, List l | List | Filter. Applies f on all elements of l. Returns a list where the result of f is truthy.
`Fe` | Fun f, List l | List | For Each. Returns a list with f applied on l.

## I/O

I/O is handled by `light` and `heat`.

Element | Arg Types | Return Types | Notes
-|-|-|-
`light` | - | Num | Pushes 1 byte from STDIN.
`light_N` | - | List | Pushes N bytes from STDIN as a list.
`heat` | Any | - | Pops 1 element and prints to STDOUT.
`heat_N` | Any, N | - | Pops N elements and prints to STDOUT.
