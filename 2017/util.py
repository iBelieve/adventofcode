def expect(func, input, expected):
    actual = func(input)
    input = str(input)
    if len(input) > 30:
        input = '...'
    if actual == expected:
        print(f'✔ {func.__name__}({input}) = {actual}')
    else:
        print(f'✘ {func.__name__}({input}) = {actual} != {expected}')


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
        return f.read().strip('\n')

