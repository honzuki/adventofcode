from utils import read_input
from part1 import part_1
from part2 import part_2


def main():
    input_data = read_input()
    print('part 1 result:', part_1(input_data))
    print('part 2 result:', part_2(input_data))


if __name__ == "__main__":
    main()