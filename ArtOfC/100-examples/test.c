#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define arraysize 3
char **return_string_array()
{
    static char *arr[arraysize] = {"asf", "asdf", "wrq"};
    for (int i = 0; i < arraysize; i++)
    {
        printf("%s\n", arr[i]);
    }
    // return "returned from the array fucntion";
    return arr;
}

int *return_integer_array()
{
    static int arr[10] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
    return arr;
}

int main()
{
    char *lcp = (char *)calloc(10, sizeof(char));
    char a[] = "asdfsdf";
    char *c = "asdf";
    char sn = 'a';
    strcat(lcp, a);
    printf("%s\n", lcp);

    char **arr = return_string_array();
    for (int i = 0; i < arraysize; i++)
    {
        printf("%s ", arr[i]);
    }

    // int *arr = return_integer_array();
    // for (int i=0; i<10; i++) {
    //     printf("%d ", arr[i]);
    // }
    printf("\n");

    int nums[3] = {1, 2, 3};

    free(lcp);
    exit(0);
}
