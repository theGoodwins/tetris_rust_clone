use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}

impl Difficulty {
    pub fn next(self) -> Difficulty {
        match self {
            Difficulty::Easy => Difficulty::Normal,
            Difficulty::Normal => Difficulty::Hard,
            Difficulty::Hard => Difficulty::Easy,
        }
    }
    pub fn prev(self) -> Difficulty {
        match self {
            Difficulty::Easy => Difficulty::Hard,
            Difficulty::Normal => Difficulty::Easy,
            Difficulty::Hard => Difficulty::Normal,
        }
    }
    pub fn as_str(self) -> &'static str {
        match self {
            Difficulty::Easy => "Easy",
            Difficulty::Normal => "Normal",
            Difficulty::Hard => "Hard",
        }
    }
}

#[derive(Clone, Copy)]
pub enum GameMode {
    Classic,
    Timed,
    Endless,
}

impl GameMode {
    pub fn next(self) -> GameMode {
        match self {
            GameMode::Classic => GameMode::Timed,
            GameMode::Timed => GameMode::Endless,
            GameMode::Endless => GameMode::Classic,
        }
    }
    pub fn prev(self) -> GameMode {
        match self {
            GameMode::Classic => GameMode::Endless,
            GameMode::Timed => GameMode::Classic,
            GameMode::Endless => GameMode::Timed,
        }
    }
    pub fn as_str(self) -> &'static str {
        match self {
            GameMode::Classic => "Classic",
            GameMode::Timed => "Timed",
            GameMode::Endless => "Endless",
        }
    }
}

pub struct MainMenu {
    pub selected_index: usize, // 0: Player Name, 1: Music, 2: Difficulty, 3: Game Mode, 4: Start Game
    pub player_name: String,
    pub music_index: usize,
    pub difficulty: Difficulty,
    pub game_mode: GameMode,
}

impl MainMenu {
    pub fn new() -> Self {
        Self {
            selected_index: 0,
            player_name: "Player".to_string(),
            music_index: 0,
            difficulty: Difficulty::Normal,
            game_mode: GameMode::Classic,
        }
    }

    /// Returns true if "Start Game" is activated.
    pub fn update(&mut self) -> bool {
        // Navigate menu options.
        if is_key_pressed(KeyCode::Up) {
            if self.selected_index == 0 {
                self.selected_index = 4;
            } else {
                self.selected_index -= 1;
            }
        }
        if is_key_pressed(KeyCode::Down) {
            self.selected_index = (self.selected_index + 1) % 5;
        }

        // For non-text fields, use left/right.
        if self.selected_index == 1 {
            if is_key_pressed(KeyCode::Left) {
                if self.music_index == 0 {
                    self.music_index = 2; // assuming 3 tracks (0, 1, 2)
                } else {
                    self.music_index -= 1;
                }
            }
            if is_key_pressed(KeyCode::Right) {
                self.music_index = (self.music_index + 1) % 3;
            }
        }
        if self.selected_index == 2 {
            if is_key_pressed(KeyCode::Left) {
                self.difficulty = self.difficulty.prev();
            }
            if is_key_pressed(KeyCode::Right) {
                self.difficulty = self.difficulty.next();
            }
        }
        if self.selected_index == 3 {
            if is_key_pressed(KeyCode::Left) {
                self.game_mode = self.game_mode.prev();
            }
            if is_key_pressed(KeyCode::Right) {
                self.game_mode = self.game_mode.next();
            }
        }
        // For Player Name, capture character input.
        if self.selected_index == 0 {
            if is_key_pressed(KeyCode::Backspace) {
                self.player_name.pop();
            }
            // Process all characters pressed this frame.
            while let Some(c) = get_char_pressed() {
                if c != '\u{8}' { // ignore backspace as char
                    if c.is_alphanumeric() || c == ' ' {
                        self.player_name.push(c);
                    }
                }
            }
        }
        // If "Start Game" is selected and Enter is pressed, return true.
        if self.selected_index == 4 && is_key_pressed(KeyCode::Enter) {
            return true;
        }
        false
    }

    pub fn draw(&self) {
        let start_x = screen_width() / 2.0 - 200.0;
        let mut start_y = screen_height() / 2.0 - 150.0;
        let spacing = 50.0;

        // Option 0: Player Name
        let player_text = format!("Player Name: {}", self.player_name);
        let color = if self.selected_index == 0 { YELLOW } else { WHITE };
        draw_text(&player_text, start_x, start_y, 30.0, color);
        start_y += spacing;

        // Option 1: Music Track
        let music_text = format!("Music Track: {}", self.music_index + 1);
        let color = if self.selected_index == 1 { YELLOW } else { WHITE };
        draw_text(&music_text, start_x, start_y, 30.0, color);
        start_y += spacing;

        // Option 2: Difficulty
        let diff_text = format!("Difficulty: {}", self.difficulty.as_str());
        let color = if self.selected_index == 2 { YELLOW } else { WHITE };
        draw_text(&diff_text, start_x, start_y, 30.0, color);
        start_y += spacing;

        // Option 3: Game Mode
        let mode_text = format!("Game Mode: {}", self.game_mode.as_str());
        let color = if self.selected_index == 3 { YELLOW } else { WHITE };
        draw_text(&mode_text, start_x, start_y, 30.0, color);
        start_y += spacing;

        // Option 4: Start Game
        let start_text = "Start Game";
        let color = if self.selected_index == 4 { YELLOW } else { WHITE };
        draw_text(start_text, start_x, start_y, 30.0, color);

        // Extra instructions for editing player name.
        if self.selected_index == 0 {
            draw_text("Type to change name. Backspace to delete.", start_x, start_y + 40.0, 20.0, GRAY);
        }
    }
}
