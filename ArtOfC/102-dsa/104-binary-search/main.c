#include <stdio.h>
#include <stdlib.h>


typedef struct _Node {
    int value;
    struct _Node * left;
    struct _Node * right;
} Node;

Node * create_node();
void add_value(Node * node, int a0);


int main() {

    Node * node = create_node(12);
    add_value(node);
    printf("value: %d, left: %d, right: %d\n", node->value, node->left->value, node->right->value);


    free(node->left);
    free(node->right);
    free(node);

    exit(0);
}

Node * create_node() {
    node = (Node *) malloc(sizeof(Node));
    
    Node * node = NULL;
    node->left->value = NULL;
    node->right->value = NULL;
    
    return node;
}

void add_value(Node * node, int a0) {
    
}