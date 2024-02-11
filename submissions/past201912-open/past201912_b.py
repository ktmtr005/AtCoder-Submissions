N = int(input())
A = []
for i in range(N):
    A.append(int(input()))
for i in range(1, N):
    diff = A[i] - A[i - 1]
    if (diff == 0):
        print('stay')
    if (diff > 0):
        print('up {}'.format(diff))
    if (diff < 0):
        print('down {}'.format(-1 * diff))