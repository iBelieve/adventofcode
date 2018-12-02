#! /usr/bin/env python3

# Advent of Code 2018 Puzzle 2 - https://adventofcode.com/2018/day/2

from utils import *

data = input('02')


# To make sure you didn't miss any, you scan the likely candidate
# boxes again, counting the number that have an ID containing exactly
# two of any letter and then separately counting those with exactly
# three of any letter. You can multiply those two counts together to
# get a rudimentary checksum and compare it to what your device
# predicts.
def problem1(data):
    count_2 = 0
    count_3 = 0
    for word in data.split('\n'):
        letters = Counter(word)

        if any(value == 2 for key, value in letters.items()):
            count_2 += 1
        if any(value == 3 for key, value in letters.items()):
            count_3 += 1
    return count_2 * count_3


# The boxes will have IDs which differ by exactly one character at the
# same position in both strings. What letters are common between the
# two correct box IDs? (In the example above, this is found by
# removing the differing character from either ID, producing fgij.)
def problem2(data):
    lines = data.split('\n')

    def find_similar(word):
        for similar in lines:
            if sum(a == b for a, b in zip(word, similar)) == len(word) - 1:
                return similar

    for word in lines:
        similar = find_similar(word)
        if similar is not None:
            return ''.join(a for a, b in zip(word, similar) if a == b)


solution(problem1(data), 4920)
solution(problem2(data), 'fonbwmjquwtapeyzikghtvdxl')
