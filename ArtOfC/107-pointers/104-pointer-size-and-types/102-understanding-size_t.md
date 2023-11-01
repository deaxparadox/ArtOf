# Understanding size_t

The type **size_t** represents the maximum size any object can be in C. It is an unsigned integer since negative numbers do not make sense in this context. Its purpose is to provide a portable means of declaring a size consistent with the addressable area of memory available on a system. The **size_t** type is used as the return type for the **sizeof** operator and as the argument many functions, including **malloc** and **strlen**, among others.

----------

It is good practice to sue **size_t** when declaring variables for sizes such as the number of characters and array indexes. It should be used for loop counters, indexing into arrays, and sometimes for pointer arithmetic.

----------

The declaration of **size_t** is implementation-specific.  It is found in one or more standard headers, such as **stdio.h** and **stdlib.h**, and it is typically defined as follows:

```c
#ifndef __SIZE_T
#define __SIZE_T
typedef unsigned int size_t;
#endif
```

Normally, the maximum possible value for **size_t** is **SIZE_MAX**.

The recommended format specifier is **%zu**.

Consider the following example, where we define a variable as a **size_t** and then display it using two different specifiers:

```c
    size_t sizet = -5;
    printf("%d\n", sizet);      // output: -5
    printf("%zu\n", sizet);     // output: 18446744073709551611
```

```output
```

Since a variable of type **size_t** is intended for use with positive integers, using a negative value can present problems.

A positive number will be displayed properly as shown below:

```c
    size_t sizet = 5;
    printf("%d\n", sizet);      // output: 5
    printf("%zu\n", sizet);     // output: 5
```

Since **size_t** is unsigned, always assign positive number to a variable of that type.

[<<< Predefined pointer related types](101-predefined-pointer-related-types.md) ... [Using the sizeof operator with pointers](103-using-the-sizeof-operator-with-pointers.md)