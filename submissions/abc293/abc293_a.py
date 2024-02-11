S = input()
l = len(S)
ans = [0] * l
for i in range(1, l, 2):
    ans[i - 1] = S[i]
    ans[i] = S[i - 1]
print(''.join(ans))