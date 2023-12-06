#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Record {
    pub time: u64,
    pub distance: u64,
}

#[derive(thiserror::Error, Debug)]
pub enum ParseRecordErr {
    #[error("can not recognized the record format")]
    BadFormat,

    #[error("the record contains a mismatching number of races ({0} times and {1} distances)")]
    RecordMismatch(usize, usize),
}

pub fn parse_record_sheet(raw: &str) -> Result<Vec<Record>, ParseRecordErr> {
    let raw = raw.trim().lines().collect::<Vec<_>>();
    let (_, times) = raw
        .first()
        .ok_or(ParseRecordErr::BadFormat)?
        .trim()
        .split_once("Time:")
        .ok_or(ParseRecordErr::BadFormat)?;
    let (_, distances) = raw
        .get(1)
        .ok_or(ParseRecordErr::BadFormat)?
        .trim()
        .split_once("Distance:")
        .ok_or(ParseRecordErr::BadFormat)?;

    fn parse_line(line: &str) -> Result<Vec<u64>, ParseRecordErr> {
        line.trim()
            .split_ascii_whitespace()
            .map(|num| num.parse::<u64>().map_err(|_| ParseRecordErr::BadFormat))
            .collect()
    }

    let times = parse_line(times)?;
    let distances = parse_line(distances)?;
    if distances.len() != times.len() {
        return Err(ParseRecordErr::RecordMismatch(times.len(), distances.len()));
    }

    Ok(times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| Record { time, distance })
        .collect())
}

pub fn parse_record_sheet_correctly(raw: &str) -> Result<Record, ParseRecordErr> {
    let records = parse_record_sheet(raw)?;

    let (time, distance) = records.iter().fold(
        (String::new(), String::new()),
        |(time, distance), record| {
            (
                time + &record.time.to_string(),
                distance + &record.distance.to_string(),
            )
        },
    );

    Ok(Record {
        time: time.parse().unwrap(),
        distance: distance.parse().unwrap(),
    })
}

#[cfg(test)]
mod tests {
    use super::Record;

    #[test]
    fn parse_record_sheet() {
        let input = r#"Time:      7  15   30
Distance:  9  40  200"#;
        let expected_output = vec![
            Record {
                time: 7,
                distance: 9,
            },
            Record {
                time: 15,
                distance: 40,
            },
            Record {
                time: 30,
                distance: 200,
            },
        ];

        let output = super::parse_record_sheet(input).unwrap();
        assert_eq!(output, expected_output);
    }
}
