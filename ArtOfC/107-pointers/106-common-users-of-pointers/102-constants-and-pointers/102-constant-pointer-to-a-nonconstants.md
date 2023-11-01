# Constant pointers to a nonconstants


We can declare constant pointer to a nonconstant. When we do this, it means that while the pointer cannot be changed, the data pointed to can be modified. As can example of such a pointer follows:

```c
    int num;
    int * const cpi = &num;
```

With this declaration:

- **cpi** must be initialized to a nonconstant variable
- **cpi** cannot be modified
- The data pointed to by **cpi** can be modified.

This is possible to deference **cpi** and assign a new vlaue to whatevery **cpi** is referencing. The following are two valid assignments:

```c
    *cpi = limit;
    *cpi = 25;
```

However, if we attempt to initalize **cpi** to that constant **limit**, we will get a warning:

```c
    const int limit = 500;
    int * const cpi = &limit;
```

The warning will appear as follows:

```
warning: initialization discards 'const' qualifier from pointer target type [-Wdiscarded-qualifiers]
```

If **cpi** referenced to constant **limit**, the constant would be modified. This is not desirable. We generally prefer constants to remain constant.

Once an address has been assigned to **cpi**, we cannot assigned a new value to **cpi** as shown:

```c
    int num;
    int age;
    int * const cpi = &num;
    cpi = &age;
```

The error message generated is shown below:

```
error: assignment of read-only variable 'cpi
```