#include <stdio.h>
#include <stdlib.h>


void print_number(int);
void gen(int, void (*f)(int));

int main() {
    gen(10, print_number);
    exit(0);
}

void gen(int range, void (*f)(int a)) {
    int i =0;
    for (;i<range; i++) {
        f(i);
    }
}

void print_number(int a) {
    printf("%d\n", a);
}