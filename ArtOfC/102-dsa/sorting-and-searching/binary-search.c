#include <stdio.h>
#include <math.h>
#define MAX 10


int binary_serach(int numbers[], int target) {
    int low = 0;
    int high = MAX;
    while (low < high) {
        int mid = (low + (high-low)) / 2;
        if (target < numbers[mid]) {
            high = mid - 1;
        } else if (target > numbers[mid]) {
            low = mid + 1;
        } else {
            return mid;
        }
    }
    return -1;
}

int main() {

    int numbers[MAX], *ptr = numbers;
    for (int i=0; i<MAX; i++) {
        numbers[i] = i+11;
    }

    

    int target = 100000;
    int index = binary_serach(numbers, target);
    if (index < 0) {
        printf("Targrt not found");
    } else {
        printf("Target at index: %d", index);
    }

    printf("\n");
    return 0;
}