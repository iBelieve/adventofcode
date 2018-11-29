#! /usr/bin/env python3
from utils import *
from hashlib import md5

data = input('05')


def hash_of(door_id, index):
    m = md5()
    m.update(f'{door_id}{index}'.encode('utf8'))
    return m.digest().hex()


def problem1(door_id):
    password = ''
    index = 0
    while len(password) < 8:
        hash = hash_of(door_id, index)
        if hash.startswith('00000'):
            password += hash[5]
            print(password.ljust(8, '_'), index, hash)
        index += 1

    return password


def problem2(door_id):
    password = ['_' for _ in range(8)]
    index = 0
    count = 0

    while count < 8:
        hash = hash_of(door_id, index)
        pass_index = int(hash[5], 16)
        if hash.startswith('00000') and pass_index < len(password) and password[pass_index] == '_':
            password[pass_index] = hash[6]
            count += 1
            print(''.join(password), index, hash)
        index += 1

    return ''.join(password)


example(problem1, 'abc', '18f47a30')
solution(problem1(data), 'f77a0e6e')

example(problem2, 'abc', '05ace8e3')
solution(problem2(data), '999828ec')
