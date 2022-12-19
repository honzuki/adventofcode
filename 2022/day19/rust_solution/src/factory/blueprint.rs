use regex::Regex;
use std::str::FromStr;

use super::{Clay, Obsidian, Ore};

#[derive(Debug, Clone)]
pub struct Blueprint {
    pub(super) id: usize,
    pub(super) ore_robot: Ore,
    pub(super) clay_robot: Ore,
    pub(super) obsidian_robot: (Ore, Clay),
    pub(super) geode_robot: (Ore, Obsidian),
    pub(super) max_ore: Ore,
    pub(super) max_clay: Clay,
    pub(super) max_obsidian: Obsidian,
}

impl Blueprint {
    fn new(
        id: usize,
        ore_robot: Ore,
        clay_robot: Ore,
        obsidian_robot: (Ore, Clay),
        geode_robot: (Ore, Obsidian),
    ) -> Blueprint {
        let max_ore = ore_robot
            .max(clay_robot)
            .max(obsidian_robot.0)
            .max(geode_robot.0);
        let max_clay = obsidian_robot.1;
        let max_obsidian = geode_robot.1;

        Blueprint {
            id,
            ore_robot,
            clay_robot,
            obsidian_robot,
            geode_robot,
            max_ore,
            max_clay,
            max_obsidian,
        }
    }
}

impl FromStr for Blueprint {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.",
        ).unwrap();

        let cap = re.captures(s).unwrap();

        Ok(Blueprint::new(
            cap[1].parse().unwrap(),
            cap[2].parse().unwrap(),
            cap[3].parse().unwrap(),
            (cap[4].parse().unwrap(), cap[5].parse().unwrap()),
            (cap[6].parse().unwrap(), cap[7].parse().unwrap()),
        ))
    }
}
