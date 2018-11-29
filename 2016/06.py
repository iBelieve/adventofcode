#! /usr/bin/env python3
from utils import *
from collections import defaultdict

data = input('06')

EXAMPLE = '''
eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar
'''.strip()

def problem(data, reverse):
    lines = data.split('\n')
    word = ''

    for column in range(len(lines[0])):
        letters = defaultdict(lambda: 0)

        for line in lines:
            letter = line[column]
            letters[letter] += 1
        pairs = list(letters.items())
        pairs.sort(key=lambda pair: pair[1], reverse=reverse)
        most_common_letter = pairs[0][0]
        word += most_common_letter

    return word


def problem1(data):
    return problem(data, reverse=True)


def problem2(data):
    return problem(data, reverse=False)


example(problem1, EXAMPLE, 'easter')
solution(problem1(data), 'ursvoerv')
example(problem2, EXAMPLE, 'advent')
solution(problem2(data), 'vomaypnn')
