fn parse_input(input: &str) -> ((i64, i64), (i64, i64)) {
    let sub_string = input.split(": ").collect::<Vec<_>>()[1];

    let range_strings = sub_string.split(", ").collect::<Vec<_>>();

    let x_range_string = range_strings[0].split("=").collect::<Vec<_>>()[1];
    let y_range_string = range_strings[1].split("=").collect::<Vec<_>>()[1];

    let x_range = x_range_string.split("..").collect::<Vec<_>>();
    let y_range = y_range_string.split("..").collect::<Vec<_>>();

    let x_min = x_range[0].parse::<i64>().unwrap();
    let x_max = x_range[1].parse::<i64>().unwrap();

    let y_min = y_range[0].parse::<i64>().unwrap();
    let y_max = y_range[1].parse::<i64>().unwrap();

    return ((x_min, x_max), (y_min, y_max));
}

fn calc_trajectory(origin: (i64, i64), target_region: ((i64, i64), (i64, i64)), initial_velocity: (i64, i64)) -> (bool, (i64, i64), i64, usize)
{
    let mut n_steps: usize = 0;
    let mut position = origin;
    let mut velocity = initial_velocity;
    let mut target_region_reached = false;
    let mut peak: i64 = i64::MIN;

    while position.0 <= target_region.0.1 && position.1 >= target_region.1.0 {
        peak = i64::max(peak, position.1);

        if position.0 >= target_region.0.0 && position.0 <= target_region.0.1 && position.1 >= target_region.1.0 && position.1 <= target_region.1.1 {
            target_region_reached = true;
            break;
        }

        position.0 += velocity.0;
        position.1 += velocity.1;

        if velocity.0 > 0 {
            velocity.0 -= 1;
        }
        else if velocity.0 < 0 {
            velocity.0 += 1;
        }

        velocity.1 -= 1;

        n_steps += 1;
    }

    return (target_region_reached, position, peak, n_steps)
}

#[allow(dead_code)]
pub fn day_17_part_1(input: &str) -> (i64, usize) {
    let ((x_min, x_max), (y_min, y_max)) = parse_input(input);
    println!("x={}..{}, y={}..{}", x_min, x_max, y_min, y_max);

    let origin = (0i64, 0i64);

    let max_y_velocity = i64::abs(y_min);
    let min_y_velocity = y_min;

    let max_x_velocity = x_max;
    let min_x_velocity = if x_min < 0 { -1 } else { 1 };

    let mut acheived_peak: i64 = i64::MIN;
    let mut total_valid_velocities: usize = 0;

    for vel_x in min_x_velocity..=max_x_velocity {
        for vel_y in min_y_velocity..=max_y_velocity {
            let (target_region_reached, _, peak, _) = calc_trajectory(origin, ((x_min, x_max), (y_min, y_max)), (vel_x, vel_y));
            if target_region_reached {
                total_valid_velocities += 1;
                acheived_peak = i64::max(peak, acheived_peak);
            }
        }
    }

    return (acheived_peak, total_valid_velocities);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_day_17_part_1() {
        let input =
r#"target area: x=20..30, y=-10..-5"#;

        let (acheived_peak, _) = day_17_part_1(&input);

        assert_eq!(acheived_peak, 45);
    }

    #[test]
    fn test_day_17_part_1() {
        let input =
r#"target area: x=34..67, y=-215..-186"#;

        let (acheived_peak, _) = day_17_part_1(&input);

        assert_eq!(acheived_peak, 23005);
    }
}
