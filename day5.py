import sys
import re

read_matrix = True
matrix = []
moves = []
stacks = {}
stacks2 = {}

for line in sys.stdin:
	if line.strip():
		if read_matrix:
			matrix.append(line)
		else:
			moves.append(line)

	else:
		read_matrix = False

indexes_line = matrix[len(matrix)-1]

for i in range(len(indexes_line)):
	index = indexes_line[i]
	if index.strip():
		j = len(matrix)-2
		stacks[index] = []
		stacks2[index] = []

		while j >= 0:
			if len(matrix[j]) > i and matrix[j][i].strip():
				stacks[index].append(matrix[j][i])
				stacks2[index].append(matrix[j][i])
			j -= 1

def process_move(move):
	pattern = r'^move (\d+) from (\d+) to (\d+)$'
	_count, source, destination = re.match(pattern, move).group(1, 2, 3)
	count = int(_count)

	# CrateMover 9000
	for _ in range(count):
		stacks[destination].append(stacks[source].pop())

	# CreateMover 9001
	stacks2[destination].extend(stacks2[source][-count:])
	stacks2[source] = stacks2[source][:len(stacks2[source])-count]


for move in moves:
	process_move(move) 

print(''.join([stack.pop() for stack in stacks.values()]))
print(''.join([stack.pop() for stack in stacks2.values()]))
