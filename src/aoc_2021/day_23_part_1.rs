fn parse_input(input: &str) -> Vec::<(char, char)> {
    let lines = input.split('\n').collect::<Vec<_>>();

    let mut rooms: Vec::<(char, char)> = vec![('.', '.'); 4];
    for (idx, ch) in lines[2].chars().filter(|c| c.is_alphabetic()).enumerate() {
        rooms[idx].0 = ch;
    }
    for (idx, ch) in lines[3].chars().filter(|c| c.is_alphabetic()).enumerate() {
        rooms[idx].1 = ch;
    }

    return rooms;
}

fn get_room_index_for_pod_type(pod: char) -> usize {
    match pod {
        'A' => return 0,
        'B' => return 1,
        'C' => return 2,
        'D' => return 3,
        _ => ()
    }
    return 0;
}

fn get_door_index_for_room_index(idx: usize) -> usize {
    match idx {
        0 => return 2,
        1 => return 4,
        2 => return 6,
        3 => return 8,
        _ => ()
    }
    return 2;
}

fn is_valid_hallway_index(idx: usize) -> bool {
    return idx != 2 && idx != 4 && idx != 6 && idx != 8;
}

fn is_solved(rooms: &Vec::<(char, char)>) -> bool {
    return rooms[0].0 == 'A' && rooms[0].1 == 'A' &&
        rooms[1].0 == 'B' && rooms[1].1 == 'B' &&
        rooms[2].0 == 'C' && rooms[2].1 == 'C' &&
        rooms[3].0 == 'D' && rooms[3].1 == 'D';
}

fn get_cost_per_move(pod: char) -> i64 {
    match pod {
        'A' => return 1,
        'B' => return 10,
        'C' => return 100,
        'D' => return 1000,
        _ => ()
    }
    return 0;
}

fn get_cost_of_move(pod: char, room_idx: usize, room_pos: usize, hallway_idx: usize) -> i64 {
    let door_idx = get_door_index_for_room_index(room_idx);
    let n_hallway_moves = if door_idx > hallway_idx { door_idx - hallway_idx } else { hallway_idx - door_idx };
    let n_room_moves = room_pos + 1;
    let cost_per_move = get_cost_per_move(pod);
    return ((n_room_moves + n_hallway_moves) as i64) * cost_per_move;
}

fn iterate_sim(rooms: &Vec::<(char, char)>, hallway: &Vec::<char>, current_cost: i64, min_cost: &mut i64) {
    // println!("rooms = {:?}", rooms);
    // println!("hallway = {:?}\n", hallway);

    if current_cost > *min_cost {
        return;
    }
    if is_solved(rooms) {
        *min_cost = i64::min(*min_cost, current_cost);
        return;
    }

    for (idx, &pod) in hallway.iter().enumerate() {
        if pod != '.' {
            let room_idx = get_room_index_for_pod_type(pod);
            let door_idx = get_door_index_for_room_index(room_idx);
            if door_idx < idx {
                if hallway[door_idx..idx].iter().find(|&&ch| ch != '.') != None {
                    continue;
                }
            }
            else {
                if hallway[idx + 1..door_idx].iter().find(|&&ch| ch != '.') != None {
                    continue;
                }
            }


            if rooms[room_idx].1 == '.' && rooms[room_idx].0 == '.' {
                let mut new_rooms = rooms.clone();
                let mut new_hallway = hallway.clone();

                new_hallway[idx] = '.';
                new_rooms[room_idx].1 = pod;

                // println!("Moving \'{}\' from hallway {} to room {} pos {}", pod, idx, room_idx, 1);

                let new_cost = current_cost + get_cost_of_move(pod, room_idx, 1, idx);
                iterate_sim(&new_rooms, &new_hallway, new_cost, min_cost);
            }
            else if rooms[room_idx].0 == '.' && rooms[room_idx].1 == pod {
                let mut new_rooms = rooms.clone();
                let mut new_hallway = hallway.clone();

                new_hallway[idx] = '.';
                new_rooms[room_idx].0 = pod;

                // println!("Moving \'{}\' from hallway {} to room {} pos {}", pod, idx, room_idx, 0);

                let new_cost = current_cost + get_cost_of_move(pod, room_idx, 0, idx);
                iterate_sim(&new_rooms, &new_hallway, new_cost, min_cost);
            }
        }
    }
    for (idx, (pod0, pod1)) in rooms.iter().enumerate() {
        if *pod0 != '.' && (idx != get_room_index_for_pod_type(*pod0) || (idx == get_room_index_for_pod_type(*pod0) && *pod1 != '.' && *pod0 != *pod1)) {
            let door_idx = get_door_index_for_room_index(idx);
            for hallway_idx in (0..door_idx).rev() {
                if is_valid_hallway_index(hallway_idx) {
                    if hallway[hallway_idx] != '.' {
                        break;
                    }

                    let mut new_rooms = rooms.clone();
                    let mut new_hallway = hallway.clone();

                    new_hallway[hallway_idx] = *pod0;
                    new_rooms[idx].0 = '.';

                    // println!("Moving \'{}\' from room {} pos {} to hallway {}", *pod0, idx, 0, hallway_idx);

                    let new_cost = current_cost + get_cost_of_move(*pod0, idx, 0, hallway_idx);
                    iterate_sim(&new_rooms, &new_hallway, new_cost, min_cost);
                }
            }
            for hallway_idx in door_idx + 1..hallway.len() {
                if is_valid_hallway_index(hallway_idx) {
                    if hallway[hallway_idx] != '.' {
                        break;
                    }

                    let mut new_rooms = rooms.clone();
                    let mut new_hallway = hallway.clone();

                    new_hallway[hallway_idx] = *pod0;
                    new_rooms[idx].0 = '.';

                    // println!("Moving \'{}\' from room {} pos {} to hallway {}", *pod0, idx, 0, hallway_idx);

                    let new_cost = current_cost + get_cost_of_move(*pod0, idx, 0, hallway_idx);
                    iterate_sim(&new_rooms, &new_hallway, new_cost, min_cost);
                }
            }
        }
        if *pod0 == '.' && *pod1 != '.' && idx != get_room_index_for_pod_type(*pod1) {
            let door_idx = get_door_index_for_room_index(idx);
            for hallway_idx in (0..door_idx).rev() {
                if is_valid_hallway_index(hallway_idx) {
                    if hallway[hallway_idx] != '.' {
                        break;
                    }

                    let mut new_rooms = rooms.clone();
                    let mut new_hallway = hallway.clone();

                    new_hallway[hallway_idx] = *pod1;
                    new_rooms[idx].1 = '.';

                    // println!("Moving \'{}\' from room {} pos {} to hallway {}", *pod1, idx, 1, hallway_idx);

                    let new_cost = current_cost + get_cost_of_move(*pod1, idx, 1, hallway_idx);
                    iterate_sim(&new_rooms, &new_hallway, new_cost, min_cost);
                }
            }
            for hallway_idx in door_idx + 1..hallway.len() {
                if is_valid_hallway_index(hallway_idx) {
                    if hallway[hallway_idx] != '.' {
                        break;
                    }

                    let mut new_rooms = rooms.clone();
                    let mut new_hallway = hallway.clone();

                    new_hallway[hallway_idx] = *pod1;
                    new_rooms[idx].1 = '.';

                    // println!("Moving \'{}\' from room {} pos {} to hallway {}", *pod1, idx, 1, hallway_idx);

                    let new_cost = current_cost + get_cost_of_move(*pod1, idx, 1, hallway_idx);
                    iterate_sim(&new_rooms, &new_hallway, new_cost, min_cost);
                }
            }
        }
    }
}

#[allow(dead_code)]
fn day_23_part_1(input: &str) -> i64 {
    let rooms = parse_input(input);
    let hallway: Vec::<char> = vec!['.'; 11];
    let current_cost: i64 = 0;
    let mut min_cost: i64 = i64::MAX;

    iterate_sim(&rooms, &hallway, current_cost, &mut min_cost);

    return min_cost;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_day_23_part_1() {
        let input =
r#"#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########"#;

        let ans = day_23_part_1(&input);

        assert_eq!(ans, 12521);
    }

    #[test]
    fn test_day_23_part_1() {
        let input =
r#"#############
#...........#
###D#D#B#A###
  #B#C#A#C#
  #########"#;

        let ans = day_23_part_1(&input);

        assert_eq!(ans, 16244)
    }
}