#include <stdio.h>
#include <stdlib.h>

int main()
{
    int x, *p1, **p2;
    x = 100;
    p1 = &x;
    p2 = &p1;

    printf("%d\n", **p2);
    return 0;
}