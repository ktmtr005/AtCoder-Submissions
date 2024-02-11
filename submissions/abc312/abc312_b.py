N, M = map(int, input().split())
S = [input() for i in range(N)]
def takcode(i, j):
    for k in range(3):
        for l in range(3):
            if S[i+k][j+l] != "#" or S[i+k+6][j+l+6] != "#":
                return 0
    for k in range(4):
        if S[i+3][j+k] == "#" or S[i+5][j+k+5] == "#" or S[i+k][j+3] == "#" or S[i+k+5][j+5] == "#":
            return 0
    print("{} {}".format(i+1, j+1))
for i in range(N-8):
    for j in range(M-8):
        if S[i][j] == "#":
            takcode(i, j)
    