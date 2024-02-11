import sys
N, M, H, K = map(int, input().split())
S = input()
xy = {tuple(map(int, input().split())) for i in range(M)}
pos = [0, 0]
for i in S:
    if i == 'R':
        pos[0] += 1
    if i == 'L':
        pos[0] -= 1
    if i == 'U':
        pos[1] += 1
    if i == 'D':
        pos[1] -= 1
    H -= 1
    if H < 0:
        print('No')
        sys.exit()
    if H < K and tuple(pos) in xy:
        H = K
        xy.remove(tuple(pos))
print('Yes')