#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>

#define INFORMATION_LIMIT 2

struct Information 
{
    char *name;
    int age;
    float weight;
};


void display(struct Information *);
struct Information *person(char *, int , float);

int main()
{
    

    struct Information *Persons[INFORMATION_LIMIT] = {person("Nitish Kushwaha", 24, 80.5), person("Mohit Barve", 22, 57.3)};


    display(Persons[0]);
    display(Persons[1]);

    for (int i=0; i<INFORMATION_LIMIT; i++) {
        free(Persons[i]);
    }

    exit(0);
}

void display(struct Information *info_ptr)
{
    printf("\nInformation  => \n\tName: %s\n\tAge: %d\n\tWeight: %.1f\n", info_ptr->name, info_ptr->age, info_ptr->weight);
    printf("Address => \n\tName: %u\n\tAge: %u\n\tWeight: %u\n\n", &info_ptr->name, &info_ptr->age, &info_ptr->weight);
}

struct Information *person(char * name, int age, float weight) 
{
    struct Information *info_1_ptr;
    info_1_ptr = (struct Information *) malloc(sizeof(struct Information));
    info_1_ptr->name = name;
    info_1_ptr->age = age;
    info_1_ptr->weight = weight;
    return info_1_ptr;
}
