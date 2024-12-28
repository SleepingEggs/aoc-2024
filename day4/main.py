import argparse
import time


parser = argparse.ArgumentParser(prog='aoc_2024_d4', description='Advent of code 2024 day 4')
parser.add_argument('filename')
parser.add_argument('-p', '--part2', action='store_true')

args = parser.parse_args()

line = 0
count = 0
my_map: list[list[str]] = []
starts: list[tuple[int, int]] = []
mids: list[tuple[int, int]] = []
ends = []
with open(args.filename, "r") as input_file:
    start_time = time.time()
    while True:
        char = input_file.read(1)
        if not char:
            break
        if count == 0:
            my_map.append([])
        if char == '\n':
            line += 1
            count = 0
            continue
        if char == "X":
            starts.append((count, line))
        if char == "S":
            ends.append((count, line))
        if char == "A":
            mids.append((count, line))
        count += 1
        my_map[line].append(char)


def generator_stringor(x, y, operator_x, operator_y):
    ret_string = ""
    for i in range(4):
        next_x = x + i * operator_x
        next_y = y + i * operator_y
        if next_x < 0 or next_y < 0 or next_x >= len(my_map) or next_y >= len(my_map[0]):
            ret_string += ""
            continue
        ret_string += my_map[next_y][next_x]
    return ret_string

nums = [-1, 0, 1]
def diagonal_generatoros(x, y):
    first_diag = ""
    first_diag_nums = zip(nums, nums)
    for num in first_diag_nums:
        num_x, num_y = num
        next_x = x + num_x
        next_y = y + num_y
        if next_x < 0 or next_y < 0 or next_x >= len(my_map) or next_y >= len(my_map[0]):
            first_diag += ""
            continue
        first_diag += my_map[next_y][next_x]

    second_diag = ""
    rev_nums = nums[:]
    rev_nums.reverse()
    second_diag_nums = zip(nums, rev_nums)
    for num in second_diag_nums:
        num_x, num_y = num
        next_x = x + num_x
        next_y = y + num_y
        if next_x < 0 or next_y < 0 or next_x >= len(my_map) or next_y >= len(my_map[0]):
            second_diag += ""
            continue
        second_diag += my_map[next_y][next_x]

    return first_diag, second_diag

result = 0
if args.part2:
    for mid in mids:
        mid_x, mid_y = mid
        first, second = diagonal_generatoros(mid_x, mid_y)
        if (first == "SAM" or first == "MAS") and (second =="SAM" or second == "MAS"):
            result += 1
else:
    for start in starts:
        start_x, start_y = start
        for num_x in nums:
            for num_y in nums:
                if generator_stringor(start_x, start_y, num_x, num_y) == "XMAS":
                    result += 1

print(result)
end_time = time.time()
print("time {0}".format(end_time - start_time))