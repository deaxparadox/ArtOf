# Contants and Pointers

Using the **const** keywords with pointers is a rish and powerfull aspect of C. It provides different types of protections for different problem sets.

### Pointers to a contants


A  pointer can be defined to point to a constant. This means the pointer cannot be used to modify the value it is referencing.

```c

    int num = 5;
    const int limit = 500;
    
    int *pi;            // Pointer to an integer
    const int *pci;     // Pointer to a constant integer

    pi = &num;
    pci = &limit;

```

![Pointer to a constant integer](../asset/pointers-to-a-constant-integer.png)

The following sequence will display the address and value of these variables:

```c
    printf("    num - Address: %p   value: %d\n", &num, num);
    printf("  limit - Address: %p   value: %d\n", &limit, limit);
    printf("     pi - Address: %p   value: %p\n", &pi, pi);
    printf("    pci - Address: %p   value: %p\n", &pci, pci);
```

When executed,this sequence will produce values similar to the following:

```
    num - Address: 0x7ffdda8c186c   value: 5
  limit - Address: 0x7ffdda8c1868   value: 500
     pi - Address: 0x7ffdda8c1860   value: 0x7ffdda8c186c
    pci - Address: 0x7ffdda8c1858   value: 0x7ffdda8c1868
```

Deferencing a constant pointer is fine if we are simply reading the integer's value. 

```c
printf("%d\n", *pci);           // Output: 500
```

*We cannot dereference a constant pointer to change what the pointer references, but we can change the pointer*.

The following statemwnt is legal:

```c
    pci = &num;
```

We can deference *pci* to read it.

```c
    printf("%d\n", *pci);
```

But we cannot dereference it to modify it.

```c
    *pci = 200;
```

This will result in the following error:

```bash
error: assignment of read-only location '*pci'
```

The pointer thinks it is pointing to a constant integer; therefore, it does allow the modification of the integer using the pointer. We can still modify **num** using its name. We just can't use **pci** to modify it.

The declaration of **pci**  as a pointer to a constant integer means:

- **pci** can be assigned to  point to a different const integer.
- **pci** can be assigned to point to different nonconstant integers.
- **pci** can be deferenced for reading purposes.
- **pci** cannot be dereferenced to change what it points to.

The order of the type and **const** keyword is not important.

