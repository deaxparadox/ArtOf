#include <stdio.h>
#include <stdlib.h>

int main() {
    int arr[5];
    arr[0] = 1;
    arr[1] = 1;
    arr[2] = 1;

    int i = 0;
    while (i < 5) {
        printf("%d\n", arr[i]);
        i++;
    }

    exit(0);
}