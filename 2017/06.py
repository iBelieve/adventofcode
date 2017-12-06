#! /usr/bin/env python3

# Day 6 - https://adventofcode.com/2017/day/6

from util import expect, solution, input
import operator


# The reallocation routine operates in cycles. In each cycle, it finds the
# memory bank with the most blocks (ties won by the lowest-numbered memory
# bank) and redistributes those blocks among the banks. To do this, it removes
# all of the blocks from the selected bank, then moves to the next (by index)
# memory bank and inserts one of the blocks. It continues doing this until it
# runs out of blocks; if it reaches the last memory bank, it wraps around to
# the first one.
#
# The debugger would like to know how many redistributions can be done before a
# blocks-in-banks configuration is produced that has been seen before.
#
#
# Out of curiosity, the debugger would also like to know the size of the loop:
# starting from a state that has already been seen, how many block
# redistribution cycles must be performed before that same state is seen again?
#
# How many cycles are in the infinite loop that arises from the configuration
# in your puzzle input?
def puzzle(data):
    steps = 0
    seen = dict()

    while str(data) not in seen:
        seen[str(data)] = steps
        
        index, value = max(enumerate(data), key=operator.itemgetter(1))
        data[index] = 0
        while value > 0:
            index = (index + 1) % len(data)
            data[index] += 1
            value -= 1
        
        steps += 1
    return steps, steps - seen[str(data)]


def puzzle1(data): 
    return puzzle(data[:])[0]


def puzzle2(data):
    return puzzle(data[:])[1]


if __name__ == '__main__':
    data = list(map(int, input('06').split('\t')))

    # Part 1
    expect(puzzle1, [0, 2, 7, 0], 5)
    solution(puzzle1(data), 12841)
    
    # Part 2
    expect(puzzle2, [0, 2, 7, 0], 4)
    solution(puzzle2(data), 8038)
