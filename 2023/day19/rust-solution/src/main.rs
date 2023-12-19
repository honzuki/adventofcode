use part::Part;
use workflow::{WorkflowErr, Workflows};

use crate::workflow::Decision;

mod part;
mod workflow;

#[derive(Debug, Clone)]
struct Input {
    parts: Vec<Part>,
    workflows: Workflows,
}

fn parse_input(input: &str) -> Result<Input, Box<dyn std::error::Error>> {
    let (workflows, parts) = input.trim().split_once("\n\n").ok_or("bad input format")?;

    let parts = parts
        .trim()
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<Part>())
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Input {
        parts,
        workflows: workflows.parse()?,
    })
}

fn part_1(input: &Input) -> Result<u32, WorkflowErr> {
    input
        .parts
        .iter()
        .filter_map(|part| match input.workflows.process(part) {
            Ok(decision) => match decision {
                Decision::Accept => Some(Ok(part.sum_all())),
                Decision::Reject => None,
            },
            Err(err) => Some(Err(err)),
        })
        .sum::<Result<u32, _>>()
}

fn part_2(input: &Input) -> Result<usize, WorkflowErr> {
    input.workflows.count_accepted_combinations()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = rust_shared_utils::read_puzzle()?;
    let input = parse_input(&input)?;
    println!("part 1: {}", part_1(&input)?);
    println!("part 2: {}", part_2(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#;

    #[test]
    fn part_1() {
        let input = super::parse_input(INPUT).unwrap();
        assert_eq!(super::part_1(&input).unwrap(), 19114);
    }

    #[test]
    fn part_2() {
        let input = super::parse_input(INPUT).unwrap();
        assert_eq!(super::part_2(&input).unwrap(), 167409079868000);
    }
}
