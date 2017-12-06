#! /usr/bin/env python3

# Day 5 - https://adventofcode.com/2017/day/5

from util import expect, solution, input


# An urgent interrupt arrives from the CPU: it's trapped in a maze of jump
# instructions, and it would like assistance from any programs with spare
# cycles to help find the exit.
#
# The message includes a list of the offsets for each jump. Jumps are relative:
# -1 moves to the previous instruction, and 2 skips the next one. Start at the
# first instruction in the list. The goal is to follow the jumps until one
# leads outside the list.
#
# In addition, these instructions are a little strange; after each jump, the
# offset of that instruction increases by 1. So, if you come across an offset
# of 3, you would move three instructions forward, but change it to a 4 for the
# next time it is encountered.
# 
#
# Now, the jumps are even stranger: after each jump, if the offset was three or
# more, instead decrease it by 1. Otherwise, increase it by 1 as before.
def puzzle1(data, part_two=False):
    data = data[:]
    index = 0
    steps = 0
    while index >= 0 and index < len(data):
        value = data[index]
        if part_two and value >= 3:
            data[index] -= 1
        else:
            data[index] += 1
        index += value
        steps += 1
    return steps


def puzzle2(data):
    return puzzle1(data, part_two=True)


if __name__ == '__main__':
    data = list(map(int, input('05').split('\n')))

    # Part 1
    expect(puzzle1, [0, 3, 0, 1, -3], 5)
    solution(puzzle1(data), 326618)

    # Part 2
    expect(puzzle2, [0, 3, 0, 1, -3], 10)
    solution(puzzle2(data), 21841249)
