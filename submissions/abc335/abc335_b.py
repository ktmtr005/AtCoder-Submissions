#!/usr/bin/env python3
# from typing import *
# def solve(N: int) -> Any:
def solve(N):
    l = []
    for i in range(N+1):
        for j in range(N+1):
            for k in range(N+1):
                if (i + j + k <= N):
                    l.append([i, j, k])
    return l
# generated by oj-template v4.8.1 (https://github.com/online-judge-tools/template-generator)
def main():
    N = int(input())
    ans = solve(N)
    for i in ans:
        print(*i)
if __name__ == '__main__':
    main()