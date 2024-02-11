import numpy as np
H, W, N = map(int, input().split())
g = np.zeros((H + 2, W + 2))
for i in range(N):
    A, B, C, D = map(int, input().split())
    g[A][B] += 1
    g[C + 1][D + 1] += 1
    g[A][D + 1] -= 1
    g[C + 1][B] -= 1
s = np.cumsum(np.cumsum(g, axis=0, dtype=int), axis=1, dtype=int)
for i in range(1, H + 1):
    print(*s[i][1: W + 1])