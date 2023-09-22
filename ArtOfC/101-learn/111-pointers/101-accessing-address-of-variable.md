# Accessing the address of a Variable



The operator **&** immediately poreceding a variable returns the address of the variable associated with it. For example, the statement:

```c
p = &quantity
```

would assign the address 5000 (the location of **quantity**) to the variable **p**. The **&** operator can be remembered as 'address of'.

The **&** operator can be used only with a simple variable or an array element.

```c
#include <stdio.h>
#include <stdlib.h>

int main() 
{
    char a;
    int x;
    float p, q;

    a = 'A';
    x = 125;
    p = 10.25, q=18.75;

    printf("%c is stored at address %u.\n", a, &a);
    printf("%d is stored at address %u.\n", x, &x);
    printf("%f is stored at address %u.\n", p, &p);
    printf("%f is stored at address %u.\n", q, &q);

    return 0;
}
```