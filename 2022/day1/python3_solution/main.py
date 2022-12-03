import sys


def read_input():
    if len(sys.argv) < 2:
        print('missing input file path')
        exit(1)
    with open(sys.argv[1], 'r', encoding='utf-8') as file:
        return file.read()


def get_calories_list(input_data):
    return [
        sum([int(item) for item in calories.split('\n') if item])
        for calories in input_data.split('\n\n')
    ]


def part_1(input_data):
    return max(get_calories_list(input_data))


def part_2(input_data):
    return sum(sorted(get_calories_list(input_data), reverse=True)[:3])


def main():
    input_data = read_input()
    print("part 1 result: {}".format(part_1(input_data)))
    print("part 1 result: {}".format(part_2(input_data)))


if __name__ == "__main__":
    main()