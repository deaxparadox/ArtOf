#include <stdio.h>

struct class 
{
    int number;
    char name[20];
    float marks;
};


struct marks {
    int subject1;
    int subject2;
    int subject3;
};

int main() {

    struct marks student[3] = {
        {45, 58, 81},
        {75, 53, 69},
        {57, 36, 71}
    };

    student[0].subject1 = 45;
    student[0].subject2 = 65;
    student[2].subject3 = 71;

    return 1;
}