use std::collections::{HashMap, VecDeque};

use crate::{cube::Cube, utils};

pub fn solve(cube: &Cube) -> Vec<u8> {
    let cube1 = cube.clone();
    let cube2 = Cube::new();
    meet_in_middle(cube1, cube2)
}

fn invert_action(action: u8) -> u8 {
    if action % 2 == 1 {
        action - 1
    } else {
        action + 1
    }
}

fn meet_in_middle(left_cube: Cube, right_cube: Cube) -> Vec<u8> {
    if right_cube == left_cube {
        return vec![];
    }

    let mut visited_left: HashMap<u64, Vec<u8>> = HashMap::new();
    visited_left.insert(utils::calculate_hash(&left_cube), vec![]);

    let mut visited_right: HashMap<u64, Vec<u8>> = HashMap::new();
    visited_right.insert(utils::calculate_hash(&right_cube), vec![]);

    let mut queue_left: VecDeque<Cube> = VecDeque::new();
    queue_left.push_back(left_cube);

    let mut queue_right: VecDeque<Cube> = VecDeque::new();
    queue_right.push_back(right_cube);

    // let mut iteration = 0;
    loop {
        if let Some(cube) = queue_left.pop_front() {
            let current_hash = utils::calculate_hash(&cube);
            let left_action_start = visited_left.get(&current_hash).unwrap().clone();
            for action in 0..12 {
                let mut copy = cube.clone();
                copy.rotate(action);
                let copy_hash = utils::calculate_hash(&copy);

                let mut left_action = left_action_start.clone();
                left_action.push(action);

                if visited_right.contains_key(&copy_hash) {
                    let right_action = visited_right.get(&copy_hash).unwrap().to_owned();
                    left_action.extend(right_action.iter().rev().map(|&x| invert_action(x)));
                    return left_action;
                }

                if !visited_left.contains_key(&copy_hash) {
                    visited_left.insert(copy_hash, left_action);
                    queue_left.push_back(copy);
                }
            }
        }

        if let Some(cube) = queue_right.pop_front() {
            let current_hash = utils::calculate_hash(&cube);
            let right_action_start = visited_right.get(&current_hash).unwrap().clone();
            for action in 0..12 {
                let mut copy = cube.clone();
                copy.rotate(action);
                let copy_hash = utils::calculate_hash(&copy);

                let mut right_action = right_action_start.clone();
                right_action.push(action);

                if visited_left.contains_key(&copy_hash) {
                    let mut left_action = visited_left.get(&copy_hash).unwrap().to_owned();
                    left_action.extend(right_action.iter().rev().map(|&x| invert_action(x)));
                    return left_action;
                }

                if !visited_right.contains_key(&copy_hash) {
                    visited_right.insert(copy_hash, right_action);
                    queue_right.push_back(copy);
                }
            }
        }
    }
}
