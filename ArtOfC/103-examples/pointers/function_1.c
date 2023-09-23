#include <stdio.h>
#include <stdlib.h>

int * number() {
    int a = 1, *ptr;
    ptr = &a;
    return ptr;
}

int main() {
    int *pa = number();
    printf("%d\n", *pa);
    return 0;
}