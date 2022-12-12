import sys


def find_marker(s, n):
	i = 0
	while i <= len(s)-n:

		if len({x for x in s[i:i+n]}) == n:
			return i+n

		i+=1

for line in sys.stdin:
	print(find_marker(line.strip(), 4))
	print(find_marker(line.strip(), 14))
