import math
A, B, C = map(int, input().split())
r = math.gcd(math.gcd(A, B), C)
ans = (A // r - 1) + (B // r - 1) + (C // r - 1)
print(ans)