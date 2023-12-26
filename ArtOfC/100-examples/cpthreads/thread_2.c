#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <unistd.h>
#include <signal.h>

void SIGINT_handler(int);
void *computation(void *add);
pthread_mutex_t mutex1 = PTHREAD_MUTEX_INITIALIZER;

pthread_t create_thread_identifier();

int main() {
    pthread_t thread1;
    pthread_t thread2;
    pthread_t threads[2] = {thread1, thread2};

    signal(SIGINT, SIGINT_handler);


    long value1 = 0;
    // long value2 = 2;

    // computation((void *) &value1);
    // computation((void *) &value2);

    pthread_create(&threads[0], NULL, computation, (void *) &value1);
    pthread_create(&threads[1], NULL, computation, (void *) &value1);

    pthread_join(threads[0], NULL);
    pthread_join(threads[1], NULL);
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


void SIGINT_handler(int) {
    printf("Terminating program\n");
}