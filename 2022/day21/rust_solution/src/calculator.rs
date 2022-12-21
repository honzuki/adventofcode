use std::{collections::HashMap, hash::Hash};

type Numbers = HashMap<String, Number>;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operation {
    pub fn new(raw: &str) -> Result<Operation, &'static str> {
        let op = match raw {
            "+" => Operation::Add,
            "-" => Operation::Sub,
            "*" => Operation::Mul,
            "/" => Operation::Div,
            _ => return Err("unknown operation"),
        };

        Ok(op)
    }

    pub fn calculate(&self, left: i64, right: i64) -> i64 {
        match self {
            Self::Add => left + right,
            Self::Sub => left - right,
            Self::Mul => left * right,
            Self::Div => left / right,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Equation {
    left: String,
    op: Operation,
    right: String,
}

impl Equation {
    fn new(left: String, op: Operation, right: String) -> Equation {
        Equation { left, op, right }
    }

    fn evaluate(&self, numbers: &Numbers) -> Option<i64> {
        match (numbers.get(&self.left), numbers.get(&self.right)) {
            (Some(left), Some(right)) => match (left.get_value(), right.get_value()) {
                (Some(left), Some(right)) => Some(self.op.calculate(left, right)),
                _ => None,
            },
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Number {
    name: String,
    value: Option<i64>,
    eq: Option<Equation>,
}

impl Number {
    fn create_pure(name: String, value: i64) -> Number {
        Number {
            name,
            value: Some(value),
            eq: None,
        }
    }

    fn create_complex(name: String, eq: Equation) -> Number {
        Number {
            name,
            value: None,
            eq: Some(eq),
        }
    }

    pub fn calc_value(&mut self, numbers: &Numbers) -> Option<i64> {
        match self.value {
            Some(value) => Some(value),
            None => {
                let eq = self.eq.as_ref().unwrap();
                match eq.evaluate(numbers) {
                    Some(value) => {
                        self.value = Some(value);
                        Some(value)
                    }
                    None => None,
                }
            }
        }
    }

    pub fn get_value(&self) -> Option<i64> {
        self.value
    }

    pub fn set_value(&mut self, value: i64) {
        self.value = Some(value);
    }

    pub fn set_op(&mut self, op: &str) -> Result<(), &'static str> {
        if let Some(eq) = &mut self.eq {
            eq.op = Operation::new(op)?;
        }

        Ok(())
    }

    pub fn get_dependencies(&self) -> Vec<&str> {
        match &self.eq {
            Some(eq) => vec![&eq.left, &eq.right],
            None => vec![],
        }
    }

    pub fn resolve_dep_list(&mut self, numbers: &mut Numbers) {
        if let Some(_) = self.calc_value(numbers) {
            return;
        }

        // resolve dependencies
        for &dep in self.get_dependencies().iter() {
            let mut num = numbers.remove(dep).unwrap();

            num.resolve_dep_list(numbers);

            numbers.insert(dep.clone().into(), num);
        }

        self.calc_value(numbers).unwrap();
    }

    pub fn from_str(s: &str) -> Result<Number, &'static str> {
        let parts: Vec<_> = s.split(' ').collect();
        if parts.len() < 1 {
            return Err("invalid equation");
        }

        let (name, _) = parts[0].split_once(':').unwrap();
        let name = name.trim().into();

        if parts.len() == 2 {
            let value: i64 = parts[1]
                .parse()
                .map_err(|_| "invalid number in pure equation")?;

            return Ok(Number::create_pure(name, value));
        } else if parts.len() != 4 {
            return Err("invalid equation");
        }

        let left = parts[1].trim().into();
        let op = Operation::new(parts[2])?;
        let right = parts[3].trim().into();

        Ok(Number::create_complex(name, Equation::new(left, op, right)))
    }
}

pub fn build_numbers_map(input: &str) -> Result<Numbers, &'static str> {
    let mut numbers = HashMap::new();

    for line in input.lines() {
        let mut number = Number::from_str(line)?;
        number.calc_value(&mut numbers);
        numbers.insert(number.name.clone(), number);
    }

    Ok(numbers)
}

pub fn find_controlled_value(
    controlled: &str,
    controlled_by: &str,
    target_value: i64,
    mut numbers: Numbers,
) -> i64 {
    let modify_value = |numbers: &mut Numbers, value| {
        let controlled_by = numbers.get_mut(controlled_by).unwrap();
        let current = controlled_by.get_value().unwrap();
        controlled_by.set_value(current + value);
    };

    let check_result = |numbers: &Numbers| {
        let mut expanded = numbers.clone();
        let mut controlled = expanded.remove(controlled).unwrap();
        controlled.resolve_dep_list(&mut expanded);
        controlled.get_value().unwrap()
    };

    let mut step = 2i64.pow(40);
    let mut last = check_result(&numbers).abs_diff(target_value);
    while last != 0 {
        modify_value(&mut numbers, step);

        let current = check_result(&numbers).abs_diff(target_value);
        if current > last {
            // go the other way
            step *= -1;
            step /= 2;
        }

        last = current;
    }

    numbers.get(controlled_by).unwrap().get_value().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_numbers_map() {
        build_numbers_map(get_input()).unwrap();
    }

    #[test]
    fn test_dep_list_resolve() {
        let mut numbers = build_numbers_map(get_input()).unwrap();

        let mut root = numbers.remove("root").unwrap();
        root.resolve_dep_list(&mut numbers);
        assert_eq!(root.value.unwrap(), 152)
    }

    #[test]
    fn test_find_controlled_value() {
        let mut numbers = build_numbers_map(get_input()).unwrap();
        numbers.get_mut("root").unwrap().set_op("-").unwrap();

        let value = find_controlled_value("root".into(), "humn".into(), 0, numbers);
        assert_eq!(value, 301)
    }

    fn get_input() -> &'static str {
        r#"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"#
    }
}
