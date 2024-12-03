use std::fs;

#[derive(Debug)]
pub enum AdventError {
    Io(std::io::Error),
    Regex(regex::Error),
    ParseInt(std::num::ParseIntError),
}

impl From<std::io::Error> for AdventError {
    fn from(err: std::io::Error) -> Self {
        AdventError::Io(err)
    }
}

impl From<std::num::ParseIntError> for AdventError {
    fn from(err: std::num::ParseIntError) -> Self {
        AdventError::ParseInt(err)
    }
}

pub fn part1() -> Result<u32, AdventError> {
    let contents = fs::read_to_string("./src/input.txt")?;
    let mut safe_reports = 0;
    for line in contents.lines() {
        let parsed = parse_report(line)?;
        if is_report_safe(parsed)? {
            safe_reports += 1;
        }
    }
    Ok(safe_reports)
}

fn parse_report(report: &str) -> Result<Vec<i32>, AdventError> {
    let mut report_vec: Vec<i32> = Vec::new();
    for num in report.split_whitespace() {
        report_vec.push(num.parse::<i32>()?);
    }
    Ok(report_vec)
}

fn is_report_safe(report: Vec<i32>) -> Result<bool, AdventError> {
    let mut optional_previous_value: Option<i32> = Option::None;
    let mut optional_direction = Option::<bool>::None;

    for value in &report {
        if let Some(previous_value) = optional_previous_value {
            if previous_value == *value {
                return Ok(false);
            }
            if (previous_value - value).abs() > 3 {
                return Ok(false);
            }
            if let Some(direction) = optional_direction {
                if direction != (previous_value < *value) {
                    return Ok(false);
                }
            } else {
                optional_direction = Some(previous_value < *value);
            }
        }
        optional_previous_value = Some(*value);
    }

    return Ok(true);
}

pub fn part2() -> Result<u32, AdventError> {
    let contents = fs::read_to_string("./src/input.txt")?;
    let mut safe_reports = 0;
    for line in contents.lines() {
        let parsed = parse_report(line)?;
        if is_any_version_of_report_safe(parsed)? {
            safe_reports += 1;
        }
    }
    Ok(safe_reports)
}

fn is_any_version_of_report_safe(report: Vec<i32>) -> Result<bool, AdventError> {
    // stupid solution, but my other strategies weren't working
    for i in 0..report.len() {
        let mut modified_report = report.clone();
        modified_report.remove(i);
        if is_report_safe(modified_report)? {
            return Ok(true);
        }
    }
    Ok(false)
}
