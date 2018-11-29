#! /usr/bin/env python3
from utils import *

data = input('07')

def supports_tls(ip):
    in_hypernet = False
    supports_tls = False
    for index in range(0, len(ip)):
        if ip[index] == '[':
            in_hypernet = True
        elif ip[index] == ']':
            in_hypernet = False
        elif index >= 2 and index + 1 < len(ip):
            maybe_supports_tls = (ip[index-2:index] == ip[index:index+2][::-1] and
                                  ip[index] != ip[index+1])

            if maybe_supports_tls:
                if in_hypernet:
                    return False
                else:
                    supports_tls = True
    return supports_tls


def supports_ssl(ip):
    aba = set()
    bab = set()
    in_hypernet = False

    for index in range(0, len(ip)):
        if ip[index] == '[':
            in_hypernet = True
        elif ip[index] == ']':
            in_hypernet = False
        elif index >= 1 and index + 1 < len(ip):
            is_pattern = (ip[index - 1] == ip[index + 1] and ip[index] != ip[index + 1])
            value = ip[index - 1:index+2]
            opposite = value[1] + value[0] + value[1]

            if is_pattern:
                if in_hypernet:
                    if opposite in aba:
                        return True
                    else:
                        bab.add(value)
                else:
                    if opposite in bab:
                        return True
                    else:
                        aba.add(value)
    return False


def problem1(data):
    return sum(map(supports_tls, data.split('\n')))


def problem2(data):
    return sum(map(supports_ssl, data.split('\n')))


example(supports_tls, 'abba[mnop]qrst', True)
example(supports_tls, 'abcd[bddb]xyyx', False)
example(supports_tls, 'aaaa[qwer]tyui', False)
example(supports_tls, 'ioxxoj[asdfgh]zxcvbn', True)
solution(problem1(data), 115)

example(supports_ssl, 'aba[bab]xyz', True)
example(supports_ssl, 'xyx[xyx]xyx', False)
example(supports_ssl, 'aaa[kek]eke', True)
example(supports_ssl, 'zazbz[bzb]cdb', True)
solution(problem2(data), 231)
