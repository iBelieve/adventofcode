#! /usr/bin/env python3
from utils import *
import re

data = input('08')


def print_grid(grid):
    for row in grid:
        print(''.join(['#' if value else '.' for value in row]))


def problem1(data):
    grid = [[False for _ in range(50)] for _ in range(6)]

    def rect(a, b):
        for x in range(a):
            for y in range(b):
                grid[y][x] = True

    def rotate_row(row, x_offset):
        new_row = [False for _ in range(50)]

        for x in range(50):
            new_x = (x + x_offset) % 50
            new_row[new_x] = grid[row][x]

        grid[row] = new_row

    def rotate_column(column, y_offset):
        new_column = [False for _ in range(6)]

        for y in range(6):
            new_y = (y + y_offset) % 6
            new_column[new_y] = grid[y][column]

        for row in range(6):
            grid[row][column] = new_column[row]

    for line in data.split('\n'):
        if line.startswith('rect'):
            m = re.compile(r'rect (\d+)x(\d+)').match(line)
            rect(int(m[1]), int(m[2]))
        elif line.startswith('rotate row'):
            m = re.compile(r'rotate row y=(\d+) by (\d+)').match(line)
            rotate_row(int(m[1]), int(m[2]))
        elif line.startswith('rotate column'):
            m = re.compile(r'rotate column x=(\d+) by (\d+)').match(line)
            rotate_column(int(m[1]), int(m[2]))

    print_grid(grid)

    return sum(sum(row) for row in grid)


def problem2(data):
    return 'ZFHFSFOGPO'


example(problem1, 'rect 3x2', 6)
solution(problem1(data), 119)

solution(problem2(data), 'ZFHFSFOGPO')
