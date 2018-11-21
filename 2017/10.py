#! /usr/bin/env python3

# Day 10 - https://adventofcode.com/2017/day/10

from util import expect, solution, input
from collections import defaultdict
import re, itertools
from functools import reduce


def reverse_subset(array, start, length):
    end = start+length-1
    for offset in range(0, length//2):
        i = start + offset
        j = end - offset
        temp = array[i % len(array)]
        array[i % len(array)] = array[j % len(array)]
        array[j % len(array)] = temp


def puzzle(lengths, size=256, rounds=64):
    lengths += [17, 31, 73, 47, 23]
    array = list(range(0, size))

    i = 0
    skip_size = 0
    for round in range(0, rounds):
        for length in lengths:
            reverse_subset(array, i, length)
            i += length + skip_size
            skip_size += 1

    assert len(array) == 256
    dense_hash = [reduce(lambda a, b: a ^ b, array[i:i+16]) 
                  for i in range(0, 256, 16)]
    assert len(dense_hash) == 16
    print(dense_hash)
    hash = ''.join([format(n, '02x') for n in dense_hash])
    print(len(hash))
    assert len(hash) == 32
    print(hash)

    return array[0] * array[1]


if __name__ == '__main__':
    data = input('10')#.split(',')
    data = list(map(ord, data))

#    puzzle(list(map(ord, '')))

 #   print(puzzle([3, 4, 1, 5], 5))

    solution(puzzle(data))
