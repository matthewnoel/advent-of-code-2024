use std::fs;
use regex::Regex;

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

impl From<regex::Error> for AdventError {
    fn from(err: regex::Error) -> Self {
        AdventError::Regex(err)
    }
}

impl From<std::num::ParseIntError> for AdventError {
    fn from(err: std::num::ParseIntError) -> Self {
        AdventError::ParseInt(err)
    }
}

pub fn part1() -> Result<u32, AdventError> {
    let contents = fs::read_to_string("./src/input.txt")?;
    let regex = Regex::new(r"(\d+)\s+(\d+)")?;
    let mut list1: Vec<u32> = Vec::new();
    let mut list2: Vec<u32> = Vec::new();
    for line in contents.lines() {
        if let Some(captures) = regex.captures(line) {
            let capture1 = captures[1].parse::<u32>()?;
            let capture2 = captures[2].parse::<u32>()?;
            list1.push(capture1);
            list2.push(capture2);
        }
    }
    list1.sort();
    list2.sort();
    if list1.len() != list2.len() {
        return Err(AdventError::Io(std::io::Error::new(std::io::ErrorKind::Other, "Lists are not the same length")));
    }
    let mut total = 0;

    for i in 0..list1.len() {
        if list1[i] > list2[i] {
            total += list1[i] - list2[i];
        } else {
            total += list2[i] - list1[i];
        }
    }

    Ok(total)
}

pub fn part2() -> Result<u32, AdventError> {
    let contents = fs::read_to_string("./src/input.txt")?;
    let regex = Regex::new(r"(\d+)\s+(\d+)")?;
    let mut list1: Vec<u32> = Vec::new();
    let mut list2: Vec<u32> = Vec::new();
    for line in contents.lines() {
        if let Some(captures) = regex.captures(line) {
            let capture1 = captures[1].parse::<u32>()?;
            let capture2 = captures[2].parse::<u32>()?;
            list1.push(capture1);
            list2.push(capture2);
        }
    }

    let mut similarity = 0;
    for value in list1.iter() {
        for value2 in list2.iter() {
            if value == value2 {
                similarity += value;
            }
        }
    }
    Ok(similarity)
}
