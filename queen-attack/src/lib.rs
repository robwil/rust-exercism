#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        let valid_position = |n: i32| n >= 0 && n <= 7;
        if valid_position(rank) && valid_position(file) {
            Some(ChessPosition { rank, file })
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
        let is_diagonal = |q1: &Queen, q2: &Queen| {
            (q1.position.file - q2.position.file).abs()
                == (q1.position.rank - q2.position.rank).abs()
        };
        self.position.rank == other.position.rank
            || self.position.file == other.position.file
            || is_diagonal(self, other)
    }
}
