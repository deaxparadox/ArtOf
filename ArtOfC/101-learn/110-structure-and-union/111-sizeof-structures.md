# Size of Structures

We use unary operator **sizeoff** to tell us the size of a structure (or any variable). The expression:

```c
sizeof(struct x)
```

will evaluate the number of bytes required to hold all  the members of the structure **x**. If **y** is a simple structure varaible of type **struct  x**, then the expression:

```c
sizeof(y)
```

would also give the same answer. However if **y** is an array variable of type **struct x**, then:

```c
sizeof(y)
```

would give the total number of bytes the array **y** requries.

----------

This kind of information would be usefull to determine the number of records in a database. For example, the expression:

```c
sizeof(y)/sizeof(x)
```

would give the number of elements in the array **y**.