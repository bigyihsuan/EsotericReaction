# Esoteric Reaction

Esoteric Reaction is a functional/stack-based/concatenative esoteric programming language where code is chemical equations.

## Goals

* Practice programming in Rust.
* Write an esolang that looks like chemical equations.

## Examples

### Hello World

```lisp
S"Hello World!" + heat -> S
; explanation
S"Hello World!" ; push string literal
+ heat          ; pop an element, output to STDOUT
-> S            ; conservation of mass
```

### Cat

```lisp
Li(light + heat) -> Li
; explanation
Li(            ; loop forever...
  light + heat ; push a line of STDIN, pop and output to STDOUT
)              ; end loop
-> Li          ; conservation of mass
```

## Law of Conservation of Mass

All Esoteric Reaction programs must follow the Law of Conservation of Mass:
the number of atoms per element on the left side of the `->` must be equal to the number of atoms per element on the right side.

Note that this is conservation of *mass*, and not energy.

## Elements

All 118 known elements are reserved for built-ins.

### Undiscovered Elements

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

### Light and Heat

## Coefficients

## Subscripts

## Molecules

## Equations

## Naming Equations

```lisp
Uue: ... ; equation here
; usage
ABC + Uue -> AUue + BC ; use like any other element
```

## Types

### Number

### String

### List

### Map

### Code

## The Virtual Reaction Machine
