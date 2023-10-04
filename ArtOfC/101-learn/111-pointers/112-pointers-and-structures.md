# Pointers and Structures


The name of the array stands for the address of its zeroth element. The same thing is true of the names of arrays of structure variables. Suppose **product** is an array variable of **struct** type. The name product represents the address of its zeroth element. Consider the following declaration:

```c
struct inventory 
{
    char name[30];
    int number;
    float price;
} product[2], *ptr;
```

This statement declares **product** as any array of two elements, each of the type **struct inventory** and **ptr** as a poointer to data objects of the type **struct inventory**. The assignment.

```c
ptr = product
```

would assign the address of the zeroth element of **product** to **ptr**. That is, the pointer **ptr** will now poniter to **product[0]**. Its memebers can be accessed using the following notation.

```c
ptr->name
ptr->number
ptr->price
```

The synbom **->** is called the arrow operator (also konwn as member selection operator) and is made up of a minus sign and a greater than sign. Note that **ptr->** is simple another way of writing **product[0]**.

When the pointer **ptr** is incremented by one, it is made to point to the next record, i.e., product[1]. The following **for** statement will print the values of members of all the elements of product array.

```c
for(ptr = product; ptr<product+2; ptr++) 
    printf("%s %d %f\n", ptr->name, ptr->number, ptr->price);
```

We could also use the notation

```c
(*ptr).number
```

to access the member **number**. The parentheses around `*ptr` are necessary because the member operator `.` has a higher precedence then the *.

While using structures pointers, we should take care of the precedence of operators.

The operators '->' and '.', and () and [] enjoy the highest priority among the operators. They bind very tightly with their operands. For example, given the definition:

```c
struct { 
    int count;      /* poitner inside the struct */
    float *p;       /* struct type pointer */
}
```

then the statement

```c
++ptr->count;
```

increments **count**, not **ptr**, However,

```c
(++ptr).count
```

increaments **ptr** first, and then links **count**. The statement 

```c
ptr++->count;
```

is legal and increments **ptr** after accessing **count**.


The following statements also behave in the similar fashion.

```
*ptr->p         Fetches whatever p points to.
*ptr->p++       Increments P after accessing whatever it points to.
(*ptr->p)++     Increments whatever p points to.
*ptr++->p       Increments ptr after accessing whatever it points to.
```

----------


We can pass the pointer to struct as function argument and then use this poitner to work on the structure members. Consider the following function:

```c
print_invent(struct invent *item) {
    printf("Name: %s\n", item->name);
    printf("Price: %f\n", item->price);
}
```

This function can be called by 

```c
print_invent(&product);
```

The formal argument **item** receives the address of the structure **product** and therefore it must be declare as a poitner of type **struct invent**, which represents the structure of product.