import re
import itertools
from collections import *
from itertools import *


def input(name):
    with open(f'{name}.txt') as f:
        return f.read().strip()


def example(func, input, expected):
    actual = func(input)
    input = repr(input)
    if len(input) > 30:
        input = '...'
    if actual == expected:
        print(f'✔ {func.__name__}({input}) = {actual}')
    else:
        print(f'✘ {func.__name__}({input}) = {actual} != {expected}')


def solution(answer, solution=None):
    if solution is not None:
        if answer == solution:
            print(f'Correct solution: {answer}')
        else:
            print(f'Wrong solution: {answer} != {solution}')
    else:
        print(f'Solution: {answer}')


def first_duplicate(items):
    already_seen = set()

    for item in items:
        if item in already_seen:
            return item
        else:
            already_seen.add(item)

    return None
