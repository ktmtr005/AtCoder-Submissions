S = input()
cnt = 0
for i in S:
    cnt += 1
    if i.isupper():
        print(cnt)