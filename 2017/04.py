#! /usr/bin/env python3

from util import expect, solution, input


# A new system policy has been put in place that requires all accounts to use a
# passphrase instead of simply a password. A passphrase consists of a series of
# words (lowercase letters) separated by spaces.
#
# To ensure security, a valid passphrase must contain no duplicate words.
#
# For example:
#
# - aa bb cc dd ee is valid.
# - aa bb cc dd aa is not valid - the word aa appears more than once.
# - aa bb cc dd aaa is valid - aa and aaa count as different words.
def is_valid1(password):
    words = password.split(' ')
    seen = set()
    for word in words:
        if word in seen:
            return False
        seen.add(word)
    return True


# For added security, yet another system policy has been put in place. Now, a
# valid passphrase must contain no two words that are anagrams of each other -
# that is, a passphrase is invalid if any word's letters can be rearranged to
# form any other word in the passphrase.
#
# For example:
#
# - abcde fghij is a valid passphrase.
# - abcde xyz ecdab is not valid - the letters from the third word can be 
#   rearranged to form the first word.
# - a ab abc abd abf abj is a valid passphrase, because all letters need to be 
#   used when forming another word.
# - iiii oiii ooii oooi oooo is valid.
# - oiii ioii iioi iiio is not valid - any of these words can be rearranged to
#   form any other word.
def is_valid2(password):
    words = password.split(' ')
    seen = set()
    for word in words:
        word = ''.join(sorted(word))
        if word in seen:
            return False
        seen.add(word)
    return True


if __name__ == '__main__':
    passwords = input('04').split('\n')

    # Part 1
    expect(is_valid1, 'aa bb cc dd ee', True)
    expect(is_valid1, 'aa bb cc dd aa', False)
    solution(sum(map(is_valid1, passwords)), 455)

    # Part 2
    expect(is_valid2, 'abcde fghij', True)
    expect(is_valid2, 'abcde xyz ecdab', False)
    solution(sum(map(is_valid2, passwords)), 186)
