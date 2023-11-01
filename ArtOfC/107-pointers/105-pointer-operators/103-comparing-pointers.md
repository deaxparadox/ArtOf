# Comparing Pointers

Pointers can be compared using the standard comparison operators.

However, when comparing pointers to elements of an array, the comparison's results can be used to determine the relative ordering of the array's elements.

Several comparison operators are applied to the pointers, and their results are displayed as 1 for true and 0 for false.

```c
int main() {

    int vector[] = {28, 14, 7};
    int *p0 = vector;
    int *p1 = vector + 1;
    int *p2 = vector + 2;

    printf("p2>p0:\t%d\n", p2>p0);  // p2>p0: 1
    printf("p2<p0:\t%d\n", p2<p0);  // p2<p0: 0
    printf("p0>p1:\t%d\n", p0>p1);  // p0>p1: 0
}
```

```
p2>p0:  1
p2<p0:  1
p0>p1:  0
```