#! /usr/bin/env python3

from util import expect, solution, input


def pairwise(iterable, offset=1):
    for index, a in enumerate(iterable):
        b = iterable[(index + offset) % len(iterable)]
        yield a, b


# The captcha requires you to review a sequence of digits (your puzzle input)
# and find the sum of all digits that match the next digit in the list. The
# list is circular, so the digit after the last digit is the first digit in the
# list.
def captcha1(string):
    return sum(int(char) for char, next in pairwise(string)
               if char == next)


# Now, instead of considering the next digit, it wants you to consider the
# digit halfway around the circular list. That is, if your list contains 10
# items, only include a digit in your sum if the digit 10/2 = 5 steps forward
# matches it. Fortunately, your list has an even number of elements.
def captcha2(string):
    assert len(string) % 2 == 0 # Is even
    
    return sum(int(char) for char, next in pairwise(string, len(string)//2)
               if char == next)


if __name__ == '__main__':
    # Part 1
    string = input('01')

    expect(captcha1, '1122', 3)
    expect(captcha1, '1111', 4)
    expect(captcha1, '1234', 0)
    expect(captcha1, '91212129', 9)
    solution(captcha1(string), 1223)

    # Part 2
    expect(captcha2, '1212', 6)
    expect(captcha2, '1221', 0)
    expect(captcha2, '123425', 4)
    expect(captcha2, '123123', 12)
    expect(captcha2, '12131415', 4)
    solution(captcha2(string), 1284)
