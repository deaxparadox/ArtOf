# Altering the size of a block: realloc

If the previously allocated memory is not sufficient and we nned additional space for more elements. It is also possible that the memory allocated is much larger than necessary and we want to reduce it. In both the cases, we can change the memory size already allocated with the help of the function **realloc**. This process is called the *reallocation* of memory. 

For example, if the original allocation is done by the statement:

```c
ptr = malloc(size);
```

the reallocation of space may be done by the statement:

```c
ptr = realloc(ptr, newsize);
```

This function allocates a new memory space of size *newsize* to the pointer variable *ptr* and returns the a pointer to the first byte of the new memory block. The newsize may be larger or smaller than the size. 

Remeber, the new memory may or may not begin at the same place as the old one. In case, it is not able to find additional space in the same region, it will create the same in an entirely new region and move the content of the old block in the new block. The function gurantees that the old data will remain intact.

If the function is unsuccessfull in locationg addition space, it returns a NULL pointer and the original block is freed (lost). This implies that it is necessary to test the success of operation before preceeding further.