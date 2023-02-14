# Functional Group Examples

"Empty" hydrogens are not shown.

```mermaid
graph TD

subgraph int118[Integer: 118]
c118[C] --- o118[O] --- uuo[Uuo]
end
```

```mermaid
graph TD
subgraph int5 [Integer: 7, in two ways]
c51[C] --- o51[O] --- i51[N]
c52[C] --- o52[O] --- c522[C] --- H521[H]
c522 --- H522[H]
c522 --- H523[H]
end
```

```mermaid
graph TD
subgraph int1 [Integer: 1]
c1[C] --- o1[O] --- h1[H]
end

subgraph int2 [Integer: 2]
c2[C] --- o2[O] --- he2[He]
end
```

```mermaid
graph TD

subgraph arr1 [Array of Ints: 1,2,92,8]
ca0[C] --- ca1[C] --- ca2[C] --- ca3[C]
ca0 --- oa0[O]--- ha0[H]
ca1 --- oa1[O]--- ha1[H] & ha12[H]
ca2 --- oa2[O]--- ua2[U]
ca3 --- oa3[O]--- oa32[O]
end
```

```mermaid
graph TB
subgraph map1 [Map of Int-Bool: 1:true,2:false,92:false,8:true]
cm0 --- nm0[N] --- om0[O] --- hm0[H] %% 1:true
nm0[N] --- bm0[B] --- boh0[OH] & bh0[C]
cm1 --- nm1[N] --- om1[O] --- hm1[He] %% 2:false
nm1[N] --- bm1[B] --- boh1[OH] & bh1[H]
cm2 --- nm2[N] --- om2[O] --- um2[U] %% 92:false
nm2[N] --- bm2[B] --- boh2[OH] & bh2[H]
nm3[N] --- om3[O] --- om32[O] %% 8:true
cm3 --- nm3[N] --- bm3[B] --- boh3[OH] & bh3[He]
cm0[C] --- cm1[C] --- cm2[C] --- cm3[C] %% main chain
end
```

```mermaid
graph TD
subgraph string [String Hello!]
%% [72, 101, 108, 108, 111, 33]
cs5 --- o5[O] --- as[As]
cs4 --- o4[O] --- rg[Rg]
cs2 --- o2[O] --- hs0[Hs]
cs3 --- o3[O] --- hs1[Hs]
cs1 --- o1[O] --- md[Md]
cs0 --- o0[O] --- hf[Hf]
c[C] --- s[S] --- cs0[C] --- cs1[C] --- cs2[C] --- cs3[C] --- cs4[C] --- cs5[C]
end
```