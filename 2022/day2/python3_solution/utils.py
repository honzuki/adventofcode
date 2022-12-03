import sys


def map_choice(choice):
    if choice == 'A' or choice == 'X':
        return 0
    if choice == 'B' or choice == 'Y':
        return 1
    if choice == 'C' or choice == 'Z':
        return 2


def read_input():
    if len(sys.argv) < 2:
        print('missing input file path')
        exit(1)
    with open(sys.argv[1], 'r', encoding='utf-8') as file:
        return file.read()


def get_outcome(player1, player2):
    result = player1 - player2
    if result == 0:
        return 0
    if result == -1:
        return 2  # player2 wins
    if result == -2:
        return 1  # player1 wins
    return result


def get_game_score(game):
    game = [*game]
    score = game[1] + 1
    outcome = get_outcome(*game)
    if outcome == 0:
        score += 3
    if outcome == 2:
        score += 6
    return score