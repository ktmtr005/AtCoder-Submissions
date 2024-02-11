N = int(input())
A = list(map(int, input().split()))
ans = "Yes"
for i in A:
    if i != A[0]:
        ans = "No"
print(ans)