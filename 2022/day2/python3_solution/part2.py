from utils import get_game_score, map_choice


def map_game_choice(raw_game):
    [opponent_choice, outcome] = raw_game.split(' ')
    opponent_choice = map_choice(opponent_choice)
    if outcome == 'X':
        return opponent_choice, (opponent_choice - 1) % 3
    if outcome == 'Y':
        return opponent_choice, opponent_choice
    if outcome == 'Z':
        return opponent_choice, (opponent_choice + 1) % 3


def part_2(input_data):
    return sum(
        map(get_game_score, [
            map_game_choice(game)
            for game in input_data.split('\n') if game.strip()
        ]))
