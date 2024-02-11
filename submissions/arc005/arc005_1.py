words = ('TAKAHASHIKUN', 'Takahashikun', 'takahashikun')
def main():
    N = int(input())
    w = list(input().split())
    w[-1] = w[-1].rstrip('.')
    cnt = 0
    for i in w:
        if i in words:
            cnt += 1
    print(cnt)
main()