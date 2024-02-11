import math
T = int(input())
L, X, Y = map(int, input().split())
Q = int(input())
E = []
for i in range(Q):
    E.append(int(input()))
rad = list(map(lambda x: x * 2 / T * math.pi, E))
r = L / 2
for i in range(Q):
    y = r * math.sin(rad[i]) * -1
    z = r - r * math.cos(rad[i])
    dist = math.sqrt(X ** 2 + (Y - y) ** 2 + z ** 2)
    ans = math.degrees(math.asin(z / dist))
    print(ans)