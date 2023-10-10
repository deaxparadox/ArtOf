#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

typedef struct _node {
    int value;
    struct _node * next;
} Node;


typedef struct _list {
    Node *head;
    Node *ptr;
    Node *last;
} List;

List * create_list() {
    List *l = (List *) malloc(sizeof(List));
    l->head = NULL;
    l->last = NULL;
    l->ptr = NULL;
    return l;
}

void append(List *l, int value) {
    if (l->head == NULL && l->last == NULL && l->ptr == NULL) {
        printf("\nFirst node: %d", value);
        l->head = (Node *) malloc(sizeof(Node));
        l->head->value = value;
        l->head->next = NULL;
        l->ptr = l->head;
        l->last = l->head;
        // 
        // l->last = l->head; 
        // l->ptr = l->head;
    } else {
        printf("Node %d", value);
        l->last->next = (Node *) malloc(sizeof(Node));
        l->last = l->last->next;
        l->last->value = value;
        l->last->next = NULL;
        // 
        // l->last = l->ptr;
    }
    printf("\n");
}

void display(List *l) {
    printf("\n");
    l->ptr = l->head;
    while (l->ptr != NULL) {
        printf("Value: %d\tAddress: %u\n", l->ptr->value, l->ptr);
        l->ptr = l->ptr->next;
    }
    printf("\n");
}

void free_all(List *l) {
    l->ptr = l->head;
    l->last = NULL;
    printf("\n\n");
    while (l->head !=  NULL) {
        l->head = l->head->next;
        printf("free %d => ", l->ptr->value);
        free(l->ptr);
        printf("after free %u\n", l->ptr->value);
        l->ptr = l->head;
    }
    l->head = NULL, l->ptr = NULL;
    printf("\n");
}

int main() {

    // List *l = (List *) malloc(sizeof(List));

    // l->head = (Node *) malloc(sizeof(Node));
    // l->ptr = l->last = l->head;
    // l->head->value = 10;
    // l->head->next = NULL;

    // l->head->next = (Node *) malloc(sizeof(Node));
    // l->head = l->head->next;
    // l->head->value = 12;
    // l->head->next = NULL;

    // l->head->next = (Node *) malloc(sizeof(Node));
    // l->head = l->head->next;
    // l->head->value = 13;
    // l->head->next = NULL;

    // printf("%d\n", l->last->value);
    // printf("%d\n", l->last->next->value);
    // printf("%d\n", l->head->value);
    
    // free(l->last);
    // free(l->last->next);
    // free(l->head);
    // free(l);

    List * l = create_list();
    append(l, 10);
    append(l, 12);
    append(l, 14);
    append(l, 16);
    append(l, 18);
    append(l, 20);
    // for (int i=30; i<=100000; i++) {
    //     append(l, i);
    // }
    display(l);
    free_all(l);
    free(l);
    exit(0);
}