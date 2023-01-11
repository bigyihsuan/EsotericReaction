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

### Unknown Elements

### Light and Heat

## Coefficients

## Subscripts

## Molecules

## Types

### Number

### String

### List

### Map

### Code

## The Virtual Reaction Machine
