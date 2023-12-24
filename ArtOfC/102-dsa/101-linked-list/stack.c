#include <stdio.h>
#include <stdlib.h>
#include <signal.h>
#include <time.h>
#include <unistd.h>




int stack_length = 0;
/*
    sni: Stack node for int
*/
typedef struct __sni
{
    int data;
    struct __sni *next;
} sni;
sni * head;


sni *sni_create(int);
sni *sni_insert(int x, sni *);
void freeall(sni *);

void sigint_handler(int signal);

int main() {
    signal(SIGINT, sigint_handler);
    head = sni_create(12);
    sni *last;
    sni_insert(16, head);
    last = sni_insert(312, head);

    for (int i=20; i<=10000000; i++) {
        printf("adding: %d\n", i);
        last = sni_insert(i, last);
        // sleep(1);
    }

    sni * temp = head;
    while (temp != NULL) {
        printf("%d ", temp->data);
        temp = temp->next;
    }
    temp=NULL;

    // printf("%d\n", last->data);
    last = NULL;

    freeall(head);
    exit(0);
}

void sigint_handler(int signal) {
    printf("Captured SIGINT Signal\n");
    freeall(head);
    exit(1);
}

sni *sni_create(int x) {
    sni *node = (sni *) malloc(sizeof(sni));
    node->next = NULL;
    node->data = x;
    stack_length += 1;
    return node;
}
sni *sni_insert(int x, sni *h) {
    sni *t = sni_create(x);
    while (h->next != NULL)
    {
        h = h->next;
    }
    h->next = t;
    stack_length += 1;
    return h->next;
}
void freeall(sni *h) {
    printf("Freeing memory\n");
    sni *t;
    while (h != NULL) {
        // assign head to temp
        t = h;
        
        // set head to next node
        h = h->next;

        // make temp next node to NULL
        t->next = NULL;

        printf("Freed: %d\n", t->data);

        // free temp
        free(t);
    }
    free(h);
    stack_length = 0;
}