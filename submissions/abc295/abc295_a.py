import re
N = int(input())
W = list(input().split())
for i in W:
    match = re.search(r'^(and|not|that|the|you)$', i)
    if match != None:
        print("Yes")
        exit()
print("No")