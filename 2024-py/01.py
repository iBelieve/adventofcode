#! /usr/bin/env python3
from utils import *
from collections import Counter
import re

data = input("01")

SAMPLE = """\
3   4
4   3
2   5
1   3
3   9
3   3\
"""


def parse_input(data: str):
    left = []
    right = []

    for line in data.splitlines():
        l, r = re.split(r"\s+", line)
        left.append(int(l))
        right.append(int(r))

    return left, right


def problem1(data):
    left, right = parse_input(data)
    left.sort()
    right.sort()

    return sum(abs(l - r) for l, r in zip(left, right))


def problem2(data):
    left, right = parse_input(data)
    right = Counter(right)

    return sum(l * right[l] for l in left)


example(problem1, SAMPLE, 11)
solution(problem1(data))


example(problem2, SAMPLE, 31)
solution(problem2(data))
