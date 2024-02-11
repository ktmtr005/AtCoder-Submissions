H, W = map(int, input().split())
for i in range(H):
    s = list(map(int, input().split()))
    S = [chr(64 + j).replace('@', '.') for j in s]
    print(''.join(S))