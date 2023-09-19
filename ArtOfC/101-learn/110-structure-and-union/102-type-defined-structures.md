# Type-Defined Structures

We can use the keyword **typedef** to define a structure as follows:


```c
typedef struct {
    // .....
    // type member1;
    // type member2;
    // ....
} type_name;

```


The type_name represents structure definition associated with it and therefore can be used to declare structure variables as show below:

```
type_name variable1, variable2, ...;
```

Remember:

1. The name *type_name* is the type definition name, not a variable
2. We cannot define a variable with *typedef* declaration.