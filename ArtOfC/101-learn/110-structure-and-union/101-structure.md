
C supports a constructed data type known as *structures*, a mechanism for packing data of different types.

A structure is a convenient tool for handling a group of logically related data items. For example, it can be used to represent a set of attributes, such as student_name, rool_name and marks.

More examples of such structures are:

- time        :   seconds, minutes, hours
- date        :   day, month, year
- book        :   author, title, price, year
- city        :   name, country, population
- address     :   name, door-number, street, city
- inventory   :   item, stock, value
- customer    :   name, telephone, city, category

<p>
    Structures help to organize complex data in a more meaningful way.
</p>


<hr>

<h3>Defining a Structure</h3>

<p>
    Structures must be defined first for their format. Consider a book database consisting of book name, author, number of pages, and price. We can define a structure to hold this information as follows:
<p>


```c
struct book_bank
{
    char    title[20];
    char    author[20];
    int     page;
    float   price;
};
```

<p>
    The keyword <strong>strong</strong> declares a structure to hold the details of four data fields, namely <strong>title</strong>,  <strong>author</strong>,  <strong>pages</strong>, and  <strong>price</strong>. These fields are called <e>structure elements or members</e>.
<p>

<p>
    Each member may belong to a different type of data. <strong>book_bank</strong> is the name of the structure and is called the <e>structure tag</e>. The tag may be used subsequently to declare variables that have the tag's structure.
<p>

<p>
Note that the above definition has not declared any variables. It simple describes a format called <e>template</e> to represent information.
</p>

<hr>

### Declaring Structure Variables

After defining a structure format we can declare variables of that type. A structure variable declaration is similar to the declaration of variables of any other data types.

It includes the following elements:

1. The keyword **struct**.
2. The structure tag name.
3. List of variable names separated by commas.
4. A terminating semicolon.

For example, the statement:

```c
struct book_bank book1, book2, book3;
```

- declares **book1**, **book2**, and **book3** as variables of the type **struct book_bank**.

The members of a structure themselves are not variables. They do not occupy any memory until they are associated with the structure variables such as **book1**. When the compiler comes across a declaration statement, it reserves memory space for the structure variables. It is also allowed to combine both the structure definition and variables declaration in one statement.

```c
struct book_bank
{
    char    title[20];
    char    author[20];
    int     page;
    float   price;
} book1, book2, book3;
```

is valid. This used of tag name is optional here. For example: 

```c
struct
{
    char    title[20];
    char    author[20];
    int     page;
    float   price;
} book1, book2, book3;
``

declares **book1**, **book2**, and **book3** as structure variables represnting three books, but does not included tag name. However, this approach is not recommended for two reasons:

1. Without a tag name, we cannot use it for future declarations.
2. Normally, structure definition appear at the begining of the program file, before any variables or function are defined.