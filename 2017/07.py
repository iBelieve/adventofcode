#! /usr/bin/env python3

# Day 7 - https://adventofcode.com/2017/day/7

from util import expect, solution, input
from collections import defaultdict
import re


class WrongWeightError(BaseException):
    pass


def build_tree(lines):
    tree = dict()
    children = set()
    weights = dict()

    for line in lines:
        m = re.match(r'(\w+)\s\((\d+)\)(?:\s->\s(.+))?', line)
        name = m.group(1)
        weight = m.group(2)
        direct_children = m.group(3).split(', ') if m.group(3) is not None else []

        tree[name] = direct_children
        weights[name] = int(weight)
        children.update(direct_children)

    roots = [name for name in tree.keys() if name not in children]
    assert len(roots) == 1

    return roots[0], tree, weights


def odd_one_out(list):
    """
    Return the index and value of a single unique list item, if one exists,
    along with the "normal" value in the list
    """
    for index, value in enumerate(list):
        if list.count(value) == 1:
            normal = next(other for other in list if other != value)
            return index, normal, value
    return None, None, None


def puzzle1(lines):
    root, _, _ = build_tree(lines)
    return root


def puzzle2(lines):
    root, tree, weights = build_tree(lines)

    def get_weight(name):
        weight = weights[name]
        children = tree[name]
        children_weights = list(map(get_weight, children))
        wrong_index, right, wrong = odd_one_out(children_weights) 
        if wrong is not None:
            wrong_child = weights[children[wrong_index]]
            right_child = wrong_child - wrong + right
            raise WrongWeightError(dict(wrong=wrong_child, right=right_child))
        return weight + sum(children_weights)
    
    try:
        get_weight(root)
        return None
    except WrongWeightError as error:
        return error.args[0]['right']


if __name__ == '__main__':
    example = input('07-example').split('\n')
    data = input('07').split('\n')

    # Part 1
    expect(puzzle1, example, 'tknk')
    solution(puzzle1(data), 'hlqnsbe')

    # Part 2
    expect(puzzle2, example, 60)
    solution(puzzle2(data), 1993)
