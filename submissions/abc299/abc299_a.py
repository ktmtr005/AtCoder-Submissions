N = int(input())
S = input()
a = None
a2 = None
b = None
for i in range(N):
    if S[i] == '|' and a == None:
        a = i
    if S[i] == '|' and a != None:
        a2 = i
    if S[i] == '*':
        b = i
if a < b and b < a2:
    print('in')
else:
    print('out')