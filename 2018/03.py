#! /usr/bin/env python3
from utils import *

data = input('03')

Claim = namedtuple('Claim', 'id x y w h')


def parse_claims(text):
    for claim in data.split('\n'):
        m = re.match(r'#(\d+) @ (\d+),(\d+): (\d+)x(\d+)', claim)
        yield Claim(id=int(m[1]),
                    x=int(m[2]),
                    y=int(m[3]),
                    w=int(m[4]),
                    h=int(m[5]))

def claims_grid(claims):
    grid = defaultdict(lambda: defaultdict(lambda: 0))

    for claim in claims:
        for a in range(claim.x, claim.x + claim.w):
            for b in range(claim.y, claim.y + claim.h):
                grid[a][b] += 1

    return grid


def problem1(data):
    grid = claims_grid(parse_claims(data))

    return sum(value > 1 for row in grid.values() for value in row.values())


def problem2(data):
    claims = list(parse_claims(data))
    grid = claims_grid(claims)

    for claim in claims:
        doesnt_overlap = all(grid[a][b] == 1
                             for a in range(claim.x, claim.x + claim.w)
                             for b in range(claim.y, claim.y + claim.h))

        if doesnt_overlap:
            return claim.id


solution(problem1(data), 110891)
solution(problem2(data), 297)
