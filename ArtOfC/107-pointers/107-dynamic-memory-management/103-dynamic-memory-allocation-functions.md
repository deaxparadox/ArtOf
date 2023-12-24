# Dynamic Memory Allocation Functions

The following functions are found on most systems in the `stdlib.h` header file.

- **malloc**: Allocates memory from the heap
- **realloc**: Reallocates memory to a larger or smaller amount based on a previously allocated block of memory
- **calloc**: Allocates and zeroes out memory from the heap
- **free**: Returns a block of memroy to the heap

----------

Dynamic memory is allocated from the heap.

The memory allocated wil be aligned according to a pointer's data type.

### Using the malloc Function

The function **malloc** allocates a block of memory from the heap. Its return type is a pointer to void. If memory is not avialable, **NULL** is returned. The function's prototype follow:

```c
void * malloc(size_t);
```

The following shows a typical use of the malloc function:

```c
int *pi = (int *) malloc(sizeof(int));
```

The following steps are performed when the **malloc** function is executed:

1. Memory is allocated from the heap
2. The memory is not modified or otherwise cleard
3. The first byte's address is returned

To check for a **NULL** value before using the pointer as follows:

```c
int *pi = (int *) malloc(sizeof(int));
    if (pi != NULL) {
        // pointer should be good
    } else {
        // Bad pointer
    }
```

----------

Using the right size for the malloc function. 

For example, if we want to allocated space for 10 doubles, then we need to allocate 80 bytes. 

```c
double *pd = (double *) malloc(NUMBER_OF_DOUBLES * sizeof(double));
```