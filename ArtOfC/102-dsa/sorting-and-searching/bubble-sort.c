#include <stdio.h>
#include <time.h>
#include <stdlib.h>

#define MAX 10


void swap(int *a, int *b) {
    int temp;
    temp = *a;
    *a = *b;
    *b = temp;
}

void bubble_sort(int *a) {
    // printf("Swapped list: ");
    // for (int i=0; i<MAX; i++) {
    //     printf("%d\t%u\n", *(a + i), a + i);
    // }

    for (int i=0; i<MAX; i++) {
        for (int j=0; j<MAX; j++) {
            // printf("%d, %d\n", *(a+i), *(a+j));
            if (*(a + i) < *(a + j)) {
                swap(a+i, a+j);
            }
            // break;
        }
        // break;
    }
}

int main() {

    printf("Orignal list: ");
    int numbers[MAX], *ptr = (int *) calloc(MAX, sizeof(int));
    for (int i=0; i<MAX; i++) {
        ptr[i] = rand()%100;
        printf("%d, ", *(ptr + i));
    }
    printf("\n");
    
    bubble_sort(ptr);

    printf("Sorted list: ");
    for (int i=0; i<MAX; i++) {
        printf("%d, ", *(ptr+i));
    }
    printf("\n");

    free(ptr);
}
