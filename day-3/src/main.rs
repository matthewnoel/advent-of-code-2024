use day_3::AdventError;

fn main() -> Result<(), AdventError> {
    let answer1 = day_3::part1()?;
    assert_eq!(answer1, 173517243);
    print!("Part 1: {}\n", answer1);
    let answer2 = day_3::part2()?;
    print!("Part 2: {}\n", answer2);
    assert_eq!(answer2, 100450138);
    Ok(())
}
