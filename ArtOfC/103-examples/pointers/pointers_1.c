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