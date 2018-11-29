#! /usr/bin/env python3
from utils import *

data = input('03')


def is_valid_triangle(sides):
    if isinstance(sides, str):
        sides = sides.strip().split()
    sides = list(map(int, sides))
    sides.sort()
    return sides[0] + sides[1] > sides[2]


def problem1(data):
    return sum(map(is_valid_triangle, data.split('\n')))


def problem2(data):
    numbers = [line.strip().split() for line in data.split('\n')]
    triangles = []

    for row in range(0, len(numbers), 3):
        for column in range(3):
            triangles.append([numbers[row][column],
                              numbers[row + 1][column],
                              numbers[row + 2][column]])

    return sum(map(is_valid_triangle, triangles))


example(is_valid_triangle, '5 10 25', False)
solution(problem1(data), 983)

solution(problem2(data), 1836)
