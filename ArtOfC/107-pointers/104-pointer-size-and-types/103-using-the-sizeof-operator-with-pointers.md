# Using the sizeof operator with pointers

The **sizeof** operator can be used to determine the size of a pointer. The following displays the size of a pointer to char:

```c
    printf("Size of *char: %d\n", sizeof(char *));
```

```
Size of *char: 8
```

The size of a function pointer can vary.

[<<< Understanding size_t](102-understanding-size_t.md) ... [Using intptr_t and uintptr_t >>>](104-using-intptr_t-and-uintptr_t.md)