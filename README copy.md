# Esoteric Reaction

Esoteric Reaction is a functional/stack-based esoteric programming language where code is chemical equations.

## Types

### Number

Numbers are a floating point number.
In most cases, only the integer part of the number is used.

### Lists

Lists are infinitely-nestable collections of elements.
Lists can contain lists, numbers, and code.

### Code

Like in Lisp dialects, code is data that can be freely manipulated like other types in the language.

## Equations

Equations are code in Esoteric Reaction.
Equations consist of sequences of terms, which contain elements or molecules.
Each element represents a function call.

Equations also have two operators: `=` and `->`.
`=` defines equations, and `->` calls equations.

## Terms

Terms are groupings of elements separated by `+`. Each individual term represents a composition of functions.
`ABC + D` represents in Lisp-like pseudocode `(A (B (C))) (D)`.

## Elements

Elements are the elemental building block of Esoteric Reaction, representing a function call. Each element has the following three parts:

* Optional coefficient
* Chemical symbol
* Optional subscript

Coefficients are syntatic sugar; it represents multiple calls to the function with the same arguments.
So, `3H` desugars into `H + H + H`.

Subscripts are a way of differentiating between related functions.

## Element Groups

Functions can be assigned a name via the `=` operator.
