import sys
N = int(input())
S  = input()
ans = -1
tmp = 0
flag_o = False
flag_m= False
if 'o' not in S:
    print(ans)
    sys.exit()
for i in range(N):
    if S[i] == 'o':
        flag_o = True
        tmp += 1
    else:
        flag_m = True
        if ans < tmp:
            ans = tmp
        tmp = 0
if flag_m == True and ans < tmp:
    ans = tmp
print(ans)