#! /usr/bin/env python3

# Day 10 - https://adventofcode.com/2017/day/10

from util import expect, solution, input
from collections import defaultdict
import re, itertools
from functools import reduce


def exec(cmd, args, regs, i):
    reg, val = args
    reg_val = regs.get(reg, 0)
    val = int(regs.get(val, val))

    if cmd == 'set':
        regs[reg] = val
    elif cmd == 'add':
        regs[reg] = reg_val + val
    elif cmd == 'mul':
        regs[reg] = reg_val * val
    elif cmd == 'mod':
        regs[reg] = reg_val % val
    elif cmd == 'jgz':
        if int(regs.get(reg, reg)) > 0:
            return i + val

    return i + 1


def duet(instructions):
    i = 0
    regs = {}
    last_freq = 0

    while i < len(instructions):
        line = instructions[i]
        cmd, args = line.split(' ', 1)
        args = args.split(' ')
        
        if cmd in ['snd', 'rcv']:
            val = args[0]
            val = regs.get(val, val)

            if cmd == 'snd':
                last_freq = val
            elif cmd == 'rcv':
                if val > 0:
                    return last_freq
            i += 1
        else:
            i = exec(cmd, args, regs, i)


def communicate(instructions):
    i1 = 0
    i2 = 0
    regs1 = {'p': 0}
    regs2 = {'p': 1}
    channel1 = []
    channel2 = []

    def exec_each(i, regs, cin, cout):
        line = instructions[i]
        cmd, args = line.split(' ', 1)
        args = args.split(' ')

        if cmd == 'snd':
            val = args[0]
            val = regs.get(val, val)
            cout.append(val)
            return i + 1
        elif cmd == 'rcv':
            reg = args[0]
            regs[reg] = cin.pop(0)
            return i + 1
        else:
            return exec(cmd, args, regs, i)

    def is_blocked(i, channel):
        return (i >= len(instructions) or
                (instructions[i].startswith('rcv') and len(channel) == 0))

    counter = 0

    while True:
        if not is_blocked(i1, channel1):
            i1 = exec_each(i1, regs1, channel1, channel2)
        elif not is_blocked(i2, channel2):
            i2 = exec_each(i2, regs2, channel2, channel1)
            if instructions[i2].startswith('snd'):
                counter += 1
        else:
            return counter


if __name__ == '__main__':
    example1 = input('18-example').split('\n')
    data = input('18').split('\n')

    expect(duet, example1, 4)
    solution(duet(data), 3188)

    solution(communicate(data), 7112)
