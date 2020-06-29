use std::fmt;

#[derive(Debug)]
pub struct ChessPosition {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

struct AttackTiles {
    mask: [[bool; 8]; 8],
}

impl fmt::Display for AttackTiles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.mask.iter() {
            for tile in row.iter() {
                let symbol = if *tile { 'x' } else { 'o' };
                write!(f, " {} ", symbol)?;
            }
            write!(f, "\n ")?;
        }
        Ok(())
    }
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank >= 0 && rank < 8 && file >= 0 && file < 8 {
            Some(ChessPosition { x: rank, y: file })
        } else {
            None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let attack_tiles = self.create_mask();

        attack_tiles.mask[other.position.y as usize][other.position.x as usize]
    }

    fn create_mask(&self) -> AttackTiles {
        let mut mask = [[false; 8]; 8];
        let queen_x = self.position.x as usize;
        let queen_y = self.position.y as usize;

        // row and column
        for i in 0..8 {
            mask[i][queen_x] = true;
            mask[queen_y][i] = true;
        }

        // left-up diagonal
        let mut x = queen_x;
        let mut y = queen_y;
        while x > 0 && y > 0 {
            x -= 1;
            y -= 1;
            mask[y][x] = true;
        }

        // left-down diagonal
        x = queen_x;
        y = queen_y;
        while x > 0 && y < 7 {
            x -= 1;
            y += 1;
            mask[y][x] = true;
        }

        // right-up diagonal
        let mut x = queen_x;
        let mut y = queen_y;
        while x < 7 && y > 0 {
            x += 1;
            y -= 1;
            mask[y][x] = true;
        }

        // right-down diagonal
        let mut x = queen_x;
        let mut y = queen_y;
        while x < 7 && y < 7 {
            x += 1;
            y += 1;
            mask[y][x] = true;
        }

        AttackTiles { mask }
    }
}