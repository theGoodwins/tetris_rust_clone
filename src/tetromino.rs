use macroquad::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TetrominoType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
    BonusGold,
    BonusSilver,
}

pub const TETROMINO_SHAPES: [[[i32; 2]; 4]; 7] = [
    [[0, 0], [1, 0], [2, 0], [3, 0]], // I
    [[0, 0], [1, 0], [0, 1], [1, 1]], // O
    [[1, 0], [0, 1], [1, 1], [2, 1]], // T
    [[1, 0], [2, 0], [0, 1], [1, 1]], // S
    [[0, 0], [1, 0], [1, 1], [2, 1]], // Z
    [[0, 0], [0, 1], [1, 1], [2, 1]], // J
    [[0, 0], [1, 0], [2, 0], [0, 1]], // L
];

pub const TETROMINO_ROTATION_OFFSETS: [[i32; 2]; 7] = [
    [1, 0], // I
    [0, 0], // O (doesn't rotate)
    [1, 1], // T
    [1, 1], // S
    [1, 1], // Z
    [1, 1], // J
    [1, 1], // L
];

pub const NES_COLORS: [Color; 7] = [
    Color { r: 0.0,    g: 1.0,    b: 1.0,    a: 1.0 }, // I
    Color { r: 1.0,    g: 1.0,    b: 0.0,    a: 1.0 }, // O
    Color { r: 0.6667, g: 0.0,    b: 1.0,    a: 1.0 }, // T
    Color { r: 0.0,    g: 1.0,    b: 0.0,    a: 1.0 }, // S
    Color { r: 1.0,    g: 0.0,    b: 0.0,    a: 1.0 }, // Z
    Color { r: 0.0,    g: 0.0,    b: 1.0,    a: 1.0 }, // J
    Color { r: 1.0,    g: 0.3334, b: 0.0,    a: 1.0 }, // L
];

#[derive(Clone, Copy)]
pub struct Tetromino {
    pub shape: [[i32; 2]; 4],
    pub pos: (i32, i32),
    pub color: Color,
    pub t_type: TetrominoType,
}

impl Tetromino {
    pub fn new(t_type: TetrominoType) -> Self {
        Tetromino {
            shape: TETROMINO_SHAPES[t_type as usize],
            pos: (10 / 2 - 2, 0),
            color: NES_COLORS[t_type as usize],
            t_type,
        }
    }
}

pub fn rotate_shape(shape: &[[i32; 2]; 4], t_type: TetrominoType, clockwise: bool) -> [[i32; 2]; 4] {
    let mut new_shape = [[0; 2]; 4];
    let [pivot_x, pivot_y] = TETROMINO_ROTATION_OFFSETS[t_type as usize];
    for (i, &[x, y]) in shape.iter().enumerate() {
        let rel_x = x - pivot_x;
        let rel_y = y - pivot_y;
        let (nx, ny) = if clockwise {
            (pivot_x + rel_y, pivot_y - rel_x)
        } else {
            (pivot_x - rel_y, pivot_y + rel_x)
        };
        new_shape[i] = [nx, ny];
    }
    new_shape
}
