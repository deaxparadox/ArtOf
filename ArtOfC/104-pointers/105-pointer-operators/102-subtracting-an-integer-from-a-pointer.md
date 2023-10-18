# Subtracting an integer from a pointer

Integer values can be subtracted from a pointer in the same way they are added. The size of the data type times the integer increment value is subtracted from the address.

To illustrate the effect of subtracting an integer from a pointer, we will use an array of integers as show below.

```c
#include <stdio.h>

int main() {
    int vector[] = {28, 41, 7};
    int *pi = vector + 2;
    printf("%d\n", *pi);
    pi--;
    printf("%d\n", *pi);
    pi--;
    printf("%d\n", *pi);
    return 0;
}
```

Each time one is subtracted from pi, four is subtracted from the address.

### Subtracting two pointers

When one pointer is subtracted from another, we get the difference between their addresses. This difference is not normally very useful except for determining the order of elements in an array.

The difference between the pointers is the numbers of "units" by which they differ. The difference's sign depends on the order of the operands. This is consistent with pointer addition where the number aded in the pointer's data type size. We use 'unit' as the operand. In the following example, we declare an array and pointers to the array's elements. We then take their difference:

```c
#include <stdio.h>

int main() {
    int vector[] = {28, 41, 7};
    int *p0 = vector;
    int *p1 = vector + 1;
    int *p2 = vector + 2;

    printf("p2-p0:\t%d\n", p2-p0);
    printf("p2-p1:\t%d\n", p2-p1);
    printf("p0-p1:\t%d\n", p0-p1);
    
    return 0;
}
```

```
p2-p0:  2
p2-p1:  1
p0-p1:  -1
```

In the first **printf** statement, we find the difference between the positions of the array's last element and its first element is 2. That is, their indexes differe by 2. In the last **printf** statement, the differece is a -1, indicating that p0 immediately precedes the element pointed to by p1.

The type **ptrdiff_t** is a portable way to express the difference between two pointers. In the previous example, the result of subtracting two pointers is returned as a **ptrdiff_t** type. Since pointer sizes can differ, this type simplifies the taks of working with their differences.

[Pointers to void and addition](101-pointers-to-void-and-addition.md) ... [Comparing Pointers](103-comparing-pointers.md)