#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <limits.h>


int reverse(int x) {
    int rem, quo;
    long int r = 0;
    while (x > 0) {
        quo = x / 10;
        rem = x % 10;
        x = quo;
        r = r * 10 + rem;
    }
    return r;
}

bool isPalindrome(int x) {
    if (x < INT_MIN) { return false; }
    if (x > INT_MAX) { return false; }
    if (x < 0) { return false; }

    int r = reverse(x);
    
    if (x != r) { return false; }

    return true;
}



int main() {
    int * number;
    number = (int *) malloc(sizeof(int));
    scanf("%d", number);

    bool p = isPalindrome(*number);
    if (p == 1) {
        printf("true\n");
    } else {
        printf("false\n");
    }
  

    free(number);
    return 0;
}