#! /usr/bin/env python3
from utils import *

KEYPAD_1 = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]
]

KEYPAD_2 = [
    [None, None, 1, None, None],
    [None, 2, 3, 4, None],
    [5, 6, 7, 8, 9],
    [None, 'A', 'B', 'C', None],
    [None, None, 'D', None, None]
]

OFFSETS = {
    'U': (-1, 0),
    'D': (1, 0),
    'L': (0, -1),
    'R': (0, 1)
}

data = input('02')


def problem(data, keypad, start):
    button = start
    code = ''

    for row in data.split('\n'):
        for char in row:
            offset = OFFSETS[char]

            new_button = [
                max(0, min(len(keypad) - 1, button[0] + offset[0])),
                max(0, min(len(keypad[1]) - 1, button[1] + offset[1]))
            ]

            if keypad[new_button[0]][new_button[1]] != None:
                button = new_button

        code += str(keypad[button[0]][button[1]])

    return code


def problem1(data):
    return problem(data, KEYPAD_1, [1, 1])


def problem2(data):
    return problem(data, KEYPAD_2, [2, 0])


example(problem1, 'ULL\nRRDDD\nLURDL\nUUUUD', '1985')
solution(problem1(data), '78293')
example(problem2, 'ULL\nRRDDD\nLURDL\nUUUUD', '5DB3')
solution(problem2(data), 'AC8C8')
