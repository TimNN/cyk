A quick & dirty implementation of the cyk algorithm, implemented to avoid "manual" labor for a theoretical computer science homework.

Input needs to look like this:

```
[rules]
--
[test]
[words]
```

Example:

```
S → AB, BC
A → BA
B → CC, b
C → a
--
aaaaa
aaaaaa
```

Output is some meta data and cache entries formatted as latex math.

