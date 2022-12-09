import sys
import re

fully_contained_count = 0;
overlap_count = 0

for line in sys.stdin:
	pattern = "^(\\d+)-(\\d+),(\\d+)-(\\d+)$"
	_l1, _r1, _l2, _r2 = re.match(pattern, line).group(1, 2, 3, 4)
	l1, r1, l2, r2 = int(_l1), int(_r1), int(_l2), int(_r2)
	first_into_second = l2 <= l1 and r1 <= r2
	second_into_first = l1 <= l2 and r2 <= r1

	if first_into_second or second_into_first:
		fully_contained_count += 1

	if l1 <= l2 <= r1 or l1 <= r2 <= r1 or l2 <= l1 <= r2 or l2 <= r1 <= r2: 
		overlap_count += 1

print(fully_contained_count)
print(overlap_count)