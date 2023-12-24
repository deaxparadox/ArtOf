#include <stdio.h>
#include <stdlib.h>
#include <limits.h>
#include <math.h>

int minimum(int a, int b){
    if (a > b) {
        return b;
    }
    return a;
}

int maximum(int a, int b){
    if (a <= b) {
        return b;
    }
    return a;
}

int maxArea(int* height, int heightSize) {
    int i = 0;
    int j = heightSize - 1;
    int area = 0;


    while (i < j) {
        area = maximum(area, minimum(height[i], height[j]) * (j-i));
        if (height[i] <= height[j]) { i+=1;} 
        else { j-=1 ;}
    }
    return area;
}

int main() {
    int height[] = {1,8,6,2,5,4,8,3,7};
    int n = 9;
    // int height[] = {1, 1};
    // int n = 2;
    int area = maxArea(height, n);
    printf("area: %d\n", area);
    exit(0);
}