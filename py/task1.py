from functools import reduce
from util import get_task

text = get_task(1)

groups = text.split("\n\n")
nums = [sum(int(x) for x in group.splitlines()) for group in groups]

print(f"Part 1: {max(nums)}")

def insert_to_sorted(arr: "list[int]", n: int) -> "list[int]":
    if n > arr[-1]: arr[-1] = n
    arr.sort(reverse=True)
    return arr

max3 = reduce(insert_to_sorted, nums, [0, 0, 0])

print(f"Part 2: {sum(max3)}")