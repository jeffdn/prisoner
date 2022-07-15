#include <time.h>
#include <stdio.h>
#include <stdlib.h>
#include <limits.h>
#include <stdbool.h>
#include <stdint.h>
#include <stddef.h>
#include <string.h>


unsigned int _generate_range(unsigned int max) {
    return rand() % max;
}


unsigned int *_generate_boxes(unsigned int count) {
    size_t size = count * sizeof(unsigned int);
    unsigned int *boxes = malloc(size);

    // First, populate the boxes with their corresponding slip.
    for (unsigned int slip = 0; slip < count; slip++) {
        boxes[slip] = slip;
    }

    // Now, redistribute the slips randomly.
    for (unsigned int i = count; i > 0; i--) {
        unsigned int to_swap = _generate_range(i + 1);
        unsigned int left = boxes[i], right = boxes[to_swap];

        boxes[to_swap] = left;
        boxes[i] = right;
    }

    return boxes;
}


bool run_optimized(unsigned int count) {
    unsigned int chances = count / 2;
    unsigned int *boxes = _generate_boxes(100);
    bool *slips_seen = malloc(count * sizeof(bool));
    memset(slips_seen, false, count * sizeof(bool));

    for (unsigned int prisoner = 0; prisoner < count; prisoner++) {
        unsigned int next_box = prisoner;

        if (slips_seen[prisoner] == true) {
            continue;
        }

        for (unsigned int _i = 0; _i <= chances; _i++) {
            if (_i == chances) {
                free(boxes);
                free(slips_seen);

                return false;
            }

            unsigned int slip = boxes[next_box];
            slips_seen[slip] = true;

            if (slip == prisoner) {
                break;
            }

            next_box = slip;
        }
    }

    free(boxes);
    free(slips_seen);

    return true;
}

int main(int argc, char **argv) {
    srand(time(NULL));

    unsigned int runs = 1000000, wins = 0;

    for (int i = 0; i < runs; i++) {
        wins += (unsigned int) run_optimized(100);
    }

    printf(
        "completed! of %u runs, %u were successful (%.2f%%)\n",
        runs,
        wins,
        ((double) wins / (double) runs) * 100
    );

    return 0;
}
