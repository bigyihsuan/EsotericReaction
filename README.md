# Esoteric Reaction

Esoteric Reaction is a functional/stack-based/concatenative esoteric programming language where code is chemical equations. 

Intepreter guided by [Crafting Interpreters](https://craftinginterpreters.com/).

## Types

### Number

Numbers are a floating point number.
In most cases, only the integer part of the number is used.

### Lists

Lists are infinitely-nestable collections of elements.
Lists can contain lists, numbers, and code.

### Code

Like in Lisp dialects, code is data that can be freely manipulated like other types in the language.

### Truthiness

In Esoteric Reaction, truthiness for logical operators is defined as follows:

Truthy | Falsy
-|-
Non-zero | Zero
Filled list | Empty list
Code | -

## Equations

Equations are code in Esoteric Reaction.
Equations consist of sequences of terms, which contain elements or molecules.
Each element represents a function call that takes and returns a stack.

Equations also have two operators.
`=` defines groups and terms of elements, and `->` executes terms.
`->` equations require both sides to be balanced to follow the Law of Conservation of Mass.

```
<code>	   ::= <equation> | <equation> <code>
<equation> ::= <reagent> <rhs> | <COMMENT> | <reagent> <rhs> <COMMENT>
<rhs>      ::= "=" <NAME> | "->" <reagent>
<reagent>  ::= <term> | <term> "+" <reagent>
<term>     ::= <molecule> | <NUMBER> <molecule>
<molecule> ::= <atom> | <atom> <molecule>
<atom>     ::= <ELEMENT> | <name> "_" <NUMBER>
<NUMBER>   ::= integer > 0
<ELEMENT>  ::= valid element
<NAME>     ::= non-whitespace, non-"_+;" string
<COMMENT>  ::= ";" text "\n"
```

```
<code>		 -> <equation>*
<equation>	 -> <assigment> | <execution>
<assignment> -> <reagent> "=" <NAME>
<execution>	 -> <reagent> "->" <reagent>
<reagent>	 -> <term> ("+" <term>)*
<term>		 -> <COEFF>? <atom>+
<atom>		 -> <element> ("_" <COEFF>)?
<element>	 -> element | "(" name ")"
```

## Terms

Terms are groupings of elements separated by `+`. Each individual term represents a composition of functions.

So, `ABC + D` represents in Lisp-like pseudocode `(A (B (C))) (D)`.

## Elements

Elements are the elemental building block of Esoteric Reaction, representing a function call. Each element has the following three parts:

* Optional coefficient
* Chemical symbol
* Optional subscript

### Element Groups

Functions can be assigned an arbitrary name via the `=` operator. They can then use this name to call that snippet of code later. Names must be surrounded by parentheses (as they are a grouping of elements), and can have any non-whitespace, non-`+_()` string of characters.

```lisp
Co + De = (CoDe) ; definition
(CoDe) -> CoDe ; call
3 (CoDe) + H -> 2 CoDe + CoDeH ; call with coefficent
(CoDe)_3 + H -> 3 CoDe + H ; call with subscript
```

## Modifiers

Elements and element groups can be 

### Coefficients

Coefficients are syntatic sugar; it represents multiple calls to the function with the same arguments.
So, `3 H` desugars into `H + H + H`.
`3 (CNH)` desugars into `CNH + CNH + CNH`.

### Subscripts

Subscripts are a way of differentiating between related functions. Functions without a subscript act upon individual elements on the stack, while subscripted elements take an additional number argument `n` first to act upon `n` elements of the stack.

For example, `H` pops twice and pushes the sum.
`H_x` pops once to get the number of elements to sum, then pops that many times, then pushes the sum of them all.

Subscripting an element group does the same, where it pops a number `n` and then pops an additional `n` times to pass as arguments to the group.
