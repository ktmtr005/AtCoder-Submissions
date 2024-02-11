N = int(input())
S = [0] * N
A = [0] * N
for i in range(N):
    S[i], A[i] = input().split()
    A[i] = int(A[i])
min = A.index(min(A))
for i in range(N-min):
    print(S[min+i])
for i in range(min):
    print(S[i])