use day_1::AdventError;

fn main() -> Result<(), AdventError> {
    let answer1 = day_1::part1()?;
    assert_eq!(answer1, 1830467);
    print!("Part 1: {}\n", answer1);
    let answer2 = day_1::part2()?;
    print!("Part 2: {}\n", answer2);
    assert_eq!(answer2, 26674158);
    Ok(())
}
