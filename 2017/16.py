#! /usr/bin/env python3

# Day 10 - https://adventofcode.com/2017/day/10

from util import expect, solution, input
from collections import defaultdict
import re, itertools
from functools import reduce


def parse(move):
    cmd = move[0]
    args = [int(arg) if arg.isdigit() else arg for arg in move[1:].split('/')]

    def spin(line):
        count = args[0]
        sub = line[-count:]
        del line[-count:]
        return  sub + line

    def exchange(line):
        pos1 = args[0]
        pos2 = args[1]
        line[pos1], line[pos2] = line[pos2], line[pos1]
        return line
                
    def partner(line):
        pos1 = line.index(args[0])
        pos2 = line.index(args[1])
        line[pos1], line[pos2] = line[pos2], line[pos1]
        return line

    commands = {
        's': spin,
        'x': exchange,
        'p': partner
    }

    return commands[cmd]


def dance(moves, repeat=1, line='abcdefghijklmnop'):
    moves = list(map(parse, moves))

    line = list(line)
    seen_before = dict()
    i = 0

    while i < repeat:
        for cmd in moves:
            line = cmd(line)
        string = ''.join(line)
        if string in seen_before:

            i = repeat//i * i
        else:
            seen_before[string] = i
        i += 1
    return ''.join(line)


if __name__ == '__main__':
    example = ['s1', 'x3/4', 'pe/b']


    data = input('16').split(',')
    print(dance(example, line='abcde'))
    print(dance(example, repeat=2, line='abcde'))
    solution(dance(data), 'bkgcdefiholnpmja')
    solution(dance(data, repeat=1000000000))
