# Pointer increments and scale factor

We have seen that the pointers can be incremented like:

```c
p1 = p2 + 2;
p1 = p1 + 1;
```

and so on. Remember, however, an expression like:

```c
p1++;
```

will cause the pointer **p1** to point to the next value of its type. For example, it **p1** is an integer pointer with an intial value, say 2800, then after the operation **p1 = p1 + 1**, the value of **p1** will be 2802, and not 2801. That is, when the increment a pointer, its value is increased by the "length" of the data type that it points to. This length called the **scale factor**.

The number of bytes used to store various data types depends on the system and can be found by making use of the **sizeof** operator. For example, if x is a variable, then **sizeof(x)** returns the number of bytes needed for the variables.


### Rules of Pointer operators

The following rules apply when performing operations on pointers variables

1. A pointer variable can be assigned the address of another variable.
2. A pointer variable can be assgined the values of another pointer variable.
3. A potiner variable can be initialized with NULL or zero value.
4. A poitner variable can be pre-fixed or post-fixed with increment or decrement operators
5. An integer value may be added subtracted from a pointer variable.
6. When two pointers point to the same array, one pointer variable can be subtracted from another.
7. When two pointers point to the objects of the same data types, they can be compared using relational operators.
8. A pointer variable cannot be multiplied by a constant
9. Two pointer variables canno te added.
10. A valeu cannot be assigned to an arbitrary address.