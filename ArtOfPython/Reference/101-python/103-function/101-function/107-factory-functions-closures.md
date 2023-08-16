# Factory Functions: Closures

The function object question remembers values in enclosing scopes regardless of whether those  scopes are still present in memory. In effect, have attached packets of memory (a.k.a *state retention*), which are local to each copy of the nested function created, and often provide a simple alternative to classes in the this role.

## A Simple function factory