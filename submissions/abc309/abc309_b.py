import copy
N = int(input())
A = [list(input()) for i in range(N)]
ans = copy.deepcopy(A)
for i in range(N-1):
    ans[0][i+1] = A[0][i]
    ans[N-1][i] = A[N-1][i+1]
    ans[i][0] = A[i+1][0]
    ans[i+1][N-1] = A[i][N-1]
for i in range(N):
    print(''.join(ans[i]))