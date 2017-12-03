#! /usr/bin/env python3

from util import expect, solution, input
from itertools import count, islice
import math


def nth(iterable, n, default=None):
    "Returns the nth item or a default value"
    return next(islice(iterable, n, None), default)


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
    dist = math.ceil((math.sqrt(addr) - 1)/2)
    width = 1 + 2 * dist
    
    if dist == 0:
        return 0

    start = (width - 2) ** 2 + 1
    center = start + (width - 3)//2

    offset = (addr - center) % (width - 1)

    return dist + offset


def to_coord(addr):
    dist = math.ceil((math.sqrt(addr) - 1)/2)
    width = 1 + 2 * dist
    
    if dist == 0:
        return (0, 0)

    start = (width - 2) ** 2 + 1
    center = start + (width - 3)//2
    from_start = addr - start
    from_center = addr - center
    side = width - 1

    if from_start < side:
        offset = from_center
        return (dist, offset)
    elif addr - start < 2 * side:
        offset = -(from_center - side)
        return (offset, dist)
    elif addr - start < 3 * side:
        offset = -(from_center - 2 * side)
        return (-dist, offset)
    else:
        offset = from_center - 3 * side
        return (offset, -dist)


# As a stress test on the system, the programs here clear the grid and then
# store the value 1 in square 1. Then, in the same allocation order as shown
# above, they store the sum of the values in all adjacent squares, including
# diagonals.
#
# So, the first few squares' values are chosen as follows:
#
# - Square 1 starts with the value 1.
# - Square 2 has only one adjacent filled square (with value 1), so it also
#   stores 1.
# - Square 3 has both of the above squares as neighbors and stores the sum of
#   their values, 2.
# - Square 4 has all three of the aforementioned squares as neighbors and 
#   stores the sum of their values, 4.
# - Square 5 only has the first and fourth squares as neighbors, so it gets the
#   value 5.
# 
# Once a square is written, its value does not change. Therefore, the first few
# squares would receive the following values:
#
# 147  142  133  122   59
# 304    5    4    2   57
# 330   10    1    1   54
# 351   11   23   25   26
# 362  747  806--->   ...
def storage():
    grid = {
        (0, 0): 1
    }

    yield 1

    for addr in count(start=2):
        x, y = to_coord(addr)
        adjacent = [
            (x+1, y+0),
            (x+1, y+1),
            (x+0, y+1),
            (x-1, y+1),
            (x-1, y+0),
            (x-1, y-1),
            (x+0, y-1),
            (x+1, y-1)
        ]
        value = sum(grid.get(coord, 0) for coord in adjacent)
        grid[(x, y)] = value
        yield value


def storage_at(addr):
    return nth(storage(), addr - 1)


if __name__ == '__main__':
    number = 265149
    
    # Part 1
    expect(access, 1, 0)
    expect(access, 12, 3)
    expect(access, 23, 2)
    expect(access, 1024, 31)
    solution(access(number), 438)
    
    # Part 2
    expect(to_coord, 1, (0, 0))
    expect(to_coord, 2, (1, 0))
    expect(to_coord, 3, (1, 1))
    expect(to_coord, 4, (0, 1))
    expect(to_coord, 5, (-1, 1))
    expect(to_coord, 20, (-2, -1))
    expect(to_coord, 22, (-1, -2))
    expect(to_coord, 25, (2, -2))
    expect(to_coord, 10, (2, -1))

    expect(storage_at, 1, 1)
    expect(storage_at, 2, 1)
    expect(storage_at, 3, 2)
    expect(storage_at, 4, 4)
    expect(storage_at, 5, 5)
    
    solution(next(n for n in storage() if n > number))
