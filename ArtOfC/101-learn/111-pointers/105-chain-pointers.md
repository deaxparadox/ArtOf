# Chain of Pointers

It is possible to make a pointer to point to another pointer, thus creating a chain of pointers as shown:

![pointer 11](asset/111-pointers.png)

Here, the pointer variable **p2** contains the address of the pointer variable **p1**, which points to the location that contains the desired value. This is known as *mulitple indirections*.

A variable that is a pointer to a pointer must be declared using additional indirection operator symbols in fron of the name. Example:

```c
int **p2;
```

This declaration tells the compiler that **p2** is a pointer to a pointer of **int** type. Remember, the pointer **p2** is not a pointer to an integer, but rather a pointer to an integer pointer.

We can access the target value indirectly pointer to by pointer to a pointer by applying the indrection operator twice. Consider the following code:

```c
```