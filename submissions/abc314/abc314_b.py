n = int(input())
c = [0]*n
a = []
for i in range(n):
    c[i] = int(input())
    a.append(set(map(int, input().split())))
x = int(input())
for i in range(n):
    if x not in a[i]:
        c[i] += 100000
c_min = min(c)
winner = []
if c_min >= 100: 
    print('0')
    exit()
for i in range(n):
    if c[i] == c_min:
        winner.append(i+1)
print(len(winner))
print(*winner)