mod cube;
mod solver;
mod utils;

use cube::Cube;

fn main() {
    let cube = Cube::new();
    let mut cube_copy = cube.clone();

    eprintln!("{:?}", cube);
    // eprintln!("{:?}", utils::calculate_hash(&cube));

    utils::shuffle(&mut cube_copy, 2);

    // eprintln!("{:?}", cube_copy);
    // eprintln!("{:?}", utils::calculate_hash(&cube_copy));

    match solver::solve(&cube_copy) {
        Ok(solution) => eprintln!("{:?}", solution),
        Err(s) => eprintln!("{}", s),
    }

    // cube.rotate(0);
    // eprintln!("{:?}", cube);

    // cube.rotate(1);
    // eprintln!("{:?}", cube);

    // cube.rotate(2);
    // eprintln!("{:?}", cube);

    // cube.rotate(3);
    // eprintln!("{:?}", cube);
}
