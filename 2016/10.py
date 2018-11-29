#! /usr/bin/env python3
from utils import *
from collections import defaultdict
import re

data = input('10')


def problem(data, problem1):
    bots = defaultdict(lambda: [])
    outputs = {}
    bot_actions = {}
    instructions = data.split('\n')

    for instruction in instructions:
        if instruction.startswith('value'):
            m = re.compile(r'value (\d+) goes to bot (\d+)').match(instruction)
            value = int(m[1])
            bot = int(m[2])

            bots[bot].append(value)
        elif instruction.startswith('bot'):
            m = re.compile(r'bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)').match(instruction)
            src_bot = int(m[1])
            low_target = m[2]
            low = int(m[3])
            high_target = m[4]
            high = int(m[5])

            bot_actions[src_bot] = [low_target, low, high_target, high]

    while bots:
        bot, values = next(((bot, values) for bot, values in bots.items() if len(values) == 2), (None, None))

        if bot is None:
            break

        del bots[bot]

        action = bot_actions[bot]

        low = min(values)
        high = max(values)

        if problem1 and low == 17 and high == 61:
            return bot

        if action[0] == 'output':
            outputs[action[1]] = low
        else:
            bots[action[1]].append(low)

        if action[2] == 'output':
            outputs[action[3]] = high
        else:
            bots[action[3]].append(high)

    if problem1:
        return None
    else:
        return outputs[0] * outputs[1] * outputs[2]


def problem1(data):
    return problem(data, problem1=True)


def problem2(data):
    return problem(data, problem1=False)


solution(problem1(data), 93)
solution(problem2(data), 47101)
