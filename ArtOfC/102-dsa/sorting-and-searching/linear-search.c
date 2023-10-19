#include <stdio.h>

#define MAX 10


int linear_serach(int * numbers, int target) {
    for (int i=0; i<MAX; i++) {
        if (*(numbers+i) == target) {
            return i;
        }
    }
    return -1;
}

int main() {

    int numbers[MAX], *ptr = numbers;
    for (int i=0; i<MAX; i++) {
        numbers[i] = i+11;
    }
    

    int target = 15;
    int index = linear_serach(ptr, target);
    if (index < 0) {
        printf("Targrt not found");
    } else {
        printf("Target at index: %d", index);
    }

    printf("\n");
    return 0;
}