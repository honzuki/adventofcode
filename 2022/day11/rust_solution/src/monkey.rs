use std::str::FromStr;

pub type WorryLevel = u64;

#[derive(Debug, Clone, Copy)]
pub enum TestType {
    Divisible {
        by: WorryLevel,
        success: usize,
        failure: usize,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Add(WorryLevel),
    Multiply(WorryLevel),
    Power(WorryLevel),
}

#[derive(Debug, Clone)]
pub struct Monkey {
    items: Vec<WorryLevel>,
    operator: Operator,
    test: TestType,
}

impl Monkey {
    pub fn new(mut items: Vec<WorryLevel>, operator: Operator, test: TestType) -> Monkey {
        items.reverse();
        Monkey {
            items: items,
            operator: operator,
            test: test,
        }
    }
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        if lines.len() < 6 {
            return Err("missing information".into());
        }

        let (_, line) = lines[1].split_once(':').unwrap();
        let items = line
            .split(',')
            .map(|value| value.trim().parse::<WorryLevel>().unwrap())
            .collect::<Vec<_>>();

        let (_, line) = lines[2].split_once("old").unwrap();
        let (operator, value) = line.trim().split_once(' ').unwrap();
        let operator = if value.trim() == "old" {
            Operator::Power(2)
        } else {
            let value = value.trim().parse::<WorryLevel>().unwrap();
            match operator.trim() {
                "+" => Operator::Add(value),
                "*" => Operator::Multiply(value),
                _ => return Err("unknwon operator".into()),
            }
        };

        let (_, line) = lines[3].split_once("by").unwrap();
        let by_count = line.trim().parse::<WorryLevel>().unwrap();

        let (_, line) = lines[4].split_once("monkey").unwrap();
        let success = line.trim().parse::<usize>().unwrap();

        let (_, line) = lines[5].split_once("monkey").unwrap();
        let failure = line.trim().parse::<usize>().unwrap();

        Ok(Monkey::new(
            items,
            operator,
            TestType::Divisible {
                by: by_count,
                success,
                failure,
            },
        ))
    }
}

#[derive(Debug, Clone)]
pub struct GameManager {
    monkeys: Vec<Monkey>,
    inspected: Vec<usize>,
    relief: WorryLevel,
    divide_factor: u64,
}

impl GameManager {
    pub fn execute_round(&mut self) {
        for idx in 0..self.monkeys.len() {
            while let Some(item) = self.monkeys[idx].items.pop() {
                let item = match self.monkeys[idx].operator {
                    Operator::Add(value) => item + value,
                    Operator::Multiply(value) => item * value,
                    Operator::Power(count) => item.pow(count as u32),
                };

                // divide by 3 due to relief
                let item = item / self.relief;
                let item = item % self.divide_factor;

                let to_monkey = match self.monkeys[idx].test {
                    TestType::Divisible {
                        by,
                        success,
                        failure,
                    } => {
                        let test = item % by;
                        if test == 0 {
                            success
                        } else {
                            failure
                        }
                    }
                };

                self.monkeys[to_monkey].items.push(item);
                self.inspected[idx] += 1;
            }
        }
    }

    pub fn monkey_business_level(&self) -> usize {
        let mut inspected_copy = self.inspected.clone();
        inspected_copy.sort();

        let mut res = 1;
        for inspected in inspected_copy.iter().rev().take(2) {
            res *= *inspected;
        }
        res
    }

    pub fn set_relief(&mut self, relief: WorryLevel) {
        self.relief = relief;
    }
}

impl FromStr for GameManager {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let monkeys = s
            .split("\n\n")
            .map(|raw_monkey| Monkey::from_str(raw_monkey).unwrap())
            .collect::<Vec<_>>();
        let monkey_count = monkeys.len();
        let mut divide_factor = 1;
        monkeys
            .iter()
            .map(|monkey| match monkey.test {
                TestType::Divisible {
                    by,
                    success: _,
                    failure: _,
                } => by,
            })
            .for_each(|by| divide_factor *= by);

        Ok(GameManager {
            monkeys,
            inspected: vec![0; monkey_count],
            relief: 3,
            divide_factor,
        })
    }
}
