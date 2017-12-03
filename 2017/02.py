#! /usr/bin/env python3

from util import expect, solution, input
from itertools import permutations


# The spreadsheet consists of rows of apparently-random numbers. To make sure
# the recovery process is on the right track, they need you to calculate the
# spreadsheet's checksum. For each row, determine the difference between the
# largest value and the smallest value; the checksum is the sum of all of these
# differences.
def checksum(table):
    return sum(max(row) - min(row) for row in table)


# It sounds like the goal is to find the only two numbers in each row where one
# evenly divides the other - that is, where the result of the division
# operation is a whole number. They would like you to find those numbers on
# each line, divide them, and add up each line's result.
def divisible(table):
    return sum(next(a // b for a, b in permutations(row, 2) if a % b == 0)
               for row in table)


if __name__ == '__main__':
    table = [[int(s) for s in row.split('\t')]
             for row in input('02').split('\n')]

    # Part 1
    expect(checksum, [[5, 1, 9, 5],
                      [7, 5, 3],
                      [2, 4, 6, 8]], 18)
    solution(checksum(table), 42299)

    # Part 2
    expect(divisible, [[5, 9, 2, 8],
                       [9, 4, 7, 3],
                       [3, 8, 6, 5]], 9)
    solution(divisible(table), 277)
