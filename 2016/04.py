#! /usr/bin/env python3
from utils import *
from collections import defaultdict, namedtuple
import re

ROOM_REGEX = re.compile(r'(?P<name>.*?)-(?P<id>\d+)(\[(?P<checksum>.*?)\])?')

Room = namedtuple('Room', 'name id checksum')

data = input('04')


def parse_room(line):
    m = ROOM_REGEX.match(line)
    return Room(m.group('name'), int(m.group('id')), m.group('checksum'))


def checksum_of(text):
    letters = defaultdict(lambda: 0)

    for letter in text:
        if letter.isalpha():
            letters[letter] += 1
    pairs = list(letters.items())
    pairs.sort(key=lambda pair: (-pair[1], pair[0]))
    return ''.join([letter for (letter, count) in pairs[:5]])


def is_real_room(room):
    if isinstance(room, str):
        room = parse_room(room)
    return checksum_of(room.name) == room.checksum


def rotate_letter(letter, count):
    if letter == '-':
        return ' '

    base = ord('a')
    offset = ord(letter.lower()) - base
    offset = (offset + count) % 26

    return chr(base + offset)


def decrypt_room_name(room):
    if isinstance(room, str):
        room = parse_room(room)
    return ''.join(rotate_letter(letter, room.id) for letter in room.name)


def problem1(data):
    rooms = [parse_room(line) for line in data.split('\n')]
    return sum(room.id for room in rooms if is_real_room(room))


def problem2(data):
    rooms = [parse_room(line) for line in data.split('\n')]
    return next(room.id for room in rooms
                if is_real_room(room) and decrypt_room_name(room) == 'northpole object storage')


example(checksum_of, 'aaaaa-bbb-z-y-x-123', 'abxyz')
example(is_real_room, 'aaaaa-bbb-z-y-x-123[abxyz]', True)
solution(problem1(data), 361724)

example(decrypt_room_name, 'qzmt-zixmtkozy-ivhz-343', 'very encrypted name')
solution(problem2(data), 482)
