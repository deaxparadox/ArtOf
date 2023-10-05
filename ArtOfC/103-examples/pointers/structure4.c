#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

// #define INFORMATION_LIMIT 2


#include "theader1.h"


int main() {
    printf("\nINFORMATION LIMIT: %d\n\n", INFORMATION_LIMIT);

    char name[] = "Nitish Kushwaha", *ptr = NULL;
    // ptr = name;
    if (ptr==NULL)
        printf("\nValue not assigned\n\n");
    else
        printf("\nName: %s\n\n", ptr);

    exit(0);
}
