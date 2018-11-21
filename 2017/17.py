#! /usr/bin/env python3

# Day 10 - https://adventofcode.com/2017/day/10

from util import expect, solution, input
from collections import defaultdict
import re, itertools
from functools import reduce


def spinlock1(steps):
    buffer = [0]
    i = 0
    next_value = 1

    while next_value <= 2017:
        i = (i + steps) % len(buffer)
        buffer.insert(i + 1, next_value)
        i += 1
        next_value += 1

    return buffer[i + 1]


def spinlock2(steps):
    buffer = [0]
    i = 0
    next_value = 1

    while next_value <= 50000000:
        i = (i + steps) % next_value
        if i == 0:
            if len(buffer) == 1:
                buffer.append(next_value)
            else:
                buffer[1] = next_value
        i += 1
        next_value += 1

    return buffer[1]



if __name__ == '__main__':
    expect(spinlock1, 3, 638)
    solution(spinlock1(394), 926)

    solution(spinlock2(394), 10150888)
