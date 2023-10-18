# Pointers to void and addition

Most compilers allow arithmatics to be preformed on a pointer to void as an extension.

Here we will assume the size of a pointer to void is four. However, trying to add one to a pointer to void may result in a syntax error. 

In the the following code snippet, we declare and attempt to add one to the pointer.

```c
int num = 5;
void *pv = &num;
printf("%p\n", pv);
pv = pv + 1;
```

The resulting warning follows:


```
warning: pointer of type 'void *' used in arithmetic;
```

Since this is not standard C, the compiler issued a warning. However, the resulting address contained in **pv** will be incremented by four bytes.

[Pointer Operators](README.md) ... [Subtracting and integer from a pointer >>>](102-subtracting-an-integer-from-a-pointer.md)