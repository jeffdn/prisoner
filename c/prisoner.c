#include <time.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>


static unsigned int *boxes = NULL;
static bool *slips_seen = NULL;

unsigned int _generate_range(unsigned int max) {
    return rand() % max;
}


void _generate_boxes(unsigned int count) {
    // Now, redistribute the slips randomly.
    for (unsigned int i = count - 1; i > 0; i--) {
        unsigned int to_swap = _generate_range(i + 1);
        unsigned int left = boxes[i], right = boxes[to_swap];

        boxes[to_swap] = left;
        boxes[i] = right;
    }
}


bool run_optimized(unsigned int count, unsigned int chances) {
    memset(slips_seen, false, count * sizeof(bool));

    for (unsigned int prisoner = 0; prisoner < count; prisoner++) {
        unsigned int next_box = prisoner;

        if (slips_seen[prisoner] == true) {
            continue;
        }

        for (unsigned int _i = 0; _i <= chances; _i++) {
            if (_i == chances) {
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

    return true;
}

int main(int argc, char **argv) {
    unsigned int count = 100, chances = 50;
    size_t size = count * sizeof(unsigned int);
    struct timespec start_ts, end_ts, diff_ts;
    float duration;

    boxes = malloc(size);
    slips_seen = malloc(size);

    srand(time(NULL));

    unsigned int runs = 1 * 1000 * 1000, wins = 0;

    timespec_get(&start_ts, TIME_UTC);

    // First, populate the boxes with their corresponding slip.
    for (unsigned int slip = 0; slip < count; slip++) {
        boxes[slip] = slip;
    }

    for (int i = 0; i < runs; i++) {
        _generate_boxes(count);
        wins += (unsigned int) run_optimized(count, chances);
    }

    timespec_get(&end_ts, TIME_UTC);

    diff_ts.tv_sec = end_ts.tv_sec - start_ts.tv_sec;
    diff_ts.tv_nsec = end_ts.tv_nsec - start_ts.tv_nsec;

    if ((end_ts.tv_nsec - start_ts.tv_nsec) < 0) {
        diff_ts.tv_sec -= 1;
        diff_ts.tv_nsec += 1000000000;
    }

    duration = diff_ts.tv_sec + ((float) diff_ts.tv_nsec / 1000000000);

    printf(
        "complete in %.3f seconds! of %u runs, %u were successful (%.2f%%)\n",
        duration,
        runs,
        wins,
        ((double) wins / (double) runs) * 100
    );

    free(boxes);
    free(slips_seen);

    return 0;
}
