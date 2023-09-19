# Copying and Comparing Structure variables

If **person1** and **person2** belogn the same structure, the the following statements are valid:

```c
person1 = person2;
person2 = person1;
```

- c doesn not permit any logical operations on structure variables. 
- In case, we need to compare them, we may do so by comparing members individually.


```c
#include <stdio.h>

struct class 
{
    int number;
    char name[20];
    float marks;
};

int main() {
    int x;
    struct class student1 = {111, "Roa", 72.50};
    struct class student2 = {222, "Reddy", 67.00};
    struct class student3;


    student3 = student2;

    x = ((student3.number == student2.number) && (student3.marks == student2.marks)) ? 1 : 0;

    if (x == 1) {
        printf("\nStudent2 and student3 are same\n\n");
        printf("%d %s %.3f\n", student3.number, student3.name, student3.marks);
    } else {
        printf("\nstudent2 and student3 are different\n\n");
    }

    return 1;
}
```