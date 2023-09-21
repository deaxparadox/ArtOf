#include <stdio.h>

struct salary
{
    char name;
    char department;
    int basic_pary;
    int dearness_allowance;
    int house_rent_allowance;
    int city_allowance;
}
employee;


struct salary {
    char name;
    char department;
    struct {
        int dearness;
        int house_rent;
        int city;
    } allowance;
} employee;

struct salary {
    char name;
    char department;
    struct {
        int dearness;
        int house_rent;
        int city;
    } allowance, arrears;
} employee[100];



int main() {
    
    return 1;
}