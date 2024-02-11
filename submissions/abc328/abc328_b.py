# abc238/b
n = int(input())
d = list(map(int, input().split()))
ans = 0
for i in range(n):
    month_str = str(i + 1)
    for j in range(1, d[i] + 1):
        day_str = str(j)
        if len(set(month_str + day_str)) == 1:
            ans += 1
print(ans)