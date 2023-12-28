#include <stdio.h>
#include <stdlib.h>

int new_number(int num) {
    int rem = -1;
    int quo = -1;
    int new = 0;
    while (num != 0) {
        quo = num / 10;
        rem = num % 10;
        num = quo;
        new += rem;
    }
    return new;
}
int addDigits(int num) {
    while (num >= 10)
    {
        num = new_number(num);
        printf("%d\n", num);
    }
    
    return num;
}

int main() {
    int num = 0;
    // int num = 10;
    num = addDigits(num);
    printf("Final: %d\n", num);
    exit(0);
}