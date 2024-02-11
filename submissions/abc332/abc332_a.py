N, S, K = map(int, input().split())
goods = [list(map(int, input().split())) for i in range(N)] # P, Q
total = 0
for i in goods:
    total += i[0] * i[1]
if total < S:
    total += K
print(total)