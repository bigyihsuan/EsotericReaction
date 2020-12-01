# Esoteric Reaction

An esolang using chemical "formulas" as code. The name is a pun on "exothermic reaction".

Yes, imbalanced and impossible equations are possible.

## Types

Integer: bog-standard 64-bit signed integer.

* Floating point numbers are represented by taking their IEEE 754 double-precision (64-bits long) binary representation and interpreting it as an integer.

List: an infinitely-nestable heterogenous list that can hold values.

* Lists consisting of solely integer values can be considered as Strings. Each integer represents a character's value in UTF-8.

## Elements

Each of the 118 elements of the periodic table represents a different function/instruction.

Coefficients indicate that element being repeated several times. So `2O == O + O`.

Subscripts change the number of arguments expected for the function. So `Li_3` expects 3 arguments following it.

Strings of consecutive elements (`H_20`, `NLi_4`, etc) represents function composition from right to left. So, `ABC` is `C(B(A()))`.

### Light and Heat

`light` and `heat` are special elements. `light` handles STDIO, and `heat` handles the equation pointer.

`light` on the reagent side returns a single line of STDIN. On the product side, `light` prints the result of the reagents to STDOUT.

`heat` as an reagent returns the value of the equation pointer, i.e. the line of code that is currently executing. As a product the result of the reagents gets stored in `heat` (the equation pointer changes).

## Equations

Equations are always of the following grammar:

```
<equation> = <term> <eq>
<eq> = '=' <name> | '->' <term>
<name> = STRING, not `light` or `heat`
<term> = <molecule> '+' <term> | <molecule>
<molecule> = <num> <submolecule> | <atomGroup>
<atomGroup> = '(' <submolecule> ')'
<submolecule> = <atom><atom> | <atom> | 'light' | 'heat'
<atom> = <element> | <element> '_' <sub>
<num> = [2-9] | <sub>
<sub> = [1-9][0-9]*
<element> = any element on the periodic table
```

Comments are signified by a semicolon `; comment up to the end of the line`.

Equations are chemicals separated by an equation symbol.

The equation symbol can be one of the following two symbols:

* `=`: Bind a name (product) to functions (reagents). The name consists of all the elements expanded and concatenated together.
  * For example, `[lhs] = 2H + CO_2` stores `[lhs]` into the variable `HHCOO`.
* `->`: The result of the reagents is stored in the memory location indexed by the result of the product.
  * If the product results in a list, the result of the reagents is stored in each location indexed by each element of the list.

Molecules are separated by `+` signs. Coefficients are just syntactic sugar for multiple calls to the same function, so `H_2 + 20` desugars into `H_2 + O + O`, and `Li_4 + 4light` into `Li_4 + light + light + light + light`.

## Example Equations

**Cat**

```
light -> light
```
