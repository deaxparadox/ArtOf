#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <pthread.h>

long long unsigned int fibonacci(int num) {
    if (num == 1) {
        return 0;
    } 
    if (num == 2)
    {
        return 1;
    }

    return fibonacci(num-1) + fibonacci(num-2);
}

int main() {
    int num = 60;
    printf("Fibonacci of %d: %llu\n", num, fibonacci(num));
    return 1;
}