N = int(input())
S = input()
T, A = 0, 0
winner = None
for i in S:
    if i == 'T':
        T += 1
    else:
        A += 1
    if T > A:
        winner = 'T'
    if T < A:
        winner = 'A'
    print(winner)