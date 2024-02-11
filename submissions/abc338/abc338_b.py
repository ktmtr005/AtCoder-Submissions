s = input()
dic = {}
for i in s:
    dic.setdefault(i, 0)
    dic[i] += 1
dic_sorted = sorted(dic.items(), key=lambda x: x[1], reverse=True)
freq_chars = [i for i, j in dic_sorted if j == dic_sorted[0][1]]
freq_chars.sort()
print(freq_chars[0])