mod cube;

use cube::Cube;

fn main() {
    let mut cube = Cube::new();

    eprintln!("{:?}", cube);

    // cube.rotate(0);
    // eprintln!("{:?}", cube);

    // cube.rotate(1);
    // eprintln!("{:?}", cube);

    cube.rotate(2);
    eprintln!("{:?}", cube);

    cube.rotate(3);
    eprintln!("{:?}", cube);
}
