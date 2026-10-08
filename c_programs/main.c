#include <stdio.h>
#include "route.h"

int main(void) {
    int distance = calculate_distance(10, 50);

    printf("Distance: %d\n", distance);

    return 0;
}
// gcc c_programs/main.c c_programs/route.c -o output
