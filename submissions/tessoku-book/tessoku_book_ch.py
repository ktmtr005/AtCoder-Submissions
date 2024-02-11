import numpy as np
g = np.zeros((1501, 1501))
N = int(input())
for i in range(N):
    A, B, C, D = map(int, input().split())
    g[C][D] += 1
    g[C][B] -= 1
    g[A][D] -= 1
    g[A][B] += 1
s = np.cumsum(np.cumsum(g, axis=0, dtype=int), axis=1, dtype=int)
ans = 0
for i in range(1501):
    for j in range(1501):
        if s[i][j] >= 1: ans += 1
print(ans)