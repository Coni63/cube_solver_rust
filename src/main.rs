mod cube;
mod solver;
mod utils;

use std::time::Instant;

use cube::Cube;

fn main() {
    let cube = Cube::new();
    let mut cube_copy = cube.clone();

    utils::shuffle(&mut cube_copy, 50);

    let timer = Instant::now();
    let solution = solver::solve(&cube_copy);
    eprintln!("{:?}", solution);
    eprintln!("{:?}", timer.elapsed());
}
