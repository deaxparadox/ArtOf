# Memory Leaks

A memory leak occurs when allocated memory is never used again but is not freed. This can happend when:

- The memory's address is lost.
- The **free** function is never invoked though it should be (sometimes called a hidden leak).

Leaked memory cannot be reclaimed and used later. The amount of emory available to the heap manager is decreased. If memory is repeatedly allocated and then lost, then the program may terminate when more memory is needed but **malloc** cannot allocated it because it ran out of memory. In extreme cases, the operating system may crash.


### Losing the address

An example of losing the address of memory is where **pi** reassigned a new address. The address of the first allocation of memory is lost when **pi** allocated memory a second time.

```c
    int *pi = (int *) malloc(sizeof(int));
    *pi = 5;

    pi = (int *) malloc(sizeof(int));    
```

In below illustrated figure, where the before and after images refer to the programs' state before and after the second **malloc**'s execution. The memory at address 500 has not been released, and the program no longer holds this address anywhere.

![Losing an address](../../asset/dynamic-memory-allocation-1.png)

Another example alllocates memory for a string, initializes it, and then displays the string character by character:

```c
    
    char *name = (char *) malloc(strlen("Susan")+1)
    strcpy(name, "Susan");
    while (*name != 0) {
        printf("%c", *name);
        name++;
    }

```

However, it increments **name** by one and each loop iteration. At the end, name is left pointing to the string's **NUL** termination character. The allocated memory's starting address has been lost.

![Losing address of dynamically allocated memroy](../../asset/losing-address-of-dynamically-allocated-memor.png)

### Hidden memory leaks

Memory leaks can also occur when the program should release memory but does not. A hidden memory leaks occur when an object is kept in the heap even thoughthe object is no longer needed.

Memory leaks can also occur when freeing structures created using the **struct** keyword. If the structure contains pointer to dynamically allocated memroy, then these pointers may need to be freed before the structure is freed.

- [<<< Dynamic Memory Allocations](101-dynamic-memory-allocation.md) | [Dyanmic Memory Allocations Functions >>>](103-dynamic-memory-allocation-functions.md)