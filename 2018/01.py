#! /usr/bin/env python3

# Advent of Code 2018 Puzzle 1 - https://adventofcode.com/2018/day/1

from utils import *

data = input('01')

def split(text):
    if '\n' in text:
        return text.split('\n')
    else:
        return text.split(', ')

# After feeling like you've been falling for a few minutes, you look
# at the device's tiny screen. "Error: Device must be calibrated
# before first use. Frequency drift detected. Cannot maintain
# destination lock." Below the message, the device shows a sequence of
# changes in frequency (your puzzle input). A value like +6 means the
# current frequency increases by 6; a value like -3 means the current
# frequency decreases by 3.
#
# Starting with a frequency of zero, what is the resulting frequency
# after all of the changes in frequency have been applied?
def problem1(data):
    return sum(int(n) for n in split(data))


# You notice that the device repeats the same frequency change list
# over and over. To calibrate the device, you need to find the first
# frequency it reaches twice.
def problem2(data):
    changes = [int(n) for n in split(data)]
    numbers = set([0])
    number = 0

    while True:
        for change in changes:
            number += change
            if number in numbers:
                return number
            numbers.add(number)


example(problem1, '+1, -2, +3, +1', 3)
solution(problem1(data), 500)
example(problem2, '+3, +3, +4, -2, -4', 10)
solution(problem2(data), 709)
