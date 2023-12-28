#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <limits.h>
#include <math.h>

bool isHappy(int n)
{
    if ((n < INT_MIN) || (n > INT_MAX))
        return false;
    int sum = 0;
    int rem, quo;
    int store_sum[20];
    int i = 0;
    while (true)
    {

        quo = n / 10;
        rem = n % 10;
        n = quo;
        sum += (int)pow((double)rem, (double)2);
        if (sum == 1)
        {
            break;
        }

        if (n == 0)
        {
            // printf("%d\t%d\n", n, sum);
            for (int j = 0; j < i; j++)
            {
                if (sum == store_sum[j])
                    return false;
            }
            store_sum[i] = sum;
            i++;
            n = sum;
            sum = 0;
        }
    }
    return true;
}

int main()
{
    int n = 30;

    bool status;
    // status = isHappy(n);
    // printf("Status:\t%d\n",status);
    for (int i = 0; i < n; i++)
    {
        status = isHappy(i);
        printf("%d\tStatus:\t%d\n", i, status);
    }
    exit(0);
}