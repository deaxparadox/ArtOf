#include <stdio.h>
#include <stdlib.h>


/*
    3 * 4 = 12
*/


int main() {


    int a = 18;
    int *num = &a;

    if (*num == 1) {
        printf("lcm: 1\n");
    }

    int i = 1;
    while (*num != 1) {
        if (i == 1) {
            i++;
            continue;
        }


        if (*num % i == 0) {

            printf("%d, %d\n", *num, i);

            *num = *num / i;
            i = 1;
        }

        i++;
    }
    
    printf("%d\n",*num);
    exit(0);
}