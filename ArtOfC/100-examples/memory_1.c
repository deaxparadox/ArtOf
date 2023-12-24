#include <stdio.h>
#include <stdlib.h>

int main() {
    // void * malloc(size_t);
    int *pi = (int *) malloc(sizeof(int));
    if (pi != NULL) {
        // pointer should be good
    } else {
        // Bad pointer
    }
    const int NUMBER_OF_DOUBLES = 10;
    double *pd = (double *) malloc(NUMBER_OF_DOUBLES * sizeof(double));
    exit(1);
}