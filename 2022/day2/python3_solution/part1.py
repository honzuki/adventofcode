from utils import get_game_score, map_choice


def part_1(input_data):
    return sum(
        map(get_game_score, [
            map(map_choice, game.split(' '))
            for game in input_data.split('\n') if game.strip()
        ]))
