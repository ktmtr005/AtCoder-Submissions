N = int(input())
j = []
for i in range(1, 10):
    if N % i == 0: j.append(i)
ans = []
def check(i):
    for k in j:
        if i % (N // k) == 0:
            return str(k)
    return '-'
for i in range(N + 1):
    ans.append(check(i))
print(''.join(ans))