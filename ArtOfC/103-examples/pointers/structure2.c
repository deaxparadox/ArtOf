#include <stdio.h>
#include <pthread.h>
#include <unistd.h>
#include <stdlib.h>

#define THREAD_LIMIT 3;

void *myThreadFunc(void *vargp)
{
    sleep(1);
    for (int i=1; i<10; i++)
    {
        printf("Id: %d\n", i);
    }
    return NULL;
    return NULL;
}

void *myThreadFunc2(void *vargp)
{
    sleep(1);
    for (int i=11; i<20; i++)
    {
        printf("Id: %d\n", i);
    }
    return NULL;
}

void *myThreadFunc3(void *vargp)
{
    sleep(1);
    for (int i=21; i<30; i++)
    {
        printf("Id: %d\n", i);
    }
    return NULL;
}

void threading_e1() {
    pthread_t thread_id_1, thread_id_2;

    printf("Before Thread\n");


    printf("\nCreating first thread\n");
    pthread_create(&thread_id_1, NULL, myThreadFunc, NULL);

    printf("\nCreating second thread\n");
    pthread_create(&thread_id_2, NULL, myThreadFunc2, NULL);
    
    pthread_join(thread_id_1, NULL);
    pthread_join(thread_id_2, NULL);
    
    printf("After Thread\n");
}


struct _Number{
    int a;
    int b;
};

typedef struct _Number Number;

void increase_a(Number *);
void increase_b(Number *);

int main() {
    Number s1, *s1_ptr;
    s1_ptr = &s1;
    s1_ptr->a = 10;
    s1_ptr->b = 20;

    printf("Original values of a: %d, b: %d\n", s1_ptr->a, s1_ptr->b);
    printf("Total: %d\n", s1_ptr->a + s1_ptr->b);

    printf("\nIncreasing value of a by 1\n");
    increase_a(s1_ptr);
    printf("Values of a: %d, b: %d\n", s1_ptr->a, s1_ptr->b);
    printf("Total: %d\n", s1_ptr->a + s1_ptr->b);

    printf("\nIncreasing value of b by 1\n");
    increase_b(s1_ptr);
    printf("Values of a: %d, b: %d\n", s1_ptr->a, s1_ptr->b);
    printf("Total: %d\n", s1_ptr->a + s1_ptr->b);

    printf("\nTotal: %d\n", s1_ptr->a + s1_ptr->b);

    threading_e1();

    exit(0);
}

void increase_a(Number * s_a) {
    s_a->a  = s_a->a += 1;
}

void increase_b(Number * s_b) {
    s_b->b  = s_b->b += 1;
}