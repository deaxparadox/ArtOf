#include <stdio.h>


union item 
{
    int m;
    float x;
    char c;
    
} code;

int main() {
    code.m = 379;
    code.x = 7859.36;

    printf("%d\n", code.m);
    printf("%f\n", code.x);

    code.m = 19;
    printf("%d\n", code.m);

    return 1;
}