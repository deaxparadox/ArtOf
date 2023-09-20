# Arrays of Structures

```c
struct class students[3];
```

defines an array called *student*, that consists of 100 elements. Each element is defined to be of the type **struct class**.

let consider the next example:

```c

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

    return 1;
}
```

This declares the student as an array of three students **student[0]**, **student[1]**, and **student[2]**, and initializes their members as follows:

```c
student[0].subject1 = 45;
student[0].subject2 = 65;
...
student[2].subject3 = 71;
```

Note that the array is declared just as it would been with any other array. Since **student** is an array, we use the usual array-accessing methods to access individual elements and then the member operator to access members.

An array of structures is tored inside the memory in the same way as a multi-dimensional array.