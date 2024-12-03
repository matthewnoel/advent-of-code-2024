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

fn get_numbers_from_obj(obj: &Vec<char>) -> (u32, u32) {
    let mut number_one = 0;
    let mut number_two = 0;
    for c in obj.iter() {
        if c.is_digit(10) {
            number_one = (10 * number_one) + c.to_digit(10).unwrap();
        } else if *c == ',' {
            number_two = number_one;
            number_one = 0;
        }
    }
    (number_one, number_two)
}

fn increment_total(obj: &mut Vec<char>, total: &mut u32) {
    let (number_one, number_two) = get_numbers_from_obj(obj);
    *total += number_one * number_two;
    reset_temp_object(obj);
}

fn reset_temp_object(obj: &mut Vec<char>) {
    obj.clear();
}

fn try_add_character(c: char, obj: &mut Vec<char>) -> Option<bool> {
    match obj.len() {
        0 => {
            if c == 'm' {
                obj.push(c);
            }
            return None;
        },
        1 => {
            if c == 'u' {
                obj.push(c);
                return None;
            } else {
                return Some(false);
            }
        },
        2 => {
            if c == 'l' {
                obj.push(c);
                return None;
            } else {
                return Some(false);
            }
        },
        3 => {
            if c == '(' {
                obj.push(c);
                return None;
            } else {
                return Some(false);
            }
        },
        4 => {
            if c.is_digit(10) {
                obj.push(c);
                return None;
            } else {
                return Some(false);
            }
        },
        _ => ()
    }
    
    if c.is_digit(10) {
        if *obj.last().unwrap() == ',' {
            obj.push(c);
            return None;
        }
        // todo is number too big?
        if !obj[obj.len() - 2].is_digit(10) && obj[obj.len() - 2].is_digit(10) && obj[obj.len() - 1].is_digit(10) {
            return Some(false);
        } else {
            obj.push(c);
            return None;
        }

    } else if c == ',' {
        let (number_one, number_two) = get_numbers_from_obj(obj);
        if number_two > 0 || number_one == 0 {
            return Some(false);
        }
        obj.push(c);
        return None;
    } else if c == ')' {
        let (number_one, number_two) = get_numbers_from_obj(obj);
        let result = number_one > 0 && number_two > 0;
        if result {
            obj.push(c);
        }
        return Some(result);
    } else {
        return Some(false);
    }
}

pub fn part1() -> Result<u32, AdventError> {
    let contents = fs::read_to_string("./src/input.txt")?;

    let mut obj: Vec<char> = Vec::new();
    let mut total: u32 = 0;

    for line in contents.lines() {
        for c in line.chars() {
            match try_add_character(c, &mut obj) {
                Some(true) => increment_total(&mut obj, &mut total),
                Some(false) => reset_temp_object(&mut obj),
                None => ()
            }
        }
    }

    Ok(total)
}

pub fn part2() -> Result<u32, AdventError> {
    Ok(0)
}
