use crate::aoc_2021::day_21_part_1::parse_input;

fn play_game(wins: &mut Vec::<u64>, player_idx: usize, positions: &Vec<u64>, scores: &Vec<u64>, path_frequency: u64) {
    // 1 + 1 + 1 = 3
    // 1 + 1 + 2 = 4
    // 1 + 1 + 3 = 5
    // 1 + 2 + 1 = 4
    // 1 + 2 + 2 = 5
    // 1 + 2 + 3 = 6
    // 1 + 3 + 1 = 5
    // 1 + 3 + 2 = 6
    // 1 + 3 + 3 = 7
    
    // 2 + 1 + 1 = 4
    // 2 + 1 + 2 = 5
    // 2 + 1 + 3 = 6
    // 2 + 2 + 1 = 5
    // 2 + 2 + 2 = 6
    // 2 + 2 + 3 = 7
    // 2 + 3 + 1 = 6
    // 2 + 3 + 2 = 7
    // 2 + 3 + 3 = 8

    // 3 + 1 + 1 = 5
    // 3 + 1 + 2 = 6
    // 3 + 1 + 3 = 7
    // 3 + 2 + 1 = 6
    // 3 + 2 + 2 = 7
    // 3 + 2 + 3 = 8
    // 3 + 3 + 1 = 7
    // 3 + 3 + 2 = 8
    // 3 + 3 + 3 = 9

    let possible_die_totals: Vec<(u64, u64)> = vec![(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

    for (die_total, frequency) in possible_die_totals {
        let mut new_positions = positions.clone();
        new_positions[player_idx] = (new_positions[player_idx] + die_total) % 10;

        let mut new_scores = scores.clone();
        new_scores[player_idx] += if new_positions[player_idx] != 0 { new_positions[player_idx] } else { 10 };

        let new_path_frequency = path_frequency * frequency;

        if new_scores[player_idx] >= 21 {
            // +1 win for current player
            wins[player_idx] += new_path_frequency;
        }
        else {
            // Continue playing
            let new_player_idx = (player_idx + 1) % 2;
            play_game(wins, new_player_idx, &new_positions, &new_scores, new_path_frequency);
        }
    }
}

fn day_21_part_2(input: &str) -> u64 {
    let (p1_pos, p2_pos) = parse_input(input);

    let mut wins: Vec::<u64> = vec![0, 0];
    let player_idx: usize = 0;
    let positions: Vec::<u64> = vec![p1_pos, p2_pos];
    let scores: Vec<u64> = vec![0, 0];
    let path_frequency: u64 = 1;
    play_game(&mut wins, player_idx, &positions, &scores, path_frequency);

    return u64::max(wins[0], wins[1]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_day_21_part_2() {
        let input =
r#"Player 1 starting position: 4
Player 2 starting position: 8"#;

        let ans = day_21_part_2(&input);

        assert_eq!(ans, 444356092776315);
    }

    #[test]
    fn test_day_21_part_2() {
        let input =
r#"Player 1 starting position: 8
Player 2 starting position: 2"#;

        let ans = day_21_part_2(&input);

        assert_eq!(ans, 105619718613031)
    }
}