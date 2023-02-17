# Architecture

## Functional Groups

All functional groups should be represented as a tree.

All functional groups know their "head" atom, where traversal will begin.

## General structure

The main tape, at the tape-level, looks like this:

```mermaid
graph TD;
subgraph Alkane
th[Tape Head] --> C0
C0[C] --- C1[C] --- C2[C] --- C3[C] --- Cetc[...]
end

subgraph Ether
O0[O]
subgraph EtherData[Ether Data]
H0[H]
end
O0 --- EtherData
eh[Ether Head] --> O0
end

subgraph Ether2 [Ether]
O1[O]
subgraph Ether2Data[Ether Data]
H1[Fe]
end
O1 --- Ether2Data
eh2[Ether Head] --> O1
end

C0 --- Ether
C1 --- Ether2
```

## Alkanes

Alkanes are a subclass of functional groups.
They specifically have helper methods that make it more convenient
to traverse and modify.