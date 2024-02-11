N = int(input())
l, r = 1, N
while r - l > 1:
    center = (l + r) // 2
    print('? {}'.format(center))
    S = input()
    if S == '0':
        l = center
    else:
        r = center
print('! {}'.format(l))