# Constant pointers to constants

A constant pointer to a constant is an infrequently used pointer type. The pointer cannot be changed, and the data it pointers to cannot be changed through the pointer. An example of a constant pointer to a constant integer follows:

```c
    const int * const cpci = &limit;
```

As with pointers to constants, it is not necessary to assign the address of a constnat to **cpci**. Instead, we could have used **num** as shown below:

```c
    int num;
    const int * const cpci = &num;
```

Givent a constant pointer to a constant we cannot:

- Modify the pointer
- Modify the data pointed to by the pointer


Trying to assign a new address to **cpci** will result in a syntax error:

```c
    cpci = &num;        // error
```

If we try to deference the pointer and assign a value as shown below, we will also get a syntax error:

```c
    *cpci = 25;     // error
```

