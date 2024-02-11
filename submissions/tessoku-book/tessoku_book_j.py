import itertools
N = int(input())
A = list(map(int, input().split()))
P = list(itertools.accumulate(A, func=max))
Q = list(reversed(list(itertools.accumulate(reversed(A), func=max))))
D = int(input())
L = [0 for _ in range(D)]
R = [0 for _ in range(D)]
for i in range(D):
    L[i], R[i] = map(int, input().split())
for i in range(D):
    print(max(P[L[i] - 2], Q[R[i]]))