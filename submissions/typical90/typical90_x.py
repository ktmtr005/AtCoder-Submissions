N, K = map(int, input().split())
A = list(map(int, input().split()))
B = list(map(int, input().split()))
ans = False
diff = sum([abs(A[i] - B[i]) for i in range(N)])
if diff > K:
    ans = False
elif (K - diff) % 2 != 0:
    ans = False
elif (K - diff) % 2 == 0:
    ans = True
print('Yes' if ans == True else 'No')