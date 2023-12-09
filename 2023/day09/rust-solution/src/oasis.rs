use std::str::FromStr;

#[derive(thiserror::Error, Debug)]
pub enum OasisErr {
    #[error("can not recognize the history format")]
    UnknownHistoryFormat,

    #[error("history can not be empty")]
    EmptyHistory,

    #[error("can not reach the zero sequence")]
    EmptyDifference,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct History {
    values: Vec<i32>,
}

impl History {
    /// Predict the next value in the history
    pub fn predict_next(&self) -> Result<i32, OasisErr> {
        let differences = self.calculate_differences()?;

        let mut value = 0;
        for dif in differences.iter().rev() {
            value += dif.last().unwrap();
        }

        Ok(value)
    }

    /// Predict the previous value in the history
    pub fn predict_prev(&self) -> Result<i32, OasisErr> {
        let differences = self.calculate_differences()?;

        let mut value = 0;
        for dif in differences.iter().rev() {
            value = dif.first().unwrap() - value;
        }

        Ok(value)
    }

    fn calculate_differences(&self) -> Result<Vec<Vec<i32>>, OasisErr> {
        let mut differences = Vec::new();
        differences.push(self.values.clone());

        loop {
            let dif = differences
                .last()
                .unwrap()
                .windows(2)
                .map(|values| values[1] - values[0])
                .collect::<Vec<_>>();

            if dif.is_empty() {
                return Err(OasisErr::EmptyDifference);
            }

            // stop when we reach the last sequence
            if dif.iter().all(|value| *value == 0) {
                break;
            }
            differences.push(dif);
        }

        Ok(differences)
    }
}

impl FromStr for History {
    type Err = OasisErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .trim()
            .split_ascii_whitespace()
            .map(|value| value.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| OasisErr::UnknownHistoryFormat)?;

        if values.is_empty() {
            return Err(OasisErr::EmptyHistory);
        }

        Ok(History { values })
    }
}

#[cfg(test)]
mod tests {
    use super::History;

    #[test]
    fn parse_history() {
        let input = "0 3 6 9 12 15";
        let expected_output = History {
            values: vec![0, 3, 6, 9, 12, 15],
        };

        let output: History = input.parse().unwrap();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn predict_next_values() {
        let inputs = ["0 3 6 9 12 15", "1 3 6 10 15 21", "10 13 16 21 30 45"];
        let expected_outputs = [18, 28, 68];

        for (input, expected) in inputs.into_iter().zip(expected_outputs) {
            let output: History = input.parse().unwrap();
            let output = output.predict_next().unwrap();
            assert_eq!(output, expected);
        }
    }

    #[test]
    fn predict_prev_values() {
        let input: History = "10 13 16 21 30 45".parse().unwrap();
        let expected_output = 5;

        let output = input.predict_prev().unwrap();
        assert_eq!(output, expected_output);
    }
}
