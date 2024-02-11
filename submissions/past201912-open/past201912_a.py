S = input()
for i in S:
    if (i.isdecimal() == False):
        print('error')
        exit()
print(int(S) * 2)