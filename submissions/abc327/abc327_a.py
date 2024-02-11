N = int(input())
S = input()
ans = False
for i in range(0, N - 1):
    if S[i:i + 2] == "ab" or S[i:i + 2] == "ba":
        ans = True
print("Yes" if ans == True else "No")