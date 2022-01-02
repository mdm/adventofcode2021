use std::{collections::HashMap, io::BufRead};

use priority_queue::PriorityQueue;

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<char>>, state: &Vec<(usize, usize)>, per_room: usize) {
    for (y, row) in map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let amphipod_id = state.iter().position(|position| *position == (x, y));
            if let Some(amphipod_id) = amphipod_id {
                let amphipod = match amphipod_id / per_room {
                    0 => 'A',
                    1 => 'B',
                    2 => 'C',
                    3 => 'D',
                    _ => unreachable!(),
                };
                print!("{}", amphipod);
            } else {
                print!("{}", tile);
            }
        }
        println!();
    }
}

fn solved(state: &Vec<(usize, usize)>, per_room: usize) -> bool {
    for (amphipod_id, position) in state.iter().enumerate() {
        let target_x = amphipod_id / per_room * 2 + 3;
        if !(position.0 == target_x && position.1 >= 2) {
            return false;
        }
    }

    true
}

fn estimate_energy(state: &Vec<(usize, usize)>, per_room: usize) -> i32 {
    state
        .iter()
        .enumerate()
        .map(|(amphipod_id, position)| {
            let energy_per_step = 10i32.pow(amphipod_id as u32 / per_room as u32);
            let target_x = amphipod_id / per_room * 2 + 3;
            let steps = if position.0 == target_x && position.1 >= 2 {
                0
            } else {
                distance(*position, (target_x, 2))
            };

            steps * energy_per_step
        })
        .sum()
}

fn distance(position_a: (usize, usize), position_b: (usize, usize)) -> i32 {
    if position_a.0 == position_b.0 && position_a.1 >= 2 {
        (position_a.0 as i32 - position_b.0 as i32).abs()
            + (position_a.1 as i32 - position_b.1 as i32).abs()
    } else {
        let shift = position_a.1 as i32 - 1;

        shift
            + (position_a.0 as i32 - position_b.0 as i32).abs()
            + (position_a.1 as i32 - shift - position_b.1 as i32).abs()
    }
}

fn has_amphipods_above(state: &Vec<(usize, usize)>, reference: &(usize, usize)) -> bool {
    state
        .iter()
        .enumerate()
        .filter(|(_amphipod_id, position)| position.0 == reference.0 && position.1 < reference.1)
        .peekable()
        .peek()
        .is_some()
}

fn has_amphipods_below<P>(
    state: &Vec<(usize, usize)>,
    reference: &(usize, usize),
    predicate: P,
) -> bool
where
    P: Fn(usize) -> bool,
{
    state
        .iter()
        .enumerate()
        .filter(|(amphipod_id, position)| {
            position.0 == reference.0 && position.1 > reference.1 && predicate(*amphipod_id)
        })
        .peekable()
        .peek()
        .is_some()
}

fn blockers_for_room(state: &Vec<(usize, usize)>, room: usize, per_room: usize) -> Vec<usize> {
    let target_x = room * 2 + 3;

    state
        .iter()
        .enumerate()
        .filter(|(amphipod_id, position)| {
            position.0 == target_x
                && (amphipod_id / per_room != room
                    || has_amphipods_below(state, position, |below_id| below_id / per_room != room))
        })
        .map(|(amphipod_id, _position)| amphipod_id)
        .collect()
}

fn all_blockers(state: &Vec<(usize, usize)>, per_room: usize) -> Vec<usize> {
    let mut blockers = Vec::new();
    for room in 0..4 {
        blockers.extend(blockers_for_room(state, room, per_room));
    }

    blockers
}

fn can_move_into_room(state: &Vec<(usize, usize)>, amphipod_id: usize, per_room: usize) -> bool {
    let position = state[amphipod_id];
    let room = amphipod_id / per_room;
    if has_amphipods_above(state, &state[amphipod_id])
        || !blockers_for_room(state, room, per_room).is_empty()
    {
        return false;
    }

    let target_x = room * 2 + 3;
    let potential_blockers = vec![(4, 1), (6, 1), (8, 1)];
    potential_blockers
        .iter()
        .filter(|blocker| {
            state.contains(blocker)
                && ((position.0 < blocker.0 && target_x > blocker.0)
                    || (position.0 > blocker.0 && target_x < blocker.0))
        })
        .peekable()
        .peek()
        .is_none()
}

fn move_into_room(
    state: &Vec<(usize, usize)>,
    amphipod_id: usize,
    per_room: usize,
) -> (Vec<(usize, usize)>, i32) {
    let room = amphipod_id / per_room;
    let target_x = room * 2 + 3;

    let mut next_state = state.clone();
    let position = next_state[amphipod_id];
    let home_position = (0..per_room)
        .map(|i| (target_x, i + 2))
        .filter(|candidate| !next_state.contains(candidate))
        .max()
        .expect("Epected to have enough space in room");

    next_state[amphipod_id] = home_position;
    let energy_used = 10i32.pow(room as u32) * distance(position, home_position);

    (next_state, energy_used)
}

fn reachable_hallway_spots(state: &Vec<(usize, usize)>, amphipod_id: usize) -> Vec<(usize, usize)> {
    if has_amphipods_above(state, &state[amphipod_id]) {
        return vec![];
    }

    let hallway_spots = vec![(1, 1), (2, 1), (4, 1), (6, 1), (8, 1), (10, 1), (11, 1)];
    hallway_spots
        .iter()
        .filter(|target_spot| {
            if state.contains(target_spot) {
                return false;
            }

            let position = state[amphipod_id];
            hallway_spots
                .iter()
                .filter(|blocker| {
                    state.contains(blocker)
                        && ((position.0 < blocker.0 && target_spot.0 > blocker.0)
                            || (position.0 > blocker.0 && target_spot.0 < blocker.0))
                })
                .peekable()
                .peek()
                .is_none()
        })
        .map(|target_spot| *target_spot)
        .collect()
}

#[allow(unused_variables)]
fn find_lowest_energy_fast(
    map: &Vec<Vec<char>>,
    state: &Vec<(usize, usize)>,
    per_room: usize,
) -> Option<i32> {
    let mut queue = PriorityQueue::new();
    queue.push(state.clone(), -0);

    let mut energy_used = HashMap::new();
    energy_used.insert(state.clone(), -0);

    let mut iterations = 0;
    while let Some((current, energy_so_far)) = queue.pop() {
        if solved(&current, per_room) {
            // dbg!(iterations);
            return Some(-energy_so_far);
        }

        let blockers = all_blockers(&current, per_room);

        for (amphipod_id, position) in current.iter().enumerate() {
            let target_x = amphipod_id / per_room * 2 + 3;
            if position.0 == target_x
                && !has_amphipods_below(&current, position, |below_id| {
                    below_id / per_room != amphipod_id / per_room
                })
            {
                continue; // done moving
            }

            if can_move_into_room(&current, amphipod_id, per_room) {
                let (next_state, energy_for_move) = move_into_room(&current, amphipod_id, per_room);
                let next_energy = energy_used[&current] - energy_for_move;

                if !energy_used.contains_key(&next_state) || energy_used[&next_state] <= next_energy
                {
                    energy_used.insert(next_state.clone(), next_energy);
                    let next_energy_estimated =
                        next_energy - estimate_energy(&next_state, per_room);
                    queue.push_increase(next_state, next_energy);
                }

                continue;
            }

            if !blockers.contains(&amphipod_id) {
                continue;
            }

            let hallway_spots = reachable_hallway_spots(&current, amphipod_id);
            for next_position in hallway_spots {
                let next_energy = energy_used[&current]
                    - 10i32.pow(amphipod_id as u32 / per_room as u32)
                        * distance(*position, next_position);
                let mut next_state = current.clone();
                next_state[amphipod_id] = next_position;

                if !energy_used.contains_key(&next_state) || energy_used[&next_state] <= next_energy
                {
                    energy_used.insert(next_state.clone(), next_energy);
                    let next_energy_estimated =
                        next_energy - estimate_energy(&next_state, per_room);
                    queue.push_increase(next_state, next_energy);
                }
            }
        }

        iterations += 1;
    }

    None
}

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);

    let original_map = reader.lines().map(|line| line.unwrap()).collect::<Vec<_>>();

    let mut positions = Vec::new();
    let map1 = original_map
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.char_indices()
                .map(|(x, tile)| {
                    if "ABCD".contains(tile) {
                        positions.push((tile, (x, y)));
                        '.'
                    } else {
                        tile
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    positions.sort();
    let state = positions
        .into_iter()
        .map(|(_, coordinates)| coordinates)
        .collect::<Vec<_>>();

    let part1 = find_lowest_energy_fast(&map1, &state, 2).unwrap();
    println!("{}", part1);

    let mut modified_map = original_map.iter().take(3).cloned().collect::<Vec<_>>();
    modified_map.push("  #D#C#B#A#".to_string());
    modified_map.push("  #D#B#A#C#".to_string());
    modified_map.extend(original_map.iter().skip(3).cloned().collect::<Vec<_>>());

    let mut positions = Vec::new();
    let map2 = modified_map
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.char_indices()
                .map(|(x, tile)| {
                    if "ABCD".contains(tile) {
                        positions.push((tile, (x, y)));
                        '.'
                    } else {
                        tile
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    positions.sort();
    let state = positions
        .into_iter()
        .map(|(_, coordinates)| coordinates)
        .collect::<Vec<_>>();

    let part2 = find_lowest_energy_fast(&map2, &state, 4).unwrap();
    println!("{}", part2);
}
