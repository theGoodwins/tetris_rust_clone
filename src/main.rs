use macroquad::prelude::*;
use ::rand::{thread_rng, Rng};
use std::cmp::{min, max};

use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use rodio::source::{Source};

const GRID_WIDTH: usize = 10;
const GRID_HEIGHT: usize = 20;
const TILE_SIZE: f32 = 30.0;
const PREVIEW_TILE_SIZE: f32 = 25.0;

const FALL_SPEED: f32 = 3.0;
const SOFT_DROP_SPEED: f32 = 15.0;
const INITIAL_HORIZONTAL_DELAY: f32 = 0.2;
const HORIZONTAL_REPEAT_DELAY: f32 = 0.1;

const GAME_AREA_COLOR: Color = Color::new(0.2, 0.2, 0.2, 1.0);
const BLACK_COLOR: Color = BLACK;
const GOLD_COLOR: Color = Color::new(1.0, 0.84, 0.0, 1.0);
const SILVER_COLOR: Color = Color::new(0.75, 0.75, 0.75, 1.0);

const GOLD_POINTS: u32 = 500;
const SILVER_POINTS: u32 = 200;

const NES_COLORS: [Color; 7] = [
    Color { r: 0.0,    g: 1.0,    b: 1.0,    a: 1.0 }, // I
    Color { r: 1.0,    g: 1.0,    b: 0.0,    a: 1.0 }, // O
    Color { r: 0.6667, g: 0.0,    b: 1.0,    a: 1.0 }, // T
    Color { r: 0.0,    g: 1.0,    b: 0.0,    a: 1.0 }, // S
    Color { r: 1.0,    g: 0.0,    b: 0.0,    a: 1.0 }, // Z
    Color { r: 0.0,    g: 0.0,    b: 1.0,    a: 1.0 }, // J
    Color { r: 1.0,    g: 0.3334, b: 0.0,    a: 1.0 }, // L
];

const MUSIC_LIST: [&str; 3] = [
    "resources/music/music-a-gb.mp3",
    "resources/music/music-a.mp3",
    "resources/music/music-b.mp3",
];

struct MusicManager {
    mus_stream:OutputStream,
    mus_stream_hndl:OutputStreamHandle,
    mus_sink:Sink,
    mus_track:u32,
    muted:bool,
    paused:bool,
}

impl MusicManager {
    fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        MusicManager {
            mus_stream:_stream,
            mus_stream_hndl:stream_handle,
            mus_sink:sink,
            mus_track:0,
            muted:false,
            paused:false,
        }
    }

    pub fn play_song(&mut self){
        // Clear the current Sink's buffer
        self.mus_sink.clear();
        // Grab new sound file
        let file = BufReader::new(File::open(MUSIC_LIST[(self.mus_track%(MUSIC_LIST.len() as u32)) as usize]).unwrap());
        // Increase the index of track for next time we call this
        self.mus_track += 1;
        // Decode that sound file into a source
        let source = Decoder::new(file).unwrap().repeat_infinite();
        // Append the source into the buffer
        self.mus_sink.append(source);
        self.mus_sink.play();
    }

    pub fn mute(&mut self){
        if self.muted{
            self.mus_sink.set_volume(1.0);
        }
        else{
            self.mus_sink.set_volume(0.0);
        }
        self.muted = !self.muted;
    }

    pub fn pause(&mut self){
        if self.paused{
            self.mus_sink.play();
        }
        else{
            self.mus_sink.pause();
        }
        self.paused = !self.paused;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum TetrominoType {
    I, O, T, S, Z, J, L,
    BonusGold, BonusSilver, // For bonus blocks.
}

const TETROMINO_SHAPES: [[[i32; 2]; 4]; 7] = [
    [[0,0],[1,0],[2,0],[3,0]],    // I
    [[0,0],[1,0],[0,1],[1,1]],    // O
    [[1,0],[0,1],[1,1],[2,1]],    // T
    [[1,0],[2,0],[0,1],[1,1]],    // S
    [[0,0],[1,0],[1,1],[2,1]],    // Z
    [[0,0],[0,1],[1,1],[2,1]],    // J
    [[0,0],[1,0],[2,0],[0,1]],    // L
];

const TETROMINO_ROTATION_OFFSETS: [[i32; 2]; 7] = [
    [1,0], // I
    [0,0], // O (doesn't rotate)
    [1,1], // T
    [1,1], // S
    [1,1], // Z
    [1,1], // J
    [1,1], // L
];

#[derive(Clone, Copy)]
struct Tetromino {
    shape: [[i32; 2]; 4],
    pos: (i32, i32),
    color: Color,
    t_type: TetrominoType,
}

impl Tetromino {
    fn new(t_type: TetrominoType) -> Self {
        Tetromino {
            shape: TETROMINO_SHAPES[t_type as usize],
            pos: (GRID_WIDTH as i32 / 2 - 2, 0),
            color: NES_COLORS[t_type as usize],
            t_type,
        }
    }
}

fn rotate_shape(shape: &[[i32; 2]; 4], t_type: TetrominoType, clockwise: bool) -> [[i32; 2]; 4] {
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

struct SquareEffect {
    x: usize,
    y: usize,
    is_gold: bool,
    timer: f32,             // Duration per blink phase.
    flash_on: bool,         // Whether bonus color is displayed.
    blinks_remaining: u32,  // Number of on-off cycles remaining.
    original: [[(Color, TetrominoType, u32); 4]; 4],
}

struct GameState {
    // Each cell stores Option<(Color, TetrominoType, piece_id)>
    board: [[Option<(Color, TetrominoType, u32)>; GRID_WIDTH]; GRID_HEIGHT],
    tetromino: Option<Tetromino>,
    next_tetromino: Option<Tetromino>,
    hold_tetromino: Option<Tetromino>,
    hold_used: bool,

    started: bool,
    paused: bool,
    game_over: bool,
    lines_cleared: u32,
    score: u32,

    left_timer: f32,
    right_timer: f32,
    fall_timer: f32,

    line_clear_timer: f32,
    clearing_lines: Vec<usize>,

    active_squares: Vec<SquareEffect>,

    next_piece_id: u32, // For unique locked piece tagging.

    mus_mgr: MusicManager,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            board: [[None; GRID_WIDTH]; GRID_HEIGHT],
            tetromino: None,
            next_tetromino: None,
            hold_tetromino: None,
            hold_used: false,
            started: false,
            paused: false,
            game_over: false,
            lines_cleared: 0,
            score: 0,
            left_timer: 0.0,
            right_timer: 0.0,
            fall_timer: 0.0,
            line_clear_timer: 0.0,
            clearing_lines: Vec::new(),
            active_squares: Vec::new(),
            next_piece_id: 1,
            mus_mgr: MusicManager::new(),
        }
    }

    pub fn start_game(&mut self) {
        self.started = true;
        self.game_over = false;
        self.paused = false;
        self.lines_cleared = 0;
        self.score = 0;
        self.board = [[None; GRID_WIDTH]; GRID_HEIGHT];
        self.hold_tetromino = None;
        self.hold_used = false;
        self.line_clear_timer = 0.0;
        self.clearing_lines.clear();
        self.active_squares.clear();
        self.next_piece_id = 1;

        let mut rng = thread_rng();
        let curr_type = match rng.gen_range(0..7) {
            0 => TetrominoType::I,
            1 => TetrominoType::O,
            2 => TetrominoType::T,
            3 => TetrominoType::S,
            4 => TetrominoType::Z,
            5 => TetrominoType::J,
            _ => TetrominoType::L,
        };
        let next_type = match rng.gen_range(0..7) {
            0 => TetrominoType::I,
            1 => TetrominoType::O,
            2 => TetrominoType::T,
            3 => TetrominoType::S,
            4 => TetrominoType::Z,
            5 => TetrominoType::J,
            _ => TetrominoType::L,
        };

        self.tetromino = Some(Tetromino::new(curr_type));
        self.next_tetromino = Some(Tetromino::new(next_type));
        self.mus_mgr.play_song();
    }

    pub fn check_collision(&self, shape: &[[i32; 2]; 4], pos: (i32, i32)) -> bool {
        for &[dx, dy] in shape {
            let x = pos.0 + dx;
            let y = pos.1 + dy;
            if x < 0 || x >= GRID_WIDTH as i32 || y < 0 || y >= GRID_HEIGHT as i32 {
                return true;
            }
            if self.board[y as usize][x as usize].is_some() {
                return true;
            }
        }
        false
    }

    pub fn lock_tetromino(&mut self) {
        if let Some(tetro) = self.tetromino {
            let id = self.next_piece_id;
            self.next_piece_id += 1;
            for &[dx, dy] in &tetro.shape {
                let x = tetro.pos.0 + dx;
                let y = tetro.pos.1 + dy;
                if x >= 0 && x < GRID_WIDTH as i32 && y >= 0 && y < GRID_HEIGHT as i32 {
                    self.board[y as usize][x as usize] = Some((tetro.color, tetro.t_type, id));
                }
            }
        }
        let mut full_rows = Vec::new();
        for (i, row) in self.board.iter().enumerate() {
            if row.iter().all(|cell| cell.is_some()) {
                full_rows.push(i);
            }
        }
        if !full_rows.is_empty() {
            self.clearing_lines = full_rows;
            self.line_clear_timer = 0.27;
        } else {
            self.spawn_new_tetromino();
            self.check_for_4x4_squares();
        }
    }

    pub fn clear_lines_delayed(&mut self) {
        let mut new_board: Vec<[Option<(Color, TetrominoType, u32)>; GRID_WIDTH]> = Vec::new();
        for (i, row) in self.board.iter().enumerate() {
            if self.clearing_lines.contains(&i) { continue; }
            new_board.push(*row);
        }
        while new_board.len() < GRID_HEIGHT {
            new_board.insert(0, [None; GRID_WIDTH]);
        }
        self.board = new_board.try_into().unwrap();
        self.lines_cleared += self.clearing_lines.len() as u32;
        self.clearing_lines.clear();

        if let Some(next) = self.next_tetromino {
            if self.check_collision(&next.shape, next.pos) {
                self.game_over = true;
                self.started = false;
                return;
            }
        }
        self.spawn_new_tetromino();
        self.check_for_4x4_squares();
    }

    pub fn spawn_new_tetromino(&mut self) {
        if !self.started { return; }
        if let Some(next_t) = self.next_tetromino {
            if self.check_collision(&next_t.shape, next_t.pos) {
                self.game_over = true;
                self.started = false;
            } else {
                self.tetromino = Some(next_t);
                let mut rng = thread_rng();
                let t_type = match rng.gen_range(0..7) {
                    0 => TetrominoType::I,
                    1 => TetrominoType::O,
                    2 => TetrominoType::T,
                    3 => TetrominoType::S,
                    4 => TetrominoType::Z,
                    5 => TetrominoType::J,
                    _ => TetrominoType::L,
                };
                self.next_tetromino = Some(Tetromino::new(t_type));
                self.hold_used = false;
                self.fall_timer = 0.0;
            }
        }
    }

    // --- Square Detection ---
    // Only triggers when every cell in a 4x4 candidate is full (and not bonus) and for every piece present,
    // all its locked cells lie entirely within the candidate.
    pub fn check_for_4x4_squares(&mut self) {
        for y in 0..(GRID_HEIGHT - 3) {
            for x in 0..(GRID_WIDTH - 3) {
                let mut all_filled = true;
                let mut original: [[(Color, TetrominoType, u32); 4]; 4] = [[(BLACK_COLOR, TetrominoType::I, 0); 4]; 4];
                for dy in 0..4 {
                    for dx in 0..4 {
                        if let Some(cell) = self.board[y + dy][x + dx] {
                            if cell.1 == TetrominoType::BonusGold || cell.1 == TetrominoType::BonusSilver {
                                all_filled = false;
                                break;
                            }
                            original[dy][dx] = cell;
                        } else {
                            all_filled = false;
                            break;
                        }
                    }
                    if !all_filled { break; }
                }
                if !all_filled { continue; }
                let mut pieces_in_region = vec![];
                for row in &original {
                    for &(_, _t, id) in row {
                        if !pieces_in_region.contains(&id) {
                            pieces_in_region.push(id);
                        }
                    }
                }
                let mut candidate_valid = true;
                for &pid in &pieces_in_region {
                    for row in 0..GRID_HEIGHT {
                        for col in 0..GRID_WIDTH {
                            if let Some((_col, _t, id)) = self.board[row][col] {
                                if id == pid {
                                    if col < x || col >= x + 4 || row < y || row >= y + 4 {
                                        candidate_valid = false;
                                        break;
                                    }
                                }
                            }
                        }
                        if !candidate_valid { break; }
                    }
                    if !candidate_valid { break; }
                }
                if !candidate_valid { continue; }
                let mut types = vec![];
                for &pid in &pieces_in_region {
                    'outer: for dy in 0..4 {
                        for dx in 0..4 {
                            if original[dy][dx].2 == pid {
                                types.push(original[dy][dx].1);
                                break 'outer;
                            }
                        }
                    }
                }
                let all_same = types.iter().all(|&t| t == types[0]);
                if self.active_squares.iter().any(|eff| eff.x == x && eff.y == y) {
                    continue;
                }
                self.active_squares.push(SquareEffect {
                    x,
                    y,
                    is_gold: all_same,
                    timer: 0.3,
                    flash_on: true,
                    blinks_remaining: 6,
                    original: original,
                });
            }
        }
    }

    pub fn update_square_effects(&mut self, dt: f32) {
        self.active_squares.retain_mut(|eff| {
            eff.timer -= dt;
            if eff.timer <= 0.0 {
                eff.timer = 0.3;
                eff.flash_on = !eff.flash_on;
                if !eff.flash_on && eff.blinks_remaining > 0 {
                    eff.blinks_remaining -= 1;
                }
            }
            if eff.blinks_remaining == 0 {
                let bonus_type = if eff.is_gold { TetrominoType::BonusGold } else { TetrominoType::BonusSilver };
                let square_color = if eff.is_gold { GOLD_COLOR } else { SILVER_COLOR };
                for dy in 0..4 {
                    for dx in 0..4 {
                        self.board[eff.y + dy][eff.x + dx] = Some((square_color, bonus_type, 0));
                    }
                }
                self.score += if eff.is_gold { GOLD_POINTS } else { SILVER_POINTS };
                false
            } else {
                true
            }
        });
    }

    pub fn process_input(&mut self, delta: f32) {
        // Hard Drop: We use a separate block to avoid mutable/immutable borrow conflict.
        if is_key_pressed(KeyCode::Up) {
            loop {
                let can_move_down = {
                    if let Some(ref t) = self.tetromino {
                        !self.check_collision(&t.shape, (t.pos.0, t.pos.1 + 1))
                    } else {
                        false
                    }
                };
                if !can_move_down { break; }
                if let Some(t) = self.tetromino.as_mut() {
                    t.pos.1 += 1;
                }
            }
            self.lock_tetromino();
            return;
        }

        // For other inputs, we can use a local copy.
        let curr = self.tetromino.unwrap();
        if is_key_pressed(KeyCode::Left) {
            if !self.check_collision(&curr.shape, (curr.pos.0 - 1, curr.pos.1)) {
                self.move_tetromino((-1, 0));
                self.left_timer = INITIAL_HORIZONTAL_DELAY;
            }
        } else if is_key_down(KeyCode::Left) {
            self.left_timer -= delta;
            if self.left_timer <= 0.0 {
                if !self.check_collision(&curr.shape, (curr.pos.0 - 1, curr.pos.1)) {
                    self.move_tetromino((-1, 0));
                    self.left_timer = HORIZONTAL_REPEAT_DELAY;
                }
            }
        } else {
            self.left_timer = 0.0;
        }

        if is_key_pressed(KeyCode::Right) {
            if !self.check_collision(&curr.shape, (curr.pos.0 + 1, curr.pos.1)) {
                self.move_tetromino((1, 0));
                self.right_timer = INITIAL_HORIZONTAL_DELAY;
            }
        } else if is_key_down(KeyCode::Right) {
            self.right_timer -= delta;
            if self.right_timer <= 0.0 {
                if !self.check_collision(&curr.shape, (curr.pos.0 + 1, curr.pos.1)) {
                    self.move_tetromino((1, 0));
                    self.right_timer = HORIZONTAL_REPEAT_DELAY;
                }
            }
        } else {
            self.right_timer = 0.0;
        }

        if is_key_pressed(KeyCode::Z) {
            let new_shape = rotate_shape(&curr.shape, curr.t_type, false);
            if !self.check_collision(&new_shape, curr.pos) {
                self.set_tetromino_shape(new_shape);
            }
        }
        if is_key_pressed(KeyCode::X) {
            let new_shape = rotate_shape(&curr.shape, curr.t_type, true);
            if !self.check_collision(&new_shape, curr.pos) {
                self.set_tetromino_shape(new_shape);
            }
        }

        if is_key_down(KeyCode::Down) {
            self.fall_timer = 0.0;
            if !self.check_collision(&curr.shape, (curr.pos.0, curr.pos.1 + 1)) {
                self.move_tetromino((0, 1));
            }
        }

        if is_key_pressed(KeyCode::M) {
            self.mus_mgr.mute();
        }

        if is_key_pressed(KeyCode::N) {
            self.mus_mgr.play_song();
        }

        if is_key_pressed(KeyCode::C) && !self.hold_used {
            self.hold_used = true;
            let mut current_piece = curr;
            current_piece.shape = TETROMINO_SHAPES[current_piece.t_type as usize];
            if let Some(mut hold_piece) = self.hold_tetromino.take() {
                hold_piece.shape = TETROMINO_SHAPES[hold_piece.t_type as usize];
                hold_piece.pos = (GRID_WIDTH as i32 / 2 - 2, 0);
                if self.check_collision(&hold_piece.shape, hold_piece.pos) {
                    self.hold_tetromino = Some(hold_piece);
                } else {
                    self.hold_tetromino = Some(current_piece);
                    self.tetromino = Some(hold_piece);
                }
            } else {
                self.hold_tetromino = Some(current_piece);
                self.tetromino = None;
                self.spawn_new_tetromino();
            }
        }
    }

    pub fn move_tetromino(&mut self, (dx, dy): (i32, i32)) {
        if let Some(mut t) = self.tetromino {
            t.pos = (t.pos.0 + dx, t.pos.1 + dy);
            self.tetromino = Some(t);
        }
    }

    pub fn set_tetromino_shape(&mut self, shape: [[i32; 2]; 4]) {
        if let Some(mut t) = self.tetromino {
            t.shape = shape;
            self.tetromino = Some(t);
        }
    }

    pub fn update(&mut self) {
        let dt = get_frame_time();
        if !self.game_over && is_key_pressed(KeyCode::Enter) {
            self.paused = !self.paused;
            self.mus_mgr.pause();
        }
        if self.paused || !self.started || self.game_over {
            return;
        }
        if self.line_clear_timer > 0.0 {
            self.line_clear_timer -= dt;
            if self.line_clear_timer <= 0.0 {
                self.clear_lines_delayed();
            }
            return;
        }
        self.process_input(dt);
        if let Some(curr) = self.tetromino {
            let speed = if is_key_down(KeyCode::Down) { SOFT_DROP_SPEED } else { FALL_SPEED };
            let fall_interval = 1.0 / speed;
            self.fall_timer += dt;
            if self.fall_timer >= fall_interval {
                self.fall_timer -= fall_interval;
                if self.check_collision(&curr.shape, (curr.pos.0, curr.pos.1 + 1)) {
                    self.lock_tetromino();
                } else {
                    self.move_tetromino((0, 1));
                }
            }
        }
        self.update_square_effects(dt);
    }

    pub fn draw(&self) {
        clear_background(BLACK_COLOR);
        if !self.started {
            let msg = "Press SPACE to start";
            let measure = measure_text(msg, None, 40, 1.0);
            let x = (screen_width() - measure.width) / 2.0;
            let y = (screen_height() - measure.height) / 2.0;
            draw_text(msg, x, y, 40.0, YELLOW);
            return;
        }
        let board_w = GRID_WIDTH as f32 * TILE_SIZE;
        let board_h = GRID_HEIGHT as f32 * TILE_SIZE;
        let offset_x = (screen_width() - board_w) / 2.0;
        let offset_y = (screen_height() - board_h) / 2.0 - 50.0;
        draw_rectangle(offset_x, offset_y, board_w, board_h, GAME_AREA_COLOR);
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                if let Some((color, _t, _id)) = self.board[y][x] {
                    let mut draw_color = color;
                    for eff in &self.active_squares {
                        if x >= eff.x && x < eff.x + 4 && y >= eff.y && y < eff.y + 4 {
                            let rel_x = x - eff.x;
                            let rel_y = y - eff.y;
                            draw_color = if eff.flash_on {
                                if eff.is_gold { GOLD_COLOR } else { SILVER_COLOR }
                            } else {
                                eff.original[rel_y][rel_x].0
                            };
                            break;
                        }
                    }
                    let px = offset_x + x as f32 * TILE_SIZE;
                    let py = offset_y + y as f32 * TILE_SIZE;
                    draw_snes_block(px, py, TILE_SIZE, draw_color);
                }
            }
        }
        if let Some(curr) = self.tetromino {
            let mut ghost = curr;
            let mut iter = 0;
            while !self.check_collision(&ghost.shape, (ghost.pos.0, ghost.pos.1 + 1)) && iter < 100 {
                ghost.pos.1 += 1;
                iter += 1;
            }
            let ghost_color = Color::new(curr.color.r, curr.color.g, curr.color.b, 0.3);
            for &[dx, dy] in &ghost.shape {
                let x = ghost.pos.0 + dx;
                let y = ghost.pos.1 + dy;
                let px = offset_x + x as f32 * TILE_SIZE;
                let py = offset_y + y as f32 * TILE_SIZE;
                draw_rectangle(px, py, TILE_SIZE, TILE_SIZE, ghost_color);
            }
            for &[dx, dy] in &curr.shape {
                let x = curr.pos.0 + dx;
                let y = curr.pos.1 + dy;
                let px = offset_x + x as f32 * TILE_SIZE;
                let py = offset_y + y as f32 * TILE_SIZE;
                draw_snes_block(px, py, TILE_SIZE, curr.color);
            }
        }
        draw_rectangle(offset_x, offset_y, board_w, TILE_SIZE * 2.0, BLACK_COLOR);
        if self.line_clear_timer > 0.0 {
            let frames = (self.line_clear_timer * 60.0) as i32;
            let flash_on = frames % 2 == 0;
            let flash_color = if flash_on { WHITE } else { BLACK_COLOR };
            for &row in &self.clearing_lines {
                let py = offset_y + row as f32 * TILE_SIZE;
                draw_rectangle(offset_x, py, board_w, TILE_SIZE, flash_color);
            }
        }
        draw_text(&format!("Lines: {}", self.lines_cleared), screen_width() - 210.0, 170.0, 40.0, WHITE);
        draw_text(&format!("Score: {}", self.score), screen_width() - 210.0, 220.0, 40.0, WHITE);
        if self.game_over {
            let msg = "Game Over";
            let measure = measure_text(msg, None, 50, 1.0);
            let x = offset_x + (board_w - measure.width) / 2.0;
            let y = offset_y + board_h / 2.0;
            draw_text(msg, x, y, 50.0, RED);
        }
        if self.paused {
            draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0,0.0,0.0,0.6));
            let msg = "Paused";
            let measure = measure_text(msg, None, 50, 1.0);
            draw_text(msg, (screen_width()-measure.width)/2.0, screen_height()/2.0, 50.0, YELLOW);
        }
        draw_text("Hold", 79.0, 55.0, 40.0, WHITE);
        if let Some(ref hold_piece) = self.hold_tetromino {
            draw_preview(hold_piece, 79.0, 90.0, PREVIEW_TILE_SIZE);
        }
        draw_text("Next", screen_width() - 210.0, 55.0, 40.0, WHITE);
        if let Some(ref next_piece) = self.next_tetromino {
            draw_preview(next_piece, screen_width() - 218.0, 70.0, PREVIEW_TILE_SIZE);
        }
        let controls_text = "\
Controls:
 Left/Right: Move
 Up: Hard Drop
 Down: Soft Drop
 Z/X: Rotate
 C: Hold
 Enter: Pause
 Space: Start
 N: Change Song
 M: Mute Music";
        let text_x = 20.0;
        let text_y = offset_y + board_h + 80.0;
        let wrapped = wrap_text(controls_text, screen_width() - 40.0, 24);
        draw_text_ex(&wrapped, text_x, text_y, TextParams {
            font: None,
            font_size: 24,
            font_scale: 1.0,
            font_scale_aspect: 1.0,
            rotation: 0.0,
            color: WHITE,
        });
    }
}

fn wrap_text(text: &str, max_width: f32, font_size: u16) -> String {
    let mut result = String::new();
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut current_line = String::new();
    for word in words {
        let test_line = if current_line.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current_line, word)
        };
        let metrics = measure_text(&test_line, None, font_size, 1.0);
        if metrics.width > max_width && !current_line.is_empty() {
            if !result.is_empty() { result.push('\n'); }
            result.push_str(&current_line);
            current_line = word.to_string();
        } else {
            current_line = test_line;
        }
    }
    if !current_line.is_empty() {
        if !result.is_empty() { result.push('\n'); }
        result.push_str(&current_line);
    }
    result
}

fn draw_snes_block(x: f32, y: f32, size: f32, color: Color) {
    draw_rectangle(x, y, size, size, color);
    let highlight = Color::new(
        (color.r + 0.4).min(1.0),
        (color.g + 0.4).min(1.0),
        (color.b + 0.4).min(1.0),
        1.0,
    );
    let shadow = Color::new(
        (color.r * 0.5).max(0.0),
        (color.g * 0.5).max(0.0),
        (color.b * 0.5).max(0.0),
        1.0,
    );
    let border = size * 0.15;
    draw_rectangle(x, y, size, border, highlight);
    draw_rectangle(x, y, border, size, highlight);
    draw_rectangle(x, y + size - border, size, border, shadow);
    draw_rectangle(x + size - border, y, border, size, shadow);
}

fn draw_preview(tetromino: &Tetromino, pos_x: f32, pos_y: f32, tile_size: f32) {
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;
    for &[bx, by] in tetromino.shape.iter() {
        min_x = min(min_x, bx);
        min_y = min(min_y, by);
        max_x = max(max_x, bx);
        max_y = max(max_y, by);
    }
    let shape_w = (max_x - min_x + 1) as f32 * tile_size;
    let shape_h = (max_y - min_y + 1) as f32 * tile_size;
    let offset_x = pos_x + (50.0 - shape_w) / 2.0;
    let offset_y = pos_y + (50.0 - shape_h) / 2.0;
    for &[bx, by] in tetromino.shape.iter() {
        let draw_x = offset_x + (bx - min_x) as f32 * tile_size;
        let draw_y = offset_y + (by - min_y) as f32 * tile_size;
        draw_snes_block(draw_x, draw_y, tile_size, tetromino.color);
    }
}

#[macroquad::main("Tetris")]
async fn main() {
    // Optionally, uncomment the following if you need to set the window size:
    // request_new_screen_size(1000.0, 800.0);
    let mut game_state = GameState::new();

    loop {
        if is_key_pressed(KeyCode::Space) && !game_state.started {
            game_state.start_game();
        }
        game_state.update();
        game_state.draw();
        next_frame().await;
    }
}
