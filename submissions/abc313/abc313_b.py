N, M = map(int, input().split())
G = [1] * N
for i in range(M):
    A, B = map(int, input().split())
    G[B-1] = 0
if sum(G) > 1:
    print("-1")
else:
    print(G.index(1)+1)