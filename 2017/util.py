def expect(actual, expected):
    if actual == expected:
        print(f'✔ {actual} == {expected}')
    else:
        print(f'✘ {actual} != {expected}')


def solution(answer, solution=None):
    if solution is not None:
        if answer == solution:
            print(f'Correct solution: {answer}')
        else:
            print(f'Wrong solution: {answer} != {solution}')
    else:
        print(f'Solution: {answer}')


def input(problem):
    with open(problem + '.txt') as f:
        return f.read().strip()

