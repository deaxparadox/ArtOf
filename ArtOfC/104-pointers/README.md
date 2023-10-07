# Pointers

A solid understanding of pointers and the ability to effectively use them separates a novice C programmer from a more experienced one. Pointers pervade the language and provide much of its flexibility. They provide important support for dynamic memory
allocation, are closely tied to array notation, and, when used to point to functions, add another dimension to flow control in a program.

Pointers have long been stumbling block in learning C.

The key to comprehending pointers is understanding how memory is managed in a C program. After all pointers container addresses in memory.

## Pointers and Memory

When a C program is compiled, it works with three types of memory:

1. *Static/Global*

Statically declared variables are allocated to this type of memory. Global variables also use this region of memory. They are allocated when the program starts and remain in existence until the program terminates. When all functions have access to global variables, the scope of static variables is restricted to their defining function.

2. *Automatic*

These variables are declared within a function and are created when a function is called. Their scope is restricted to the function, and their lifetimes is limited to the time the function is executing.

3. *Dynamic*

Memory is allocated from the heap and can be released as necessary. A pointer reference the memory. It exists until it is released.

![pointers 1](asset/101-pointers.png)

----------

Pointers have several uses, including:

- Creating fast and efficient code
- Providing a convenient means for addressing many types of programs
- Supporting dynamic memory allocation
- Making expressions compact and succinct
- Providing the ability to pass data structure by pointer without incurring a large overhead
- Protecting data passed as a parameter to a function.

----------

For example, in the following sequence, the third character of the names' second element is displayed with two different **printf** functions. While the two approaches are equivalent and will display the character n, the simpler approach is to use array notation.

```c
    char *names[] = {"Miller", "Jones", "Anderson"};
    printf("%c\n", *(*(names+1)+2));
    printf("%c\n", names[1][2]);
```

Pointers represent a powerful tool to create and enhance applications. On the downside, many problems can occur when using pointers, such as:

- Accessing arrays and other data structures beyond their bounds
- Referencing automatic variables after they have gone out of existence
- Referencing heap allocated memory after it has been released.
- Dereferencing a pointer before memory has been allocated to it.

## Declaring Pointers

In the following example, an integer and a pointer to an integer are declared:

```c
// integer variable
int num;

// pointer variable
int *pi;
```

The following declaration is equivalent:

```c
int* pi;
int * pi;
int *pi;
int*pi;
```

The asterisk declares the variable as a pointer.

There are several points to remember:

- The content of pi should eventually be assigned the address of an integer variable.
- These variables have not been initialized and thus contain garbage.
- There is nothing inherent to a pointer's implementation that suggests what type of data it is referencing or whether its contents are valid.
- However, the pointer type has been specified and the compiler will frequently complain when the pointer is not used correctly.

### Read a Declaration

```c
const int *pci;
```

Reading the declaration backward allows us to progressively understand the declaration:

![pointer 102](asset/102-pointers.png)


### Address of Operator

The address operator, **&**, will return its operand's address. We can initialize the *pi* pointer with the address of num using this operator as follows:

```c
int num;
int *pi = &num;
```

Pointers and integers are not the same. They may both be stored using the same number of bytes on most machines, but they are not the same. However, it is possible to cast an integer to a pointer to an integer:

```c
pi = (int *)num;
```

This will not generate a syntax error. When executed, though the program may terminate abnormally when the program attempts to deference the value at address zero.

### Displaying Pointer Values

The variable's address can be determined by printing it out as follows:


```c
    int num = 0;
    int *pi = &num;
    printf("Address of num: %d Value: %d\n", &num, num);
    printf("Address of pi: %d Value: %d\n", &pi, pi);
```

When executed , we get:


```bash
Address of num: 558996404 Value: 0
Address of pi: 558996404 Value: 0
```

The **printf** function has a couple of other field specifiers useful when displaying pointer values:

- **%x**: Displays the value as a hexadecimal number;
- **%o**: Displays the value as an octal number;
- **%p**: Displays the value in an implementation-specific manner, typically as a hexadecimal number.

... [virtual-memory-and-pointers >>> ](101-virtual-memory-and-pointers.md)