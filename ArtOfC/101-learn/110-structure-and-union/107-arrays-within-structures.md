# Arrays within Structures

C permits that use of arrays as structure members. Similarly, can use single dimensional or multi-dimensional arrays of type **int** or **float**. For example, the following structure declaration is valid:

```c
struct marks {
    int number;
    float subjectr[3];
} student[2];
```

Here, the number **subject** contains three elements, **subject[0]**, **subject[1]**, and **subject[2]**. These elements can be accessed using appropriate subscripts. For example, the name 

```c 
**student[1].subject[2]**; 
```

would refer to the marks obtained in the third subject by the second student;