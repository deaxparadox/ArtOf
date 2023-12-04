#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

//
// Node 
typedef struct _Node {
    int value;
    int visited;
    struct _Node * left;
    struct _Node * right;
} Node;

bool found = false;
Node * create_node(int);
void insert(Node *, int);
void __search(Node *, int);
void search(Node *, int);
void pre_order(Node *);
void in_order(Node *);
void post_order(Node *);
void destroy_tree(Node *);

int main() {
    Node * root = create_node(12);
    insert(root, 11);
    insert(root, 15);
    insert(root, 12);
    insert(root, 5);
    insert(root, 10);
    
    search(root, 5);
    search(root, 5);
    search(root, 5);
    search(root, 15);
    search(root, 10);
    search(root, 11);
    search(root, 11);
    search(root, 5);
    search(root, 5);
    search(root, 5);


    printf("Pre order:\n");
    pre_order(root);

    // printf("In order:\n");
    // in_order(root);

    // printf("Post order:\n");
    // post_order(root);

    destroy_tree(root);
}

Node * create_node(int value) {
    Node * root = NULL;
    root = (Node *) malloc(sizeof(Node));
    root->left = NULL;
    root->right = NULL;
    root->value = value;
    root->visited = 0;
    return root;
}

void insert(Node * root, int value) {
    if (value < root->value) {
        if (root->left) {
            insert(root->left, value);
        } else {
            root->left = create_node(value);
        }
        
    } else {
        if (root->right) {
            insert(root->right, value);
        } else {
            root->right = create_node(value);
        }
        
    }
}

void search(Node * root, int value) {
    __search(root, value);
    found = false;
}
void __search(Node * root, int value) {
    if (found) {
        return;
    }
    if (root) {
        if (root->value == value) {
            root->visited += 1;
            // printf("Visited: %d times\n", root->visited);
            found = true;
            return;
        }

        __search(root->left, value);
        __search(root->right, value);
    }
}

void pre_order(Node * root) {
    if (root) {
        printf("Value: %d\tVisited: %d\n", root->value, root->visited);
        pre_order(root->left);
        pre_order(root->right);
    }
}
void in_order(Node * root) {
    if (root) {        
        pre_order(root->left);
        printf("%d\n", root->value);
        pre_order(root->right);
    }
}

void post_order(Node * root) {
    if (root) {        
        pre_order(root->left);
        pre_order(root->right);
        printf("%d\n", root->value);
    }
}


void destroy_tree(Node * root) {
    if (root) {
        destroy_tree(root->left);
        destroy_tree(root->right);
        free(root);
    }
}