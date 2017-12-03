#! /usr/bin/env python3

from util import expect, solution, input
from itertools import count
import math


# Each square on the grid is allocated in a spiral pattern starting at a
# location marked 1 and then counting up while spiraling outward. For example,
# the first few squares are allocated like this:
# 
# 17  16  15  14  13
# 18   5   4   3  12
# 19   6   1   2  11
# 20   7   8   9  10
# 21  22  23---> ...
#
# While this is very space-efficient (no squares are skipped), requested data
# must be carried back to square 1 (the location of the only access port for
# this memory system) by programs that can only move up, down, left, or right.
# They always take the shortest path: the Manhattan Distance between the
# location of the data and square 1.  up, down, left, or right. They always
# take the shortest path: the Manhattan Distance between the location of the
# data and square 1.
def access(addr):
    ring = math.ceil((math.sqrt(addr) - 1)/2)
    width = 1 + 2 * ring
    
    if ring == 0:
        return 0

    start = (width - 2) ** 2 + 1
    center = start + (width - 3)//2

    offset = (addr - center) % (width - 1)

    return ring + offset


if __name__ == '__main__':
    number = 265149
    
    expect(access(1), 0)
    expect(access(12), 3)
    expect(access(23), 2)
    expect(access(1024), 31)
    solution(access(number), 438)

