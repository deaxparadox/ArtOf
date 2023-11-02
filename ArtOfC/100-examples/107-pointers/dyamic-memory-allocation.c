#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>


int main() {
    
    char *name = (char *) malloc(strlen("Susan")+1)
    strcpy(name, "Susan");
    while (*name != 0) {
        printf("%c", *name);
        name++;
    }

    int *pi = (int *) malloc(sizeof(int));
    *pi = 5;

    pi = (int *) malloc(sizeof(int));

    printf("*pi: %d\n", *pi);
    free(pi);
    
    exit(0);
}