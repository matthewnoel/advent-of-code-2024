use core::num;
use regex::Regex;
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
    let matcher = vec!['m', 'u', 'l', '(', '0', '0', '0', ',', '1', '1', '1', ')'];
    let mut mutable_matcher = matcher.clone();
    let mut num1 = 0;
    let mut num2 = 0;
    let mut total: u32 = 0;

    for line in contents.lines() {
        for c in line.chars() {
            if mutable_matcher[0] == '0' {
                if c.is_digit(10) {
                    // is the number too big?
                    if num1 > 99 {
                        mutable_matcher = matcher.clone();
                        num1 = 0;
                        num2 = 0;
                    } else {
                        num1 = (10 * num1) + c.to_digit(10).unwrap();
                    }
                } else {
                    if num1 > 0 && c == ',' {
                        // do we already have one digit and we're ending early?
                        while mutable_matcher[0] == '0' {
                            mutable_matcher.remove(0);
                        }
                    } else {
                        mutable_matcher = matcher.clone();
                        num1 = 0;
                        num2 = 0;
                    }
                }
            } else if mutable_matcher[0] == '1' {
                if c.is_digit(10) {
                    // is the number too big?
                    if num2 > 99 {
                        mutable_matcher = matcher.clone();
                        num1 = 0;
                        num2 = 0;
                    } else {
                        num2 = (10 * num2) + c.to_digit(10).unwrap();
                    }
                } else {
                    if num2 > 0 && c == ')' {
                        // do we already have one digit and we're ending early?
                        while mutable_matcher[0] == '1' {
                            mutable_matcher.remove(0);
                        }
                        total += num1 * num2;
                        mutable_matcher = matcher.clone();
                        num1 = 0;
                        num2 = 0;
                    } else {
                        mutable_matcher = matcher.clone();
                        num1 = 0;
                        num2 = 0;
                    }
                }
            } else if c == mutable_matcher[0] {
                mutable_matcher.remove(0);
                if mutable_matcher.len() == 0 {
                    total += num1 * num2;
                    mutable_matcher = matcher.clone();
                    num1 = 0;
                    num2 = 0;
                }
            } else if mutable_matcher.len() != matcher.len() {
                // we didn't get a match but had one ongoing
                mutable_matcher = matcher.clone();
                num1 = 0;
                num2 = 0;
            }
        }
    }

    Ok(total)
}

pub fn part2() -> Result<u32, AdventError> {
    Ok(0)
}
