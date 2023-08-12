# The global Statement

The **global** statement tells Python that a function plans to change one or more global names--that is, names that live in the enclosing module's scope (namespace).

## Global names are variables assigned at the top level of the enclosing module file.

Suppose we have a file `mod_1.py` and at top we define a variable `x = 1`, this `x` in global variable, it's defined outside any enclosing scope.

```py
x = 1
```

- Global names must be declared only if they are assigned within a function.
- Global names may be referenced within a function without being declared.