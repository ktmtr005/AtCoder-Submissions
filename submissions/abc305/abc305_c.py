H, W = map(int, input().split())
S = [input() for i in range(H)]
for i in range(H):
    for j in range(W):
        cnt = 0
        if S[i][j] == '.':
            try:
                S1 = S[i+1][j]
            except IndexError:
                pass
            else:
                if S1 == '#': cnt += 1
            try:
                S2 = S[i-1][j]
            except IndexError:
                pass
            else:
                if S2 == '#': cnt += 1
            try:
                S3 = S[i][j+1]
            except IndexError:
                pass
            else:
                if S3 == '#': cnt += 1
            try:
                S4 = S[i][j-1]
            except IndexError:
                pass
            else:
                if S4 == '#': cnt += 1
                if cnt >= 2:
            print('{} {}'.format(i+1, j+1))