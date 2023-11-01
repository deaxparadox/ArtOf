# Dynamic Memory Management

Much of the power of pointers stems from their ability to track dynamically allocated memory. The management of this memory through pointers forms the basis for many operations, including those need to manipulate complex data structures. To be able to fully exploit these capabilities, we need to understand how dynamic memory management occurs to C.

The ability to allocate and them deallocate memory allows an application to manage its memory more efficiently and with greater flexibility. Instead of having to allocate memory to accomodate that largest possible size for a data structure, only the acutal amount required needs to be allocated.

C also support dynammic memory management where objects are allocated from the heap. This is done manuallly using functions to allocated and deallocate memory. This process if referred to an *dynamic memory management*.

- [Dynamic Memory Allocation](101-dynamic-memory-allocation/README.md)
- [Dynamic Memory Allocation Functions](102-dynamic-memory-allocation-functions/README.md)
- [Deallocating Memory Using the free Function](103-deallocating-memory-using-the-free-function/README.md)
- [Dangling Pointers](104-dangling-pointers/README.md)
- [Dynamic Memory Allocation Techniques](105-dynamic-memory-allocation-techniques/README.md)