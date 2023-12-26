#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>

void *computation(void *add);

pthread_mutex_t mutex1 = PTHREAD_MUTEX_INITIALIZER;

int main() {
    pthread_t thread1;
    pthread_t thread2;

    long value1 = 0;
    // long value2 = 2;

    // computation((void *) &value1);
    // computation((void *) &value2);

    pthread_create(&thread1, NULL, computation, (void *) &value1);
    pthread_create(&thread2, NULL, computation, (void *) &value1);

    pthread_join(thread1, NULL);
    pthread_join(thread2, NULL);
    exit(0);
}

void *computation(void *add){
    // long sum = 0;
    long *add_num = (long *) (add);


    pthread_mutex_lock( &mutex1 );
    for (long i=0; i<1000000000; i++) {
        // sum += *add_num;
        *add_num += 1;
    }
    printf("Add: %ld\n", *add_num);
    pthread_mutex_unlock( &mutex1 );
    return NULL;
}
