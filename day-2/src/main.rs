use day_2::AdventError;

fn main() -> Result<(), AdventError> {
    let answer1 = day_2::part1()?;
    assert_eq!(answer1, 639);
    print!("Part 1: {}\n", answer1);
    let answer2 = day_2::part2()?;
    print!("Part 2: {}\n", answer2);
    // assert_eq!(answer2, 26674158);
    Ok(())
}
