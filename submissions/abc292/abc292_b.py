N, Q = map(int, input().split())
yellow =[0] * (N+1)
red = [False] * (N+1)
for i in range(Q):
    event, player = map(int, input().split())
    if event == 1:
        yellow[player] += 1
        if yellow[player] >= 2:
            red[player] = True
    elif event == 2:
        red[player] = True
    else:
        print('Yes' if red[player] == True else 'No')