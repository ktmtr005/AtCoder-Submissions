N, L = map(int, input().split())
K = int(input())
A = list(map(int, input().split()))
def solve(M):
    count = 0
    pre = 0
    for i in range(N):
        if (A[i] - pre >= M) and (L - A[i] >= M):
            count += 1
            pre = A[i]
    if count >= K:
        return True
    return False
# 二分探索
left = -1
right = L + 1
while (right - left > 1):
    mid = left + (right - left) // 2
    if (solve(mid) == False):
        right = mid
    else:
        left = mid
print(left)