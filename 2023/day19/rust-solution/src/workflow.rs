use std::{collections::HashMap, ops::Range, str::FromStr};

use once_cell::sync::Lazy;
use paste::paste;
use regex::Regex;

use crate::part::{Part, PartRange};

#[derive(thiserror::Error, Debug)]
pub enum WorkflowErr {
    #[error("unknown worflow format")]
    UnknwonFormat,

    #[error("failed to parse condition value: {0}")]
    BadRightConditionValue(String),

    #[error("missing workflow: {0}")]
    MissingWorkflow(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Sign {
    Less,
    More,
}

impl Sign {
    fn check(self, left: u32, right: u32) -> bool {
        match self {
            Self::Less => left < right,
            Self::More => left > right,
        }
    }

    fn from_char(ch: char) -> Option<Self> {
        match ch {
            '>' => Some(Self::More),
            '<' => Some(Self::Less),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Category {
    X,
    M,
    A,
    S,
}

impl Category {
    fn extract_part(self, part: &Part) -> u32 {
        match self {
            Self::A => part.a,
            Self::S => part.s,
            Self::M => part.m,
            Self::X => part.x,
        }
    }

    fn from_char(ch: char) -> Option<Self> {
        Some(match ch {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Target {
    Reject,
    Accept,
    Map(String),
}

impl Target {
    fn from_str(s: &str) -> Self {
        let s = s.trim();
        match s {
            "A" => Self::Accept,
            "R" => Self::Reject,
            _ => Self::Map(s.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Condition {
    cateogry: Category,
    sign: Sign,
    right: u32,
    target: Target,
}

impl Condition {
    fn check(&self, part: &Part) -> bool {
        let left = self.cateogry.extract_part(part);
        self.sign.check(left, self.right)
    }

    fn split_range(&self, range: &PartRange) -> (Option<PartRange>, Option<PartRange>) {
        let break_range = |range: Range<u32>| match self.sign {
            Sign::Less => {
                let inside = if range.start < self.right {
                    Some(range.start..self.right.min(range.end))
                } else {
                    None
                };

                let outside = if range.end > self.right {
                    Some(self.right..range.end)
                } else {
                    None
                };

                (inside, outside)
            }
            Sign::More => {
                let inside = if range.end > (self.right + 1) {
                    Some(range.start.max(self.right + 1)..range.end)
                } else {
                    None
                };

                let outside = if range.start <= self.right {
                    Some(range.start..(self.right + 1))
                } else {
                    None
                };

                (inside, outside)
            }
        };

        macro_rules! edit_range {
            ($category: tt) => {{
                let (inside, outside) = break_range(range.$category.clone());
                let inside = inside.map(|irange| {
                    let mut prange = range.clone();
                    prange.$category = irange;
                    prange
                });

                let outside = outside.map(|irange| {
                    let mut prange = range.clone();
                    prange.$category = irange;
                    prange
                });

                (inside, outside)
            }};
        }

        match self.cateogry {
            Category::A => edit_range!(a),
            Category::M => edit_range!(m),
            Category::S => edit_range!(s),
            Category::X => edit_range!(x),
        }
    }
}

impl From<Condition> for PartRange {
    fn from(value: Condition) -> Self {
        let mut range = PartRange::new();

        macro_rules! edit_range {
            ($category: tt) => {
                paste! {
                    match value.sign {
                        Sign::Less => {
                            range.$category.end = value.right;
                        }
                        Sign::More => {
                            range.$category.start = (value.right + 1);
                        }
                    }
                }
            };
        }

        match value.cateogry {
            Category::A => edit_range!(a),
            Category::M => edit_range!(m),
            Category::S => edit_range!(s),
            Category::X => edit_range!(x),
        }

        range
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Workflow {
    name: String,
    conditions: Vec<Condition>,
    default: Target,
}

impl Workflow {
    fn apply(&self, part: &Part) -> &Target {
        self.conditions
            .iter()
            .find(|condition| condition.check(part))
            .map(|condition| &condition.target)
            .unwrap_or(&self.default)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workflows {
    data: HashMap<String, Workflow>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    Reject,
    Accept,
}

impl Workflows {
    pub fn process(&self, part: &Part) -> Result<Decision, WorkflowErr> {
        let mut current = self.get_workflow("in")?;
        loop {
            let target = current.apply(part);
            match target {
                Target::Reject => return Ok(Decision::Reject),
                Target::Accept => return Ok(Decision::Accept),
                Target::Map(name) => current = self.get_workflow(name)?,
            }
        }
    }
    fn get_workflow(&self, name: &str) -> Result<&Workflow, WorkflowErr> {
        self.data
            .get(name)
            .ok_or_else(|| WorkflowErr::MissingWorkflow(name.into()))
    }

    pub fn count_accepted_combinations(&self) -> Result<usize, WorkflowErr> {
        let mut combinations = 0;

        let mut frontier = vec![(self.get_workflow("in")?, PartRange::default())];
        while let Some((workflow, start_range)) = frontier.pop() {
            let mut outside = Some(start_range);
            for cond in workflow.conditions.iter() {
                let Some(range) = outside.as_ref() else { break };
                let (irange, orange) = cond.split_range(range);
                if let Some(irange) = irange {
                    match &cond.target {
                        Target::Accept => combinations += irange.size(),
                        Target::Map(name) => frontier.push((self.get_workflow(name)?, irange)),
                        Target::Reject => {}
                    };
                }

                outside = orange;
            }

            if let Some(outside) = outside {
                match &workflow.default {
                    Target::Accept => combinations += outside.size(),
                    Target::Map(name) => frontier.push((self.get_workflow(name)?, outside)),
                    Target::Reject => {}
                };
            }
        }

        Ok(combinations)
    }
}

impl FromStr for Workflow {
    type Err = WorkflowErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static WORKFLOW_RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^(\w+)\{((?:[xmas][><]\d+:\w+,)*)(\w+)\}$").unwrap());
        static CONDITION_RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"([xmas])([><])(\d+):(\w+)").unwrap());

        let parts = WORKFLOW_RE.captures(s).ok_or(WorkflowErr::UnknwonFormat)?;
        let name = parts.get(1).unwrap().as_str();
        let default = parts.get(3).unwrap().as_str();

        let conditions = parts.get(2).unwrap().as_str();
        let conditions = CONDITION_RE
            .captures_iter(conditions)
            .map(|cap| cap.extract())
            .map(|(_, [cateogry, sign, right, target])| {
                Ok(Condition {
                    cateogry: Category::from_char(cateogry.trim().chars().next().unwrap()).unwrap(),
                    sign: Sign::from_char(sign.trim().chars().next().unwrap()).unwrap(),
                    right: right
                        .trim()
                        .parse()
                        .map_err(|_| WorkflowErr::BadRightConditionValue(right.into()))?,
                    target: Target::from_str(target),
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            conditions,
            name: name.trim().into(),
            default: Target::from_str(default),
        })
    }
}

impl FromStr for Workflows {
    type Err = WorkflowErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .trim()
            .lines()
            .map(|line| {
                line.parse::<Workflow>()
                    .map(|workflow| (workflow.name.clone(), workflow))
            })
            .collect::<Result<HashMap<_, _>, _>>()?;

        Ok(Self { data })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        part::Part,
        workflow::{Decision, Target, Workflows},
    };

    use super::{Condition, Workflow};

    #[test]
    fn parse_workflow() {
        let input = r#"px{a<2006:qkq,m>2090:A,rfg}"#;
        let expected_output = Workflow {
            name: "px".into(),
            conditions: vec![
                Condition {
                    sign: super::Sign::Less,
                    cateogry: super::Category::A,
                    right: 2006,
                    target: Target::Map("qkq".into()),
                },
                Condition {
                    sign: super::Sign::More,
                    cateogry: super::Category::M,
                    right: 2090,
                    target: Target::Accept,
                },
            ],
            default: Target::Map("rfg".into()),
        };

        let output: Workflow = input.parse().unwrap();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn apply_workflow() {
        let workflow: Workflow = r#"px{a<2006:qkq,m>2090:A,rfg}"#.parse().unwrap();
        let part: Part = r#"{x=787,m=2655,a=3222,s=2876}"#.parse().unwrap();
        assert_eq!(*workflow.apply(&part), Target::Accept);

        let part: Part = r#"{x=787,m=2655,a=1222,s=2876}"#.parse().unwrap();
        assert_eq!(*workflow.apply(&part), Target::Map("qkq".into()));
    }

    #[test]
    fn process_part() {
        let workflows: Workflows = r#"in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
qs{s>3448:A,lnx}
lnx{m>1548:R,A}"#
            .parse()
            .unwrap();
        let part: Part = r#"{x=787,m=2655,a=1222,s=2876}"#.parse().unwrap();

        assert_eq!(workflows.process(&part).unwrap(), Decision::Reject);
    }
}
