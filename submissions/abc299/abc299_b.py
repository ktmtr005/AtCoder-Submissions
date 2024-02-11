N, T = map(int, input().split())
C = list(map(int, input().split()))
R = list(map(int, input().split()))
d = {}
d2 = {}
for i in range(N):
    if C[i] in d:
        if R[i] > d[C[i]]:
            d[C[i]] = R[i]
            d2[C[i]] = i + 1
    else:
        d[C[i]] = R[i]
        d2[C[i]] = i + 1
if T in d:
    print(d2[T])
else:
    print(d2[C[0]])