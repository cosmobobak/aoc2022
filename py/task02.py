from functools import reduce
from util import get_task

text = get_task(2)

nums = reduce(lambda p1, p2: (p1[0] + p2[0], p1[1] + p2[1]), map(lambda p: (
    1 + p[1] + [3, 0, 6][(p[0] - p[1]) % 3],
    1 + ((p[0] + (p[1] - 1)) % 3) + p[1] * 3
), [[ord(n) - ord(c) for n, c in zip(l.split(), "AX")] for l in text.splitlines()]))

print(f"Part 1: {nums[0]}")
print(f"Part 2: {nums[1]}")

