# Pointer to void

A pointer to void is a general-purpose pointer used to hold references to any data type.

An example of a pointer to void is:

```c
void *pv;
```

It has two interesting properties:

- A pointer to void will have the same representation and memroy alignment as a pointer to char.
- A pointer to void will never be equal another pointer. However, two void pointer assigned a NULL value will be equal.

Any pointer can be assigned to a pointer to void. It can then be cast back to its original pointer type. When this happens the value will be equal to the original pointer value.

The following illustrate sequence, where a pointer of int is assigned to a pointer to void and then back to a pointer ot int:

```c
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

int main() {
    int n=1, *nptr = &n;
    printf("Value: %d\n", *nptr);
    void *pv = nptr;
    int *ptr = (int *) pv;
    printf("Value: %d\n", *ptr);
    return 0;
}
```

Pointers to void are used for data pointers, not functions pointers.

----------

The **sizeof** operator can be used with a pointer to void. However, we cannot use the operator with void.

### Global and static pointers

If a pointer is declared as global or static, it is initialized to NULL when the program starts. An example of a global and static pointer follows:

```c
int *globalpi;

void foo() {
    static int *staticpi;
    ...
}

int main() {
    ...
}
```