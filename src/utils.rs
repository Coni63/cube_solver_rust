use crate::cube::Cube;
use rand::Rng;
use std::hash::{DefaultHasher, Hash, Hasher};

pub fn shuffle(cube: &mut Cube, n: usize) {
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        let action: u8 = rng.gen_range(0..12);
        cube.rotate(action);
    }
}

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
