S = str(input())
ans = ''
for i in S[::-1]:
    if i == '.':
        break
    ans += i
print(ans[::-1])