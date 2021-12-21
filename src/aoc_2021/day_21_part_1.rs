pub fn parse_input(input: &str) -> (u64, u64) {
    let lines = input.split('\n').collect::<Vec<_>>();

    let player_1_pos = lines[0].split(": ").collect::<Vec<_>>()[1].parse::<u64>().unwrap();
    let player_2_pos = lines[1].split(": ").collect::<Vec<_>>()[1].parse::<u64>().unwrap();

    return (player_1_pos, player_2_pos);
}

fn day_21_part_1(input: &str) -> u64 {
    let (p1_pos, p2_pos) = parse_input(input);
    let n_players = 2;
    let mut players: Vec::<(u64, u64)> = vec![(0u64, 0u64); n_players];
    let mut die_val: u64 = 1;

    players[0].0 = p1_pos;
    players[1].0 = p2_pos;

    let mut n_rolls: u64 = 0;
    let mut game_over = false;
    while !game_over {
        for i in 0..n_players {
            let mut die_total: u64 = 0;
            for _ in 0..3 {
                die_total += die_val;
                die_val += 1;
                if die_val > 100 {
                    die_val = 1;
                }
                n_rolls += 1;
            }
            let pos = (players[i].0 + die_total) % 10;
            players[i].0 = pos;
            players[i].1 += if pos != 0 { pos } else { 10 };
            if players[i].1 >= 1000 {
                game_over = true;
                break;
            }
        }
    }

    let losing_score = u64::min(players[0].1, players[1].1);

    return losing_score * n_rolls;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_day_21_part_1() {
        let input =
r#"Player 1 starting position: 4
Player 2 starting position: 8"#;

        let ans = day_21_part_1(&input);

        assert_eq!(ans, 739785);
    }

    #[test]
    fn test_day_21_part_1() {
        let input =
r#"Player 1 starting position: 8
Player 2 starting position: 2"#;

        let ans = day_21_part_1(&input);

        assert_eq!(ans, 513936)
    }
}
