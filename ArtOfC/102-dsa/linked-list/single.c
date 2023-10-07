#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

typedef struct _node {
    int value;
    struct _node * next;
} Node;


Node * new_node(int value) {
    // 
    // create a new node,
    // and return a pointer to new node
    // 
    Node * n_node = (Node *) malloc(sizeof(Node));
    n_node->value = value;
    n_node->next = NULL;
    return n_node;
}

Node * append(Node * list, int value) {
    // 
    // append new_node at the end of current list,
    // and return a pointer to last node
    // 
    while (list->next != NULL) {
        list = list->next;
    }
    list->next = new_node(value);
    return list->next;
}

Node * insert(Node * list, int value) {
    // 
    // push item at the start of the list,
    // and return a pointer of the first node.
    // 
    Node * head = new_node(value);
    head->next = list;
    return head;
}

void display(Node * list) {
    // 
    // display the given list
    // 
    bool i = true;
    while (i) {
        if (list->next == NULL) {
            printf("Value: %d\n", list->value);
            i = false;
            continue;
        }
        printf("Value: %d\n", list->value);
        list = list->next;
    }
}

void freeup(Node * list) {
    // 
    // free the memory 
    // 
    Node * hold;
    bool i = true;
    while (i) {
        if (list->next == NULL) {
            i = false;
            continue;
        }
        hold = list;
        list = list->next;
        free(hold);
        
    }
}

int main() {

    Node * list = new_node(12);


    Node * last = append(list, 14);
    for (int i=15; i<25; i++) {
        last = append(list, i);
    }

    Node * first = insert(list, 30);
    display(first);
    freeup(first);
    // freeup(list);
    
    
    exit(0);
}