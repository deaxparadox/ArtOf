# Releasing the used space: free

Compile-time storage of a variable is allocated and release by the system in accordance with its storage class. With the dynamic runt-time allocation, it is our responsibility to release the space when it is not required. The release of storage space becomes important when the storage is limited.

When we no longer need the data we stored in a block of memory, and we do not intend to use that block for storing any other information, we may release that block of memory for future use, using the **free** function:

```c
free(ptr);
```

**ptr** is a pointer to memory block, which has already been created by **malloc** or **calloc**. Use of invalid pointer in the call may create problems and cause system crash. 

We should remember two things here:

1. It is not pointer that is being release but rather what it points to.
2. To release an array of memory that was allocated by **calloc** we need only to release the pointer once. It is an error to attempt to release elements individually.

[allocating multiple block of memory: malloc >>>](102-allocaing-multiple-block-of-memory-calloc.md) ... [altering the size of a block >>>](104-altering-the-size-of-a-block.md)