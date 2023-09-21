#include <stdio.h>



int main() {
    int a = 1;
    int *ptr = &a;
    int **ptp = &ptr;

    printf("%d\n", **ptp);
    printf("%d\n", *ptr);
    printf("%d\n", &a);

    return 0;
}