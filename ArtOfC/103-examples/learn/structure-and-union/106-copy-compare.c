#include <stdio.h>

struct class 
{
    int number;
    char name[20];
    float marks;
};



typedef struct {
    int x;
    int y;
} VECTOR;

int vector_function() {
    VECTOR v, *ptr;
    ptr = &v;
}

int main() {
    int x;
    struct class student1 = {111, "Roa", 72.50};
    struct class student2 = {222, "Reddy", 67.00};
    struct class student3;

    if (student1.number == 111) {
        student1.marks += 10.00;
    }
    float sum = student1.marks + student2.marks;
    student1.marks *= 0.5;


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