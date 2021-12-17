#[allow(unused_imports)]
use crate::aoc_2021::day_17_part_1::day_17_part_1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_day_17_part_2() {
        let input =
r#"target area: x=20..30, y=-10..-5"#;

        let (_, total_valid_velocities) = day_17_part_1(&input);

        assert_eq!(total_valid_velocities, 112);
    }

    #[test]
    fn test_day_17_part_2() {
        let input =
r#"target area: x=34..67, y=-215..-186"#;

        let (_, total_valid_velocities) = day_17_part_1(&input);

        assert_eq!(total_valid_velocities, 2040);
    }
}
