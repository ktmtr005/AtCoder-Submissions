N = int(input())
P = list(map(int, input().split()))
P1 = P[0]
P.sort(reverse=True)
if len(P) ==1:
    print("0")
    exit(0)
if P1 == P[0] and P[0] > P[1]:
    print("0")
else:
    print(P[0]+1-P1)