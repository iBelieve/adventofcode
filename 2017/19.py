#! /usr/bin/env python3

# Day 10 - https://adventofcode.com/2017/day/10

from util import expect, solution, input
from collections import defaultdict
import re, itertools
from functools import reduce

MOVES = {
    'down': (1, 0),
    'up': (-1, 0),
    'left': (0, -1),
    'right': (0, 1)
}


def puzzle(grid):
    row = 0
    col = grid[0].index('|')
    direction = 'down'

    string = ''
    steps = 0

    while 0 <= row < len(grid) and 0 <= col < len(grid[0]):
        char = grid[row][col]
        
        if char == '+':
            if direction == 'down' or direction == 'up':
                if grid[row][col - 1] != ' ':
                    direction = 'left'
                elif grid[row][col + 1] != ' ':
                    direction = 'right'
                else:
                    raise f'Bad turn at {row}, {col}'
            else:
                if grid[row - 1][col] != ' ':
                    direction = 'up'
                elif grid[row + 1][col] != ' ':
                    direction = 'down'
                else:
                    raise f'Bad turn at {row}, {col}'
        elif char.isalpha():
            string += char
        elif char == ' ':
            return string, steps

        row_change, col_change = MOVES[direction]

        row += row_change
        col += col_change

        steps += 1

if __name__ == '__main__':
    data = [list(line) for line in input('19').split('\n')]

    string, steps = puzzle(data)

    solution(string, 'SXPZDFJNRL')
    solution(steps)
