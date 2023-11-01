# Pointer to (constant pointer to constant)

Pointers to constants can also have multiple levels of indirection. In the following example, we declare a pointer to the **cpci** pointer. Reading declarations from right to left helps clarify these types of declarations:

```c
    int num = 5;
    const int limit = 500;

    const int * const cpci = &limit;
    const int * const * pcpci;
    
```

The following illustrates their use. The output of this sequence should display 500 twice:

```c
    printf("%d\n", *cpci);
    pcpci = &cpci;
    printf("%d\n", **pcpci);
```

