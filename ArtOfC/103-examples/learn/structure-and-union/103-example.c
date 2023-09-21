#include <stdio.h>

typedef struct {
    double real;
    double img;
} complex;

complex add(complex c1, complex c2) {
    complex c3;
    c3.real = c1.real + c2.real;
    c3.img = c1.img + c2.img;
    printf("Complex real: %.3e, img: %.3e\n", c3.real, c3.img);
    return c3;
}

int main() {
    complex c1, c2;
    c1.real, c1.img = 1.0, 11.0;
    c2.real, c2.img = 2.0, 22.0;
    complex c3 = add(c1, c2);
    printf("Complex real: %.3lf, img: %.3lf\n", c3.real, c3.img);
    return 1;
}