use std::{
    collections::{HashMap, VecDeque},
    io::BufRead,
};

use priority_queue::PriorityQueue;

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

fn next_positions(
    map: &Vec<Vec<char>>,
    state: &Vec<(usize, usize)>,
    amphipod_id: usize,
    per_room: usize,
) -> Vec<(usize, usize)> {
    let target_x = amphipod_id / per_room * 2 + 3;

    let target_clear = state
        .iter()
        .enumerate()
        .filter(|(_, position)| position.0 == target_x && position.1 >= 2)
        .all(|(occupant_id, _)| occupant_id / per_room * 2 + 3 == target_x);

    if state[amphipod_id].0 == target_x && target_clear {
        return vec![];
    }

    let mut positions = vec![state[amphipod_id]];
    let mut queue = VecDeque::new();
    queue.push_back(state[amphipod_id]);

    while let Some(current) = queue.pop_front() {
        for offset in vec![(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let next_position = (
                (current.0 as i32 + offset.0) as usize,
                (current.1 as i32 + offset.1) as usize,
            );

            if map[next_position.1][next_position.0] == '.'
                && !state.contains(&next_position)
                && !positions.contains(&next_position)
            {
                positions.push(next_position);
                queue.push_back(next_position);
            }
        }
    }

    let (on_target, intermediate): (Vec<(usize, usize)>, Vec<(usize, usize)>) = positions
        .iter()
        .partition(|position| position.0 == target_x);

    if target_clear && !on_target.is_empty() {
        let target_position = on_target
            .into_iter()
            .max_by(|position_a, position_b| position_a.1.cmp(&position_b.1))
            .expect("Expected on_target not ot be empty");
        return vec![target_position];
    }

    intermediate
        .into_iter()
        .filter(|position| {
            !vec![state[amphipod_id], (3, 1), (5, 1), (7, 1), (9, 1)].contains(position)
        })
        .collect()
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
        .filter(|(amphipod_id, position)| position.0 == reference.0 && position.1 < reference.1)
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
            // dbg!(amphipod_id, position.0, target_x);
            if position.0 == target_x {
                let foreign = amphipod_id / per_room != room;
                let below =
                    has_amphipods_below(state, position, |below_id| below_id / per_room != room);
                // dbg!(foreign, below);
                amphipod_id / per_room != room
                    || has_amphipods_below(state, position, |below_id| below_id / per_room != room)
            } else {
                let foreign = amphipod_id / per_room != room;
                let below =
                    has_amphipods_below(state, position, |below_id| below_id / per_room == room);
                // dbg!(foreign, below);
                amphipod_id / per_room != room
                    && has_amphipods_below(state, position, |below_id| below_id / per_room == room)
            }
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
    if !blockers_for_room(state, room, per_room).is_empty() {
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

fn find_lowest_energy_fast(
    map: &Vec<Vec<char>>,
    state: &Vec<(usize, usize)>,
    per_room: usize,
) -> Option<i32> {
    let mut queue = PriorityQueue::new();
    queue.push(state.clone(), -0);

    let mut energy_used = HashMap::new();
    energy_used.insert(state.clone(), -0);

    let mut iterations = 0usize;
    while let Some((current, energy_so_far)) = queue.pop() {
        if solved(&current, per_room) {
            dbg!(iterations);
            print_map(map, &current, per_room);
            dbg!(energy_used[&current]);
            return Some(-energy_so_far);
        }

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
                // dbg!(iterations, amphipod_id, position, target_x);
                // print_map(map, &current, per_room);

                let (next_state, energy_for_move) = move_into_room(&current, amphipod_id, per_room);
                let next_energy = energy_used[&current] - energy_for_move;

                if !energy_used.contains_key(&next_state) || energy_used[&next_state] <= next_energy
                {
                    energy_used.insert(next_state.clone(), next_energy);
                    let next_energy_estimated =
                        next_energy - estimate_energy(&next_state, per_room);
                    queue.push_increase(next_state, next_energy_estimated);
                }

                continue;
            }

            let blockers = all_blockers(&current, per_room);
            if !blockers.contains(&amphipod_id) {
                continue;
            }

            let candidates = reachable_hallway_spots(&current, amphipod_id);
            for next_position in candidates {
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
                    queue.push_increase(next_state, next_energy_estimated);
                }
            }
        }

        iterations += 1;

        if iterations % 50_000 == 0 {
            dbg!(iterations);
            print_map(map, &current, per_room);
            dbg!(energy_so_far);
        }
    }

    dbg!(iterations);

    None
}

fn find_lowest_energy_slow(
    map: &Vec<Vec<char>>,
    state: &Vec<(usize, usize)>,
    per_room: usize,
) -> i32 {
    let mut queue = PriorityQueue::new();
    queue.push(state.clone(), -0);

    let mut energy_used = HashMap::new();
    energy_used.insert(state.clone(), -0);

    let mut iterations = 0usize;
    while let Some((current, energy_so_far)) = queue.pop() {
        if solved(&current, per_room) {
            dbg!(iterations);
            print_map(map, &current, per_room);
            dbg!(energy_used[&current]);
            return -energy_so_far;
        }

        for (amphipod_id, position) in current.iter().enumerate() {
            for next_position in next_positions(map, &current, amphipod_id, per_room) {
                if !(position.1 >= 2) && !(next_position.1 >= 2) {
                    continue; // stay put of in hallway and next_position is also in hallway
                }

                if position.0 != next_position.0 && next_position.1 >= 2 {
                    let target_x = amphipod_id / per_room * 2 + 3;
                    if next_position.0 != target_x {
                        continue; // not target room
                    }

                    let free = current
                        .iter()
                        .enumerate()
                        .filter(|(_, position)| position.0 == next_position.0 && position.1 >= 2)
                        .all(|(occupant_id, _)| occupant_id / per_room == amphipod_id / per_room);

                    if !free {
                        continue; // room is occupied by other amphipod type
                    }
                }

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
                    queue.push_increase(next_state, next_energy_estimated);
                }
            }
        }

        iterations += 1;

        if iterations % 500_000 == 0 {
            dbg!(iterations);
            print_map(map, &current, per_room);
            dbg!(energy_so_far);
        }
    }

    dbg!(iterations);

    -1 // no solution
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

    print_map(&map1, &state, 2);

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
    let mut state = positions
        .into_iter()
        .map(|(_, coordinates)| coordinates)
        .collect::<Vec<_>>();

    print_map(&map2, &state, 4);

    let tmp = find_lowest_energy_fast(&map2, &state, 4);
    dbg!(tmp);

    // let part2 = find_lowest_energy_fast(&map2, &state, 4);
    // println!("{}", part2);
}
