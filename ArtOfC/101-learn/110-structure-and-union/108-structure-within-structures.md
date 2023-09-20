# Structures within Structures

Structures within a structure means *nesting* of structures.

Let us consider the following structure defined to have information about the salary of employees.

```c
struct salary
{
    char name;
    char department;
    int basic_pary;
    int dearness_allowance;
    int house_rent_allowance;
    int city_allowance;
}
employee;
```

This structure defines name, department, basic pay and three kinds of allowances. We can group all the items related to allowance together declare them under a substructure:


```c

struct salary {
    char name;
    char department;
    struct {
        int dearness;
        int house_rent;
        int city;
    } allowance;
} employee;

```

The salary structure contains a member named allowance, which itself is a structure with three members. The members contained in the inner structure namely **dearness**, **house_rent** and **city** can be referred to as:

- employee.allowance.dearness
- employee.allowance.house_rent
- employee.allowance.city


----------


The following form of declaration is legal:

```c
struct salary {
    char name;
    char department;
    struct {
        int dearness;
        int house_rent;
        int city;
    } allowance, arrears;
} employee[100];
```

The inner structure has two variables, **allowance** and **arrears**. This implies that both of them have the same structure template. A base member can be access as follows:

```c
employee[1].allowance.dearness;
employee[1].arrears.dearness;
```

----------

We can also use tag names to define inner structures. Example:

```c
struct pay 
{
    int dearness;
    int house_rent;
    int city;
};

struct salary 
{
    char name;
    char department;
    struct pay allowance;
    struct pay arrears;
}

struct salary employee[100];

```