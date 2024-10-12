use std::fmt;
use std::hash::{Hash, Hasher};

/// index 0 represent UP+
/// index 1 represent LEFT+
/// index 2 represent FRONT+
/// index 3 represent RIGHT+
/// index 4 represent DOWN+
/// index 5 represent BACK+
const ROTATION: [[(usize, usize); 20]; 6] = [
    [
        (6, 0),
        (0, 2),
        (2, 8),
        (8, 6),
        (3, 1),
        (1, 5),
        (5, 7),
        (7, 3),
        (18, 9),
        (19, 10),
        (20, 11),
        (27, 18),
        (28, 19),
        (29, 20),
        (9, 53),
        (10, 52),
        (11, 51),
        (53, 27),
        (52, 28),
        (51, 29),
    ],
    [
        (11, 9),
        (9, 15),
        (15, 17),
        (17, 11),
        (10, 12),
        (12, 16),
        (16, 14),
        (14, 10),
        (18, 0),
        (21, 3),
        (24, 6),
        (0, 45),
        (3, 48),
        (6, 51),
        (45, 36),
        (48, 39),
        (51, 42),
        (36, 18),
        (39, 21),
        (42, 24),
    ],
    [
        (19, 23),
        (23, 25),
        (25, 21),
        (21, 19),
        (18, 20),
        (20, 26),
        (26, 24),
        (24, 18),
        (11, 8),
        (8, 33),
        (33, 36),
        (36, 11),
        (14, 7),
        (7, 30),
        (30, 37),
        (37, 14),
        (17, 6),
        (6, 27),
        (27, 38),
        (38, 17),
    ],
    [
        (30, 28),
        (28, 32),
        (32, 34),
        (34, 30),
        (27, 29),
        (29, 35),
        (35, 33),
        (33, 27),
        (20, 2),
        (2, 47),
        (47, 38),
        (38, 20),
        (23, 5),
        (5, 50),
        (50, 41),
        (41, 23),
        (26, 8),
        (8, 53),
        (53, 44),
        (44, 26),
    ],
    [
        (37, 41),
        (41, 43),
        (43, 39),
        (39, 37),
        (36, 38),
        (38, 44),
        (44, 42),
        (42, 36),
        (15, 24),
        (24, 33),
        (33, 47),
        (47, 15),
        (16, 25),
        (25, 34),
        (34, 46),
        (46, 16),
        (17, 26),
        (26, 35),
        (35, 45),
        (45, 17),
    ],
    [
        (46, 48),
        (48, 52),
        (52, 50),
        (50, 46),
        (45, 51),
        (51, 53),
        (53, 47),
        (47, 45),
        (0, 29),
        (29, 44),
        (44, 15),
        (15, 0),
        (1, 32),
        (32, 43),
        (43, 12),
        (12, 1),
        (2, 35),
        (35, 42),
        (42, 9),
        (9, 2),
    ],
];

/// index 0-8 represent the Up face (U).
/// index 9-17 represent the Left face (L).
/// index 18-26 represent the Front face (F).
/// index 27-35 represent the Right face (R).
/// index 36-44 represent the Down face (D).
/// index 45-53 represent the Back face (B).
#[derive(Clone, PartialEq, Eq)]
pub struct Cube {
    facets: [u8; 54], // Each face sticker is represented as an index from 0 to 53
}

impl Cube {
    pub fn new() -> Cube {
        let mut facets = [0u8; 54];

        facets
            .iter_mut()
            .enumerate()
            .skip(9)
            .for_each(|(i, facet)| {
                *facet = (i / 9) as u8;
            });

        Cube { facets }
    }

    pub fn rotate(&mut self, action: u8) {
        let mut tmp = self.facets;
        if action % 2 == 0 {
            for (from, to) in ROTATION[(action / 2) as usize].iter() {
                tmp[*to] = self.facets[*from];
            }
        } else {
            for (to, from) in ROTATION[(action / 2) as usize].iter() {
                tmp[*to] = self.facets[*from];
            }
        }
        self.facets = tmp;
    }
}

impl fmt::Debug for Cube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "   {}{}{}   ",
            self.facets[0], self.facets[1], self.facets[2]
        )?;
        writeln!(
            f,
            "   {}{}{}   ",
            self.facets[3], self.facets[4], self.facets[5]
        )?;
        writeln!(
            f,
            "   {}{}{}   ",
            self.facets[6], self.facets[7], self.facets[8]
        )?;
        writeln!(
            f,
            "{}{}{}{}{}{}{}{}{}",
            self.facets[9],
            self.facets[10],
            self.facets[11],
            self.facets[18],
            self.facets[19],
            self.facets[20],
            self.facets[27],
            self.facets[28],
            self.facets[29]
        )?;
        writeln!(
            f,
            "{}{}{}{}{}{}{}{}{}",
            self.facets[12],
            self.facets[13],
            self.facets[14],
            self.facets[21],
            self.facets[22],
            self.facets[23],
            self.facets[30],
            self.facets[31],
            self.facets[32]
        )?;
        writeln!(
            f,
            "{}{}{}{}{}{}{}{}{}",
            self.facets[15],
            self.facets[16],
            self.facets[17],
            self.facets[24],
            self.facets[25],
            self.facets[26],
            self.facets[33],
            self.facets[34],
            self.facets[35]
        )?;
        writeln!(
            f,
            "   {}{}{}   ",
            self.facets[36], self.facets[37], self.facets[38]
        )?;
        writeln!(
            f,
            "   {}{}{}   ",
            self.facets[39], self.facets[40], self.facets[41]
        )?;
        writeln!(
            f,
            "   {}{}{}   ",
            self.facets[42], self.facets[43], self.facets[44]
        )?;
        writeln!(
            f,
            "   {}{}{}   ",
            self.facets[45], self.facets[46], self.facets[47]
        )?;
        writeln!(
            f,
            "   {}{}{}   ",
            self.facets[48], self.facets[49], self.facets[50]
        )?;
        writeln!(
            f,
            "   {}{}{}   ",
            self.facets[51], self.facets[52], self.facets[53]
        )?;
        Ok(())
    }
}

impl Hash for Cube {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.facets.hash(state);
    }
}

impl From<&str> for Cube {
    fn from(seq: &str) -> Cube {
        // Split the string by commas and map each part to a parsed u8 value
        let arr: Vec<u8> = seq
            .chars()
            .map(|x| x.to_digit(10).expect("Invalid value"))
            .map(|x| x as u8)
            .collect();

        // Check if the number of values is exactly 54
        if arr.len() != 54 {
            panic!(
                "Cannot convert the given string to a cube. It requires 54 values, but found {}",
                arr.len()
            );
        }

        // Check if any value is outside the allowed range [0, 5]
        if arr.iter().any(|&x| x > 5) {
            panic!("Some values are not accepted as colors. Each value must be between 0 and 5.");
        }

        // Convert the Vec<u8> into an array [u8; 54]
        let facets: [u8; 54] = arr
            .try_into()
            .expect("Conversion to fixed-size array failed");

        Cube { facets }
    }
}

impl Into<String> for Cube {
    fn into(self) -> String {
        self.facets
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("")
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, hash::DefaultHasher};

    use super::*;

    #[test]
    fn test_into() {
        let cube = Cube::new();

        let s: String = cube.into();

        assert_eq!(
            s,
            String::from("000000000111111111222222222333333333444444444555555555")
        );
    }

    #[test]
    fn test_from() {
        let s = String::from("000555000111111111222222222333333333444444444555000555");
        let cube = Cube::from(s.as_str());

        assert_eq!(cube.facets[4], 5u8);
        assert_eq!(cube.facets[48], 0u8);
    }

    #[test]
    fn test_hash() {
        let mut cube = Cube::new();

        let mut s = DefaultHasher::new();
        cube.hash(&mut s);
        assert_eq!(s.finish(), 1795212828290158527);

        cube.rotate(2);

        let mut s = DefaultHasher::new();
        cube.hash(&mut s);
        assert_ne!(s.finish(), 1795212828290158527);
    }

    #[test]
    fn test_hash_2() {
        let mut visited: HashMap<Cube, bool> = HashMap::new();
        let mut cube = Cube::new();
        cube.rotate(2);

        visited.insert(cube, true);

        let mut cube2 = Cube::new();
        cube2.rotate(2);

        assert!(visited.contains_key(&cube2));

        let mut cube3 = Cube::new();
        cube3.rotate(3);

        assert!(!visited.contains_key(&cube3));
    }

    #[test]
    fn test_rotation() {
        let mut count = [0u8; 54];

        for r in ROTATION {
            for (from, to) in r {
                count[from] += 1;
                count[to] += 1;
            }
        }

        let expected = [6, 4, 6, 4, 0, 4, 6, 4, 6];

        for i in 0..6 {
            let start = (i * 9) as usize;
            let end = start + 9;
            let face = &count[start..end];
            assert!(face == expected)
        }
    }
}
