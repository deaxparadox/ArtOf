# Dynamic Memory Allocation Functions

The following functions are found on most systems in the `stdlib.h` header file.

- **malloc**: Allocates memory from the heap
- **realloc**: Reallocates memory to a larger or smaller amount based on a previously allocated block of memory
- **calloc**: Allocates and zeroes out memory from the heap
- **free**: Returns a block of memroy to the heap

----------

Dynamic memory is allocated from the heap.

The memory allocated wil be aligned according to a pointer's data type.