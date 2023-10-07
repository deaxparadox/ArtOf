# The Concept of NULL

The concept of null is interesting and sometimes misunderstood. Confusion can occur because we often deal with several similar, yet distinct concepts, including:

- The null concept
- The null pointer constant
- The NULL macro
- The ASCII NUL
- A null string
- The null statement

When NULL is asisgned to a pointer, it means the pointer does not point to anything. The null concept refers the idea that a pointer can hold a special value is not  equal to another pointer. It does not point to any area of memory. Two null pointers will always to equal to each other. There can be a null pointer type of each pointer type, such as a pointer to a character or a pointer to an integer, although this is uncommon.

The null concept is an abstraction supported by the null pointer constant. This constant may or may not be a constant zero.

The NULL maxro is a constant integer zero cast to a pointer to void. In many libraries it is defined as follows:

```c
#define NULL ((void *)0)
```

If a nonzero bit pattern is used by the compiler to represent null, then it is the compiler's responsibility to ensure all uses of NULL or 0 in a pointer context are treated as null pointers. The actual interal representation of null is implementation-defined. The use of NULL and 0 are language-level symbols that represent a null pointer.

The ASCII NUL is defined as a byte containing all zeros. However, this is not the same as a null pointer. A string in C is represented as a sequence of characters terminated by a zero value. The null string is an empty string and does not contain any characters. Finally, the null statement consists of a statement with a single semicolon.

----------

A null pointer and an uninitialized pointer are different. An uninitialized pointer can contain nay value, whereas a pointer containing NULL does not reference any location in memory.

----------
