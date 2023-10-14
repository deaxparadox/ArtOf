# Dereferencing a Pointer Using the Indirection Operator

The indirection operator, *, returns the value pointed to by a pointer variable. This is frequently referred to as dereferencing a pointer. In the following example, **num** and **pi** are declared and initialized:

```c
int num = 5;
int *pi = &num;
```

The indirection operator is then used in the following statement to display 5, the value of num:

```c
printf("%p\n", *pi);    // Display 5
```

We can also use the result of the dereference operator as an *lvalue*. The term lvalue referes to the operand found on the left side of the assignment operator. All **lavlues** must be modifiable since they are being assigned a value.

The following will assign 200 to the integer pointed to by **pi**. Since it is pointing to the variable **num**, 200 will be assigned to **num**:

```c
*pi = 200;
printf("%d\n", num);            # Displays 200
```

[<<< Virutal Memory and Pointer](101-virtual-memory-and-pointers.md) .. [The Concept of NULL >>>](103-concept-of-null/README.md)