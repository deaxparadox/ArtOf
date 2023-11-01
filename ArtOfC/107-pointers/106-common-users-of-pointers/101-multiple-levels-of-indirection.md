# Multiple levels of indrection

Pointers can use different levels of indrection. A variable declared as a pointer to a pointer, sometimes called a *double pointer*.

The examples below uses three arrays. The first array is an array of strings used to hold a list of book titles:

```c
char *titles[] = {
    "A Tale of Two Cities",
    "Wuthering Heights",
    "Don Quixote",
    "Odyssey",
    "Moby-Dick",
    "Hamlet",
    "Gulliver's Travels",
};
```

Two additional arrays are provided whose purpose is to maintian a list of the "best books" and English books. Instead of holding copies of the titles, they will hold the address of a title in a **titles** array. Both arrays will need to be declared as a pointer to a pointer to a **char**. The array's elements will hold the addresses of the **titles** array's elements. This will avoid having to duplicate memory for each title and reults in a single location for titles. If a title needs to be changed, then the change will only have to be preformed in one location.

The two array are declared below. Each array eleemnt contains a pointer that points to a second pointer to char:

```c

    char **bestBooks[3];
    char **englishBooks[4];
```

The two arrays are intialized and one of their elements is displayed as shown. In the assignment statements, the value of the righthand side is calculated by applying the subscripts first, followed by the address-of operator. For example, the second statement assigns the address of the fourth element of **titles** to the second element of **bestBooks**:


```c
    bestBooks[0] = &titles[0];
    bestBooks[1] = &titles[3];
    bestBooks[2] = &titles[5];


    englishBooks[0] = &titles[0];
    englishBooks[1] = &titles[1];
    englishBooks[2] = &titles[5];
    englishBooks[3] = &titles[6];

    printf("%s\n", *englishBooks[1]);   // Wuthering Heights
```

Memory is allocated for this example as shown: Figure 1-10.


[<<< README](README.md) ... [Contants and Pointers >>>](102-constants-and-pointers/README.md)