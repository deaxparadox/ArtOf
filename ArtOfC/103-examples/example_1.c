#include <stdio.h>
#include <stdlib.h>

void change (char *);

int main() {
    char *x;
    x = "Nitish";
    change(x);     /* call by reference or address */
    printf("%s\n", x);
    return 0;
}

void change(char *p) 
{
    printf("%s\n", p);
    // p = p + "10";
}