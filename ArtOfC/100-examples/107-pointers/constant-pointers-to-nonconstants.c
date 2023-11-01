#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

int main() {
    // int num = 5;
    // const int limit = 500;

    // int * const cpi = &limit;
    // printf("%d\n", *cpi);

    // *cpi = limit;
    // printf("%d\n", *cpi);

    // *cpi = num;
    // printf("%d\n", *cpi);

    // // cpi = &num;


    int num;
    int age;
    int * const cpi = &num;
    cpi = &age;

    

    exit(0);
}