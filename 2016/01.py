#! /usr/bin/env python3
from utils import *

# X, Y
OFFSETS = [
    (0, 1),  # N
    (1, 0),  # E
    (0, -1), # S
    (-1, 0)  # W
]

data = input('01')

def changes(data):
    steps = data.split(', ')
    direction = 0

    for step in steps:
        turn = step[0]

        if turn == 'L':
            turn = -1
        elif turn == 'R':
            turn = 1
        else:
            raise Exception('Invalid turn: ' + turn)

        direction = (direction + turn) % 4
        distance = int(step[1:])

        yield OFFSETS[direction], distance


def problem1(data):
    x, y = 0, 0

    for (x_offset, y_offset), distance in changes(data):
        x += x_offset * distance
        y += y_offset * distance

    return abs(x) + abs(y)


def problem2(data):
    x, y = 0, 0
    locations = set('0,0')

    for (x_offset, y_offset), distance in changes(data):
        for _ in range(distance):
            x += x_offset
            y += y_offset
            location = f'{x},{y}'

            if location in locations:
                return abs(x) + abs(y)
            else:
                locations.add(location)

    raise Exception('Duplicate location not found')

example(problem1, 'R2, L3', 5)
example(problem1, 'R2, R2, R2', 2)
example(problem1, 'R5, L5, R5, R3', 12)

example(problem2, 'R8, R4, R4, R8', 4)

solution(problem1(data), 278)
solution(problem2(data), 161)
