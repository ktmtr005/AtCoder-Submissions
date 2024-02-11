N = int(input())
S = input()
ans = -1
for i in range(N - 2):
    if [S[i], S[i + 1], S[i + 2]] == ['A', 'B', 'C']:
        ans = i + 1
        break
print(ans)