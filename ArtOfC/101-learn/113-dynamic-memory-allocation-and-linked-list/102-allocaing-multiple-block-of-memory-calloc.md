# Allocaing Multiple Blocks of Memory: calloc

**calloc** is another memory allocation function that is normally used for requesting memory space at run time for storing derived data types such as arrays and structures. While **malloc** allocates a single block of storage space, **calloc** allocates multiple blocks of storage, each of the same size, and then sets all bytes to zero. The general form of calloc is:


```c
ptr = (cast-type *) calloc (n, elem-size);
```

The above statement allocates contiguous space of *n* blocks, each of  size *elem-size* bytes. All bytes are initialized to zero and a pointer to the first byte of the allocated region is returned. If there is not enough space, a NULL pointer is returned.

```c
...
struct student 
{
    char name[25];
    float age;
    long int id_num;
};
typedef struct student record;
record *st_ptr;
int class_size = 30;
str_ptr = (record *) calloc(class_size, sizeof(record));
...
```

**record** is of type **struct student** having three memebers: **name**, **age**, and **id_number**. The **calloc** allocates memory to hold data for 30 such records. We must be sure that the requested memory has been allocated successfully before using the **st_ptr**. This may be done as follows:

```c
if (st_ptr == NULL) {
    printf("Available memory not sufficient");
    exit(1);
}
```


[allocating a block of memory >>>](101-allocating-memory-malloc.md) ... [free >>>](103-releasing-the-used-space-free.md)