# Using intptr_t and uintptr_t

The types **intptr_t** and **uintptr_t** are used for storing pointer addresses. They provided a portable and safe way of declaring pointers, and will be the same size as the underlying pointer used on the system. They are usefull for converting pointers to their integer representation.

How to use **intptr_t**:

```c
int num;
intptr_t *pi = &num;
```

If we try to assign the address of an integer to a pointer of type **uintptr_t** as follows, we will get a syntax error:


However, performing the assignment using a case will work:


```c
uintptr_t *pu = (uintptr_t *)&num;
```

We cannot use **uintptr_t** with other data types without casting:

```c
char c;
uintptr_t *pc = (uintptr_t*)&c;
```

----------

Avoid casting a pointer to an integer. In the case of 64-bit pointers, information will be lost if the integer was only four bytes

----------
