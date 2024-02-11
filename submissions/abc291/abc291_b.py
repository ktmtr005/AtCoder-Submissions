from statistics import mean
N = int(input())
X = list(map(int, input().split()))
X.sort()
del X[0:N]
del X[-N:]
ans = mean(X)
print(ans)