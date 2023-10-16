# To NULL or not to NULL

Which is better form: using NULL or using 0 when workign with pointers? Either is perfectly acceptable; the choice is one of preference. 

NULL should not be used in contexts other than pointers. It can definitely be a problem when used in place of the ASCII NUL character. This character is not defined in any standard C header file. It is equivalent  to the character literal, '\0', which evaluates to  the decimal values zero.

The meaning of zero changes depending on it context. It might mean the integer zero in some contexts, and it might mean a null pointer in a different context. Consider the following example:

```c
int num;
int *pi = 0;
pi = &num;
*pi = 0;
```

[<<< README](README.md) ... [Pointer to Void](102-pointer-to-void.md)