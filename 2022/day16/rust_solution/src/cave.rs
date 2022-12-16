use regex::Regex;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    str::FromStr,
};

type FlowRate = usize;
type Time = usize;
pub type Score = usize;
type Cost = Vec<Vec<Time>>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Valve {
    open: bool,
    rate: FlowRate,
}

impl Valve {
    fn new(rate: FlowRate) -> Valve {
        Valve {
            open: false,
            rate: rate,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct State {
    valves: Vec<Valve>,
    current: usize,
    time: Time,
    score: Score,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl State {
    fn next(&self, cost: &Cost) -> Vec<State> {
        if self.time == 0 {
            return vec![];
        }

        let mut states = vec![];
        for (idx, _) in self
            .valves
            .iter()
            .enumerate()
            .filter(|(_, valve)| !valve.open && valve.rate > 0)
        {
            if self.time < (cost[self.current][idx] + 1) {
                continue;
            }

            let mut next_state = self.clone();
            next_state.current = idx;
            next_state.time -= cost[self.current][idx] + 1; // +1 for opening the valve
            next_state.valves[idx].open = true;
            next_state.score += (next_state.time + 1) * self.valves[idx].rate;

            states.push(next_state);
        }

        states
    }
}

#[derive(Debug)]
pub struct Graph {
    start: State,
    cost: Cost,
}

impl Graph {
    fn new(valves: Vec<Valve>, cost: Cost, start_valve: usize) -> Graph {
        Graph {
            start: State {
                valves,
                current: start_valve,
                time: 29,
                score: 0,
            },
            cost,
        }
    }

    pub fn find_best_strategy(&self) -> Score {
        let mut frontier = Vec::new();
        frontier.push(self.start.clone());

        let mut max = 0;
        while let Some(state) = frontier.pop() {
            max = state.score.max(max);

            for next_state in state.next(&self.cost) {
                frontier.push(next_state);
            }
        }

        max
    }

    pub fn find_best_pair(&self) -> Score {
        let mut max = 0;

        let mut start = self.start.clone();
        start.time = 25; // we got an helper

        let mut frontier = Vec::new();
        frontier.push(start.clone());
        while let Some(state) = frontier.pop() {
            // find a state that opens all the remaining valves
            // and check if they yield a better value than max
            {
                let mut second_frontier = Vec::new();
                second_frontier.push(start.clone());

                'second_loop: while let Some(second_state) = second_frontier.pop() {
                    for valve_idx in 0..second_state.valves.len() {
                        if state.valves[valve_idx].open && second_state.valves[valve_idx].open {
                            continue 'second_loop;
                        }
                    }

                    max = max.max(second_state.score + state.score);

                    for next_state in second_state.next(&self.cost) {
                        second_frontier.push(next_state);
                    }
                }
            }

            for next_state in state.next(&self.cost) {
                frontier.push(next_state);
            }
        }

        max
    }
}

impl FromStr for Graph {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let valve_re: regex::Regex = Regex::new(r"Valve (\w{2}) has flow rate=(\d+);").unwrap();
        let tunnel_re: regex::Regex =
            Regex::new(r"Valve (\w{2}).*?valves{0,1} (\w+,{0,1} {0,1})+").unwrap();

        // map valve names to their index in the vec
        let mut idx_map: HashMap<String, usize> = HashMap::new();

        let mut valves = Vec::new();
        let mut tunnels = Vec::new();
        let mut cost = Vec::new();
        for cap in valve_re.captures_iter(s) {
            let name = cap[1].trim().into();
            let rate = cap[2].parse::<FlowRate>().unwrap();
            let valve = Valve::new(rate);

            idx_map.insert(name, valves.len());
            valves.push(valve);
            tunnels.push(vec![]);
        }

        // map tunnels
        for cap in tunnel_re.captures_iter(s) {
            let current_idx = idx_map.get(cap[1].trim()).unwrap();

            let mut names = vec![];
            if cap[0].contains("valves") {
                let (_, names_str) = cap[0].split_once("valves ").unwrap();
                for name in names_str.split(", ") {
                    names.push(name);
                }
            } else {
                names.push(&cap[2]);
            }

            for name in names {
                let idx = idx_map.get(name.trim()).unwrap();
                tunnels[*current_idx].push(*idx);
            }

            cost.push(vec![0; valves.len()]);
        }

        // calculate the cost of moving between valves
        for src in 0..valves.len() {
            for dst in 0..valves.len() {
                let mut visited: HashSet<usize> = HashSet::new();
                let mut frontier: BinaryHeap<(Reverse<Time>, usize)> = BinaryHeap::new();
                frontier.push((Reverse(0), src));

                while let Some((Reverse(time), idx)) = frontier.pop() {
                    if !visited.insert(idx) {
                        continue;
                    }

                    if idx == dst {
                        cost[src][dst] = time;
                        break;
                    }

                    for idx in &tunnels[idx] {
                        frontier.push((Reverse(time + 1), *idx));
                    }
                }
            }
        }

        Ok(Graph::new(valves, cost, *idx_map.get("AA").unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    fn get_test_graph() -> Graph {
        Graph::from_str(&fs::read_to_string("./test.input").unwrap()).unwrap()
    }

    #[test]
    fn test_parser() {
        get_test_graph();
    }
}
