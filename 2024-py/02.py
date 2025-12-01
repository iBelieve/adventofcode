#! /usr/bin/env python3
from utils import *

data = input("02")

SAMPLE_INPUT = """\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9\
"""


def parse_input(data: str):
    for line in data.splitlines():
        yield line.split()


def problem1(data: str):
    data = parse_input()
    safe_count = 0

    for line in data.splitlines():
        levels = line.split()
        last = None
        dir = None
        safe = True

        for level in levels:
            if not last:
                last = int(level)
                continue

            diff = int(level) - last

            if diff >= 1 and diff <= 3:
                if dir == "decreasing":
                    safe = False
                    break
                dir = "increasing"
            elif diff <= -1 and diff >= -3:
                if dir == "increasing":
                    safe = False
                    break
                dir = "decreasing"
            else:
                safe = False
                break

            last = int(level)

        if safe:
            safe_count += 1

    return safe_count


example(problem1, SAMPLE_INPUT, 2)
solution(problem1(data))
