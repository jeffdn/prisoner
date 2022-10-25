import random
from typing import List

BOXES = None


def run_optimized(count: int) -> bool:
    chances = int(count / 2)
    random.shuffle(BOXES)
    slips_seen = [False for _ in range(count)]

    for prisoner in range(count):
        next_box = prisoner

        if slips_seen[prisoner]:
            continue

        for idx in range(chances + 1):
            if idx == chances:
                return False

            slip = BOXES[next_box]
            slips_seen[slip] = True

            if slip == prisoner:
                break

            next_box = slip

    return True


if __name__ == '__main__':
    count = 100
    runs = 100_000
    wins = 0

    BOXES = [x for x in range(count)]

    for _ in range(runs):
        wins += int(run_optimized(count))

    print(f'completed! of {runs:,} runs, {wins:,} were successful ({(wins / runs) * 100:.2f}%)')
