use std::fmt;
use std::hash::{Hash, Hasher};

/// index 0 represent UP+
/// index 1 represent LEFT+
/// index 2 represent FRONT+
/// index 3 represent RIGHT+
/// index 4 represent DOWN+
/// index 5 represent BACK+
const ROTATION: [[(usize, usize); 20]; 2] = [
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
