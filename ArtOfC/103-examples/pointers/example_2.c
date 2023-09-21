#include <stdio.h>

int main() {
    int x[5] = {1, 2, 3, 4, 5};
    printf("%d\n", *x);
    printf("%d\n", *(x+1));
}