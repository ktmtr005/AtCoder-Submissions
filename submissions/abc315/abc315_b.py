import bisect
M = int(input())
D = list(map(int, input().split()))
cumulative_sum = [0]*(M+1)
cumulative_sum[0] = 0
for i in range(1,M+1):
    cumulative_sum[i] = cumulative_sum[i-1] + D[i-1]
center = (sum(D)+1)//2
if M == 1:
    print('{} {}'.format('1', center))
else:
    num = bisect.bisect_right(cumulative_sum, center)
    print('{} {}'.format(num, center - cumulative_sum[num-1]) if center != cumulative_sum[num-1] else '{} {}'.format(num-1, D[num-2]))