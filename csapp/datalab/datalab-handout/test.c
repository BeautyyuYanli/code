#include <stdio.h>
#include <limits.h>
#include "bits.h"
int main (){
    printf("%d\n", isLessOrEqual(INT_MAX, INT_MIN));
    printf("%d\n", isLessOrEqual(1,-1));
    printf("%d\n", isLessOrEqual(INT_MIN, INT_MAX));
    printf("%d\n", isLessOrEqual(-1,1));
    return 0;
}