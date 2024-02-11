N, M = map(int, input().split())
S = input()
T = input()
ans = [False, False] # [prefix, suffix]
if T[:N] == S:
    ans[0] = True
if T[N * -1:] == S:
    ans[1] = True
if ans == [True, True]: print('0')
elif ans == [True, False]: print('1')
elif ans == [False, True]: print('2')
else: print('3')