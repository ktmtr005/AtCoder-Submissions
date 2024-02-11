N = int(input())
S = input()
A, B, C = False, False, False
for i in range(N):
    if S[i] == "A":
        A = True
    if S[i] == "B":
        B = True
    if S[i] == "C":
        C = True
    if A and B and C:
        print(i+1)
        break