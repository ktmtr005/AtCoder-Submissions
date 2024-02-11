N, P, Q = map(int, input().split())
D = min(list(map(int, input().split())))
if D+Q > P:
    print(P)
else:
    print(D+Q)