use crate::aoc_2021::day_11_part_1::parse_input;

#[allow(dead_code)]
fn day_11_part_2(input: &str) -> usize {
    let mut grid = parse_input(&input);

    let mut step_count: usize = 0;

    loop {
        step_count += 1;

        let mut flashing = Vec::<(usize, usize)>::new();

        for (row_idx, row) in grid.iter_mut().enumerate() {
            for (col_idx, cell) in row.iter_mut().enumerate() {
                *cell += 1;
                if *cell == 10 {
                    flashing.push((row_idx, col_idx));
                }
            }
        }

        let mut flashed = Vec::<(usize, usize)>::new();
        while !flashing.is_empty() {
            let just_flashed = flashing.clone();
            flashing.clear();
            flashed.extend(just_flashed.clone());

            for (row_idx, col_idx) in just_flashed {
                let row_search_start = i64::max(row_idx as i64 - 1, 0) as usize;
                let row_search_end = usize::min(row_idx + 2, grid.len());

                let col_search_start = i64::max(col_idx as i64 - 1, 0) as usize;
                let col_search_end = usize::min(col_idx + 2, grid[0].len());

                for other_row_idx in row_search_start..row_search_end {
                    for other_col_idx in col_search_start..col_search_end {
                        if grid[other_row_idx][other_col_idx] != 10 {
                            grid[other_row_idx][other_col_idx] += 1;
                            if grid[other_row_idx][other_col_idx] == 10 {
                                flashing.push((other_row_idx, other_col_idx));
                            }
                        }
                    }
                }
            }
        }

        let num_flashes = flashed.len();

        if num_flashes == grid.len() * grid[0].len() {
            break;
        }

        for (row_idx, col_idx) in flashed {
            grid[row_idx][col_idx] = 0;
        }
    }

    return step_count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_day_11_part_2() {
        let input =
r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;

        let ans = day_11_part_2(&input);

        assert_eq!(ans, 195);
    }

    #[test]
    fn test_day_11_part_2() {
        let input =
r#"1254117228
4416873224
8354381553
1372637614
5586538553
7213333427
3571362825
1681126243
8718312138
5254266347"#;

        let ans = day_11_part_2(&input);

        assert_eq!(ans, 494);
    }
}