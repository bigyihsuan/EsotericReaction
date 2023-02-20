# Esoteric Reaction

Esoteric Reaction is a functional/stack-based/concatenative esoteric programming language where code is chemical equations.

## Goals

* Practice programming in Rust.
* Write an esolang that looks like chemical equations.

## Programs

### Quick Examples

#### "Hello World!"

```lisp
; starting with alkane with a single carbon
HS"Hello, World!" + Xn -> XnSH
```

### Law of Conservation of Mass

All Esoteric Reaction programs must follow the Law of Conservation of Mass:
the number of atoms per element on the left side of the `->`
must be equal to the number of atoms per element on the right side.

Note that this is conservation of *mass*, and not energy.

### Elements

All 118 known elements are reserved for built-ins.

#### Undiscovered Elements

All elements starting from atomic number 119 and beyond are available as bindable names.
The rules for what is considered an undiscovered element is as follows:

* The first character is uppercase, while all other characters are lowercase.
* When converting to an atomic number `x`, satisfies `x >= 119`.
* Only contains the following characters (from 0-9):
  * `nubtqphsoe`

To make a name of atomic number `N`, perform the following pseudocode:

```python
for each digit,
  map it to its letter value:
    n -> 0, u -> 1, b -> 2, t -> 3, q -> 4,
    p -> 5, h -> 6, s -> 7, o -> 8, e -> 9
capitalize the first letter.
```

#### Light and Heat

### Coefficients

### Subscripts

### Molecules

### Equations

### Naming Equations

```lisp
Uue: ... ; equation here
; usage
ABC + Uue -> AUue + BC ; use like any other element
```

## The Virtual Reaction Machine

The Virtual Reaction Machine is a virtual machine
containing a singular alkane tape (acyclic chain/tree hydrocarbon).
The alkane is manipulated upon by the Esoteric Reaction program,
and can be traversed "up" and "down" the carbons.
Moving off the alkane is considered as a crash.
On each carbon, a functional group can be read from, written to, modified, etc.
This forms the basis of the data representation in Esoteric reaction.

### Alkanes

Alkanes are the core data structure of an Esoteric Reaction program.
Each carbon atom in the alkane can have 0-4 functional groups attached to it.
Empty bond locations on a carbon atom will be filled with filler hydrogen.
Alkanes can have new, empty carbon atoms attached anywhere.

### Alkane Length

Alkanes are their own arrays and maps.
Each carbon atom can store 0-4 bonds per atom, depending on the bonded functional groups
and whether the carbon is at the end or in the middle of a molecule.
The length of the longest chain of an alkane is its *capacity*;
it represents the half maximum number of functional groups that the alkene can hold.
You cannot attach more functional groups if there are no more empty hydrogens (i.e. fully saturated).

### The Electron Rules: Octet Rule and Other Similar Rules

All bonds in the VRM must follow the octet rule, and other similar rules
(duplet rule, 18-electron rule, here collectively called "electron rules") in order to be valid.
It is considered a runtime error if a molecule invalidates the electron rules.
Any empty bonds will be filled by a filler hydrogen.

## Types and Functional Groups

Types are highly dependent on functional groups.
Certain functional groups directly attached to an alkane
determines the type of the functional groups attached to it.

In the table below, `R` represents the existing alkane chain,
and `R'`, `R''`, `R'''`, etc., are other functional groups, or another alkane.

| Type    | Description                                                                | Formula          | Name           | Notes                                                                                                          |
| ------- | -------------------------------------------------------------------------- | ---------------- | -------------- | -------------------------------------------------------------------------------------------------------------- |
| Integer | A simple 64-bit integer.                                                   | `HOR'`           | Ether          | The sum of atomic numbers of `R'` is the actual data. `R'` can be antimatter (prefixed with `-` in its symbol) |
| Boolean | A true or false value.                                                     | `HB(OH)R'`       | "Borinic Acid" | `true` if `R'` is non-`H`/non-empty.                                                                           |
| String  | A list of integers, with each integer representing a Unicode Scalar Value. | `HSR'`           | Sulfide        | `R'` must be either nothing/a `H`, or an etherized alkane.                                                     |
| Pair    | A 2-tuple of any two types, including lists, maps, and tuples.             | `HNR'R''`        | Tertiary Amine | `R'` is the "key", `R''` is the "value".                                                                       |
| List    | A heterogeneous list of any type.                                          | `HC_(n)H_(2n+1)` | Alkane         | Bonds a new alkane with `n` carbons, for a list.                                                               |
| Map     | A heterogenous list of pairs.                                              | `HC_(n)H_(2n+1)` | Alkane         | Bonds a new alkane with `n` carbons, for a map.                                                                |

## Instructions

### Alkane Manipulation

Most instructions regarding manipulating the alkane are alkali and alkali earth metals.

| Instruction | Effect                                                 | Notes                                       |
| ----------- | ------------------------------------------------------ | ------------------------------------------- |
| `K`         | Move alkene pointer "up"                               |                                             |
| `Ca`        | Move alkene pointer "down"                             |                                             |
| `Na`        | Add carbon at the "top"                                |                                             |
| `Mg`        | Add carbon at the "bottom"                             |                                             |
| `Li`        | Add carbon before pointer                              |                                             |
| `Be`        | Add carbon after pointer                               |                                             |
| `Rb`        | Remove carbon at the "top"                             | Also discards any bonded functional groups. |
| `Sr`        | Remove carbon at the "bottom"                          | Also discards any bonded functional groups. |
| `Cs`        | Remove carbon before pointer                           | Also discards any bonded functional groups. |
| `Ba`        | Remove carbon after pointer                            | Also discards any bonded functional groups. |
| `HR`        | Bond functional group `R` to the current carbon of `A` |                                             |

### Unary Alkane Operations

Operations on the alkane are highly dependent on both arguments being the same type.
However, these operations are independent of that, and are mostly noble gases.
Any instances of `X` can be `Li` or `Be`, for placing the result "above" or "below" the current carbon.
All functional groups bonded to the current carbon are moved with the carbon.

Unmarked operaions default to the carbon "below" the current one.

| Instruction | Effect                                                | Notes                                                                                  |
| ----------- | ----------------------------------------------------- | -------------------------------------------------------------------------------------- |
| `HeX`       | Duplicate the current carbon.                         |                                                                                        |
| `NeX`       | Swap the current carbon.                              | `X` determines which carbon to swap with. NOP if there is no carbon in that direction. |
| `ArX`       | Rotate surrounding carbons.                           | `X` determines which way to rotate carbons.                                            |
| `Kr`        | Print the values of the groups bonded to this carbon. | No trailing newline.                                                                   |
| `Xn`        | Print the values of the groups bonded to this carbon. | With trailing newline. Each group is separated by newlines.                            |
| `RdX`       |                                                       |                                                                                        |
| `OgX`       |                                                       |                                                                                        |

### Binary Operations

Binary operations on the alkane are highly dependent on the types of the two arguments.
Like with unary alkane operations, any `X` can be replaced with `Li` and `Be`
to change which carbon is targeted for the second argument.

| Instruction | Effect   | Notes                                                                                                                 |
| ----------- | -------- | --------------------------------------------------------------------------------------------------------------------- |
| `Al`        | Add      |                                                                                                                       |
| `Si`        | Subtract |                                                                                                                       |
| `Mn`        | Multiply |                                                                                                                       |
| `Db`        | Divide   |                                                                                                                       |
| `Mo`        | Modulo   |                                                                                                                       |
| `In`        | Index    | Current functional group must be String, Pair, List, or Map. Other functional group must be the same type as the key. |
| `Pd`        | Append   | Current functional group must be String, Pair, or List.                                                               |

#### Binary Operations Type Charts

Types in parentheses `(type)` are the second argument.

| Op  | Integer       | Boolean | String         | Pair              | List              | Map                                  |
| --- | ------------- | ------- | -------------- | ----------------- | ----------------- | ------------------------------------ |
| Add | Addition      | And     | Concat         | Element-wise Add  | Concat            | Set Union (of keys)                  |
| Sub | Subtraction   | Or      | Difference     | Element-wise Sub  | Difference        | Set Difference                       |
| Mul | Multiplcation | Xor     | (Int) Repeat   | Element-wise Mul  | (Int) Repeat      | Cartesian Product (key, pair(v1,v2)) |
| Div | Division      |         |                | Element-wise Div  |                   | Set Intersection (of keys)           |
| Mod | Modulo        |         |                | Element-wise Mod  |                   |                                      |
| Idx |               |         | (Int) Get Char | (Int) Get Element | (Int) Get Element | (key) Get Value                      |
| App |               |         |                |                   |                   |                                      |
|     |               |         |                |                   |                   |                                      |
