N = int(input())
A = list(map(int, input().split()))
insert = []
for i in range(1, N):
    if A[i] - A[i-1] > 1:
        insert.append(list(range(A[i-1]+1, A[i])))
    elif A[i] - A[i-1] < -1:
        insert.append(list(reversed(range(A[i]+1, A[i-1]))))
    else:
        insert.append('')
insert.append('')
for i in range(0, N-1):
    print(A[i], end=' ')
    if insert[i] != '':
        print(*insert[i], end=' ')
print(A[-1])