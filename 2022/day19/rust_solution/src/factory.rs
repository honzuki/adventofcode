pub mod blueprint;

use blueprint::Blueprint;

pub type Ore = usize;
pub type Clay = usize;
pub type Obsidian = usize;
pub type Geode = usize;

type Time = usize;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct State {
    ore_robot: usize,
    clay_robot: usize,
    obsidian_robot: usize,
    geode_robot: usize,
    ore: Ore,
    clay: Clay,
    obsidian: Obsidian,
    geode: Geode,
    time: Time,
}

impl State {
    pub fn new(time: usize) -> State {
        State {
            ore_robot: 1,
            clay_robot: 0,
            obsidian_robot: 0,
            geode_robot: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            time,
        }
    }

    pub fn tick(&self) -> State {
        let mut next = self.clone();
        next.ore += next.ore_robot;
        next.clay += next.clay_robot;
        next.obsidian += next.obsidian_robot;
        next.geode += next.geode_robot;

        next.time -= 1;

        next
    }

    pub fn next(&self, blueprint: &Blueprint) -> Vec<State> {
        if self.time < 1 {
            return vec![];
        }
        let mut reach = vec![];

        // there is no need to generate more than we can consume
        if self.ore_robot < blueprint.max_ore {
            let mut state = self.clone();
            while state.time > 1 && state.ore < blueprint.ore_robot {
                state = state.tick();
            }
            state = state.tick();
            if state.time > 0 {
                state.ore -= blueprint.ore_robot;
                state.ore_robot += 1;
                reach.push(state);
            }
        }
        if self.clay_robot < blueprint.max_clay {
            let mut state = self.clone();
            while state.time > 1 && state.ore < blueprint.clay_robot {
                state = state.tick();
            }
            state = state.tick();
            if state.time > 0 {
                state.ore -= blueprint.clay_robot;
                state.clay_robot += 1;
                reach.push(state);
            }
        }
        if self.obsidian_robot < blueprint.max_obsidian && self.clay_robot > 0 {
            let mut state = self.clone();
            while state.time > 1
                && (state.ore < blueprint.obsidian_robot.0
                    || state.clay < blueprint.obsidian_robot.1)
            {
                state = state.tick();
            }
            state = state.tick();
            if state.time > 0 {
                state.ore -= blueprint.obsidian_robot.0;
                state.clay -= blueprint.obsidian_robot.1;
                state.obsidian_robot += 1;
                reach.push(state);
            }
        }

        if self.obsidian_robot > 0 {
            let mut state = self.clone();
            while state.time > 1
                && (state.ore < blueprint.geode_robot.0 || state.obsidian < blueprint.geode_robot.1)
            {
                state = state.tick();
            }
            state = state.tick();
            if state.time > 0 {
                state.ore -= blueprint.geode_robot.0;
                state.obsidian -= blueprint.geode_robot.1;
                state.geode_robot += 1;
                reach.push(state);
            }
        }

        if reach.len() == 0 {
            reach.push(self.tick());
        }
        reach
    }

    pub fn geode_count(&self) -> Geode {
        self.geode
    }
}

pub fn evaluate(state: &State, blueprint: &Blueprint) -> usize {
    state.geode_count() * blueprint.id
}

pub fn dfs(current: State, blueprint: &Blueprint) -> State {
    let mut best_state = current.clone();

    for next_state in current.next(blueprint) {
        let state = dfs(next_state, &blueprint);
        if state.geode > best_state.geode {
            best_state = state.clone()
        }
    }

    return best_state;
}
