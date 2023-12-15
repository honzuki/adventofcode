use initialization::{IBox, Step, StepErr};

mod initialization;

fn part_1(input: &str) -> u32 {
    input
        .trim()
        .split(',')
        .map(|step| initialization::hash(step) as u32)
        .sum()
}

fn part_2(input: &str) -> Result<u32, StepErr> {
    let mut boxes = vec![IBox::new(); 256];

    for step in input.trim().split(',') {
        let step: Step = step.parse()?;
        boxes[step.ibox() as usize].execute(step);
    }

    Ok(boxes
        .iter()
        .enumerate()
        .map(|(idx, ibox)| ibox.focusing_power() * (idx as u32 + 1))
        .sum())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = rust_shared_utils::read_puzzle()?;
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    #[test]
    fn part_1() {
        assert_eq!(super::part_1(INPUT), 1320);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(INPUT).unwrap(), 145);
    }
}
