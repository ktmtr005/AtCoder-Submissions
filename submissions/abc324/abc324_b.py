N = int(input())
flag = True
while (N % 2 == 0):
    N = N // 2
while (N % 3 == 0):
    N = N // 3
if N != 1:
    flag = False
print("Yes" if flag == True else "No")