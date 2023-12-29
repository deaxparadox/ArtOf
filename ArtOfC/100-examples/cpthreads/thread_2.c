#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <unistd.h>
#include <signal.h>


#define SIZE 3

void SIGINT_handler(int);
void *computation(void *add);
long __factorial_calculate(int);
void *factorial(void *);
pthread_mutex_t mutex1 = PTHREAD_MUTEX_INITIALIZER;
void terminate_all_threads();
void join_all_threads();

pthread_t create_thread_identifier();
pthread_t threads[3];

int main() {
    pthread_t thread1;
    threads[0] = thread1;

    pthread_t thread2;
    threads[1] = thread2;

    pthread_t factorial_thread_t;
    threads[2] = factorial_thread_t;

    // pthread_t threads[3] = {thread1, thread2, factorial_thread_t};

    signal(SIGINT, SIGINT_handler);


    long value1 = 0;
    // long value2 = 2;
    int factorial_num = 43;

    // computation((void *) &value1);
    // computation((void *) &value2);

    pthread_create(&threads[0], NULL, computation, (void *) &value1);
    pthread_create(&threads[1], NULL, computation, (void *) &value1);
    pthread_create(&factorial_thread_t, NULL, factorial, (void *) &factorial_num);

    pthread_join(threads[0], NULL);
    pthread_join(threads[1], NULL);
    pthread_join(factorial_thread_t, NULL);
    // join_all_threads();
    exit(0);
}

void *computation(void *add){
    long *add_num = (long *) (add);
    pthread_mutex_lock( &mutex1 );
    for (long i=0; i<1000000000; i++) {
        *add_num += 1;
    }
    printf("Add: %ld\n", *add_num);
    pthread_mutex_unlock( &mutex1 );
    return NULL;
}


long __factorial_calculate(int num) {
    if (num == 0) return 0;
    if (num == 1) return 1;
    return __factorial_calculate(num-1) + __factorial_calculate(num-2);
}
void *factorial(void *x) {
    int *n = (int *) (x);
    long total = __factorial_calculate(*n);
    printf("Factorial of %d is %ld\n", *n, total);
}

void join_all_threads(){
    printf("Waiting for threads\n");
    for (int i=0; i<SIZE; i++) {
        pthread_join(threads[i], NULL);
    }
}

void terminate_all_threads(){
    printf("Terminating all threads\n");
    for (int i=0; i<SIZE; i++) {
        pthread_cancel(threads[i]);
    }
}

void SIGINT_handler(int s) {
    terminate_all_threads();
    printf("Terminating program\n");
}