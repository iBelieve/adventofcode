#! /usr/bin/env python3
from utils import *
import re

data = input('09')


def decompress(data, recurse=False, compute_length=False):
    decompressed = ''
    total_length = 0

    while data:
        if data[0] == '(':
            m = re.compile(r'\((\d+)x(\d+)\)').match(data)
            data = data[len(m[0]):]
            length = int(m[1])
            repeat = int(m[2])

            subset = data[:length]
            if recurse and '(' in subset:
                subset = decompress(subset, recurse=True, compute_length=compute_length)
            data = data[length:]
            if compute_length:
                if isinstance(subset, str):
                    subset = len(subset)
                total_length += subset * repeat
            else:
                decompressed += subset * repeat
        else:
            if compute_length:
                total_length += 1
            else:
                decompressed += data[0]
            data = data[1:]

    if compute_length:
        return total_length
    else:
        return decompressed

def decompress_recursive(data):
    return decompress(data, recurse=True)

def problem1(data):
    return decompress(data, compute_length=True)


def problem2(data):
    return decompress(data, recurse=True, compute_length=True)


example(decompress, 'A(1x5)BC', 'ABBBBBC')
example(decompress, '(6x1)(1x3)A', '(1x3)A')
example(decompress, 'X(8x2)(3x3)ABCY', 'X(3x3)ABC(3x3)ABCY')
solution(problem1(data), 138735)

example(decompress_recursive, 'X(8x2)(3x3)ABCY', 'XABCABCABCABCABCABCY')

solution(problem2(data), 11125026826)
