# Dynamic Memory Allocation

The basics steps used for dynamic memory allocation in C are:

1. User a **malloc** type function to allocated memory.
2. User this memory to support the application
3. Deallocate the memory using the **free** function

In the following example, we allocatd memory, and then the memory is released using the free function.

```c
    int *pi = (int *) malloc(sizeof(int));
    *pi = 5;
    printf("*pi: %d\n", *pi);
    free(pi);
```

The malloc function single argument specifies the number of bytes to allocate. If successfull, it returjs a pointer to memory allocated from the heap. If it fails, it returns a null pointer.

The **sizeof** operator makes the application more portable and determines the correct number of bytes to allocated for the host system.

Each time the **malloc** function (or similar function) is called, a corresponding call to the **free** function must be made when the application is done with the memory to avoid memory leaks. Once memory has been freed, it should not be accessed again. A common practice is to always assign **NULL** to a freed pointer.

[Memory Leaks](101-memory-leaks.md)