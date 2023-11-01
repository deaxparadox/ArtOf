#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

int main() {
    int num = 5;
    const int limit = 500;
    
    int *pi;            // Pointer to an integer
    const int *pci;     // Pointer to a constant integer

    pi = &num;
    pci = &limit;


    printf("    num - Address: %p   value: %d\n", &num, num);
    printf("  limit - Address: %p   value: %d\n", &limit, limit);
    printf("     pi - Address: %p   value: %p\n", &pi, pi);
    printf("    pci - Address: %p   value: %p\n", &pci, pci);

    printf("%d\n", *pci);           // Output: 500

    printf("%d\n", *pci+1);           // Output: 500

    // *pci = 200;

    // *pi = 20000;
    // printf("%p\n", pi);
    // printf("%d\n", *pi);
    exit(0);
}