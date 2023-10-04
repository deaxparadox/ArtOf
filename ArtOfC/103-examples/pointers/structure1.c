#include <stdio.h>
#include <string.h>
 
struct Student {
    int roll_no;
    char name[30];
    char branch[40];
    int batch;
};
 
int main()
{
 
    struct Student s1, *ptr;
    ptr = &s1;
 
    ptr->roll_no = 27;
    strcpy(ptr->name, "Kamlesh Joshi");
    strcpy(ptr->branch, "Computer Science And Engineering");
    ptr->batch = 2019;
 
    int length = strlen(ptr->name), i;
    printf("%d\n", length);
    for (i=0; i<length; i++) {
        printf("%c ", ptr->name[i]);
    }
    // i = 0;
    // while (ptr->name != "\0") {
    //     printf("%c", ptr->name[i]);
    //     i = i + 1;
    // }
    printf("\n\n\n");


    printf("Roll Number: %d\n", (*ptr).roll_no);
    printf("Name: %s\n", ptr->name);
    printf("Branch: %s\n", (*ptr).branch);
    printf("Batch: %d\n", (*ptr).batch);
 
    return 0;
}