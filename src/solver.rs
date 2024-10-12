use std::collections::{HashMap, VecDeque};

use crate::cube::Cube;

pub fn solve(cube: &Cube) -> Result<Vec<u8>, String> {
    let cube1 = cube.clone();
    let cube2 = Cube::new();
    let max_depth = 15;
    meet_in_middle(cube1, cube2, max_depth)
}

fn meet_in_middle(right_cube: Cube, left_cube: Cube, max_depth: usize) -> Result<Vec<u8>, String> {
    if right_cube == left_cube {
        return Ok(vec![]);
    }

    let mut visited_left: HashMap<Cube, Vec<u8>> = HashMap::new();
    visited_left.insert(left_cube.clone(), vec![]);

    let mut visited_right: HashMap<Cube, Vec<u8>> = HashMap::new();
    visited_right.insert(right_cube.clone(), vec![]);

    let mut queue_left: VecDeque<Cube> = VecDeque::new();
    queue_left.push_back(left_cube);

    let mut queue_right: VecDeque<Cube> = VecDeque::new();
    queue_right.push_back(right_cube);

    for _ in 0..max_depth {
        if let Some(cube) = queue_left.pop_front() {
            let mut left_action = visited_left.get(&cube).unwrap().clone();
            for action in 0..12 {
                let mut copy = cube.clone();
                copy.rotate(action);

                if visited_right.contains_key(&copy) {
                    let right_action = visited_right.get(&cube).unwrap().to_owned();
                    left_action.extend(right_action.iter().rev());
                    return Ok(left_action);
                }

                if !visited_left.contains_key(&copy) {
                    let mut total_actions = left_action.clone();
                    total_actions.push(action);
                    visited_left.insert(copy.clone(), total_actions);
                    queue_left.push_back(copy);
                }
            }
        }

        if let Some(cube) = queue_right.pop_front() {
            let right_action = visited_right.get(&cube).unwrap().clone();
            for action in 0..12 {
                let mut copy = cube.clone();
                copy.rotate(action);

                if visited_left.contains_key(&copy) {
                    let mut left_action = visited_left.get(&cube).unwrap().to_owned();
                    left_action.extend(right_action.iter().rev());
                    return Ok(left_action);
                }

                if !visited_right.contains_key(&copy) {
                    let mut total_actions = right_action.clone();
                    total_actions.push(action);
                    visited_right.insert(copy.clone(), total_actions);
                    queue_right.push_back(copy);
                }
            }
        }

        // eprintln!("{:?} & {:?}", queue_left.len(), queue_right.len());
    }

    Err(format!("Solution not found in {} moves", max_depth * 2))
}
