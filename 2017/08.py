#! /usr/bin/env python3

# Day 8 - https://adventofcode.com/2017/day/8

from util import expect, solution, input
from collections import defaultdict
import re


def puzzle(data):
    registers = defaultdict(lambda: 0)
    max_ever = 0

    for line in data:
        m = re.match(r'(\w+) (\w+) (-?\d+) if (.+)', line)
        reg = m.group(1)
        op = m.group(2)
        value = int(m.group(3))
        condition = m.group(4)
        if eval(condition, {}, registers):
            if op == 'inc':
                registers[reg] += value
            else:
                registers[reg] -= value
        max_ever = max(max_ever, registers[reg])
    return max(registers.values()), max_ever


if __name__ == '__main__':
    data = input('08').split('\n')

    solution(puzzle(data))
