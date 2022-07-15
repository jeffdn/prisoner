import random
from typing import List

def _generate_boxes(count: int) -> List[int]:
    boxes = [x for x in range(count)]
    random.shuffle(boxes)

    return boxes


def run_optimized(count: int) -> bool:
    chances = int(count / 2)
    boxes = _generate_boxes(count)
    slips_seen = [False for _ in range(count)]

    for prisoner in range(count):
        next_box = prisoner

        if slips_seen[prisoner]:
            continue

        for idx in range(chances + 1):
            if idx == chances:
                return False

            slip = boxes[next_box]
            slips_seen[prisoner] = True

            if slip == prisoner:
                break

            next_box = slip

    return True


if __name__ == '__main__':
    runs = 100_000
    wins = 0

    for _ in range(runs):
        wins += int(run_optimized(100))

    print(f'completed! of {runs:,} runs, {wins:,} were successful ({(wins / runs) * 100:.2f}%)')
