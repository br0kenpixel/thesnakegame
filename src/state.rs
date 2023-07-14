use crate::{consts::*, difficulties::Difficulty, moveobj::Move, set_pixel, tail::Tail};
use bracket_lib::prelude::*;
use rand::{prelude::*, rngs::OsRng};
use std::{
    process::exit,
    time::{Duration, Instant},
};

#[derive(Debug, PartialEq, Eq)]
pub enum State {
    MainScreen(MainScreenInfo),
    InGame(StateInfo),
    Paused(StateInfo),
    End(EndInfo),
}

pub enum GameEvent {
    End,
    Pause,
    Continue,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct MainScreenInfo {
    choice: u8,
    pub difficulty: Difficulty,
    last_key: Option<VirtualKeyCode>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StateInfo {
    start: Instant,
    snake_move_interval: u128,
    food_lifetime: u64,
    head_pos: Point,
    tail: Tail,
    food: Point,
    move_direction: Move,
    score: i8,
    last_move: Instant,
    food_spawn_time: Instant,
}

#[derive(Debug, PartialEq, Eq)]
pub struct EndInfo {
    final_score: i8,
    playtime: Duration,
}

impl StateInfo {
    pub fn new(difficulty: Difficulty) -> Self {
        Self {
            start: Instant::now(),
            snake_move_interval: difficulty.snake_move_interval(),
            food_lifetime: difficulty.food_lifetime(),
            head_pos: Point::new(5, 5),
            tail: Tail::new(0),
            food: Self::gen_new_food(),
            move_direction: Move::Right,
            score: 0,
            last_move: Instant::now(),
            food_spawn_time: Instant::now(),
        }
    }

    fn gen_new_food() -> Point {
        let line = FOOD_ROW_RANGE.choose(&mut OsRng).unwrap();
        let col = FOOD_COL_RANGE.choose(&mut OsRng).unwrap();

        Point::new(col, line)
    }

    fn move_player(&mut self, ctx: &mut BTerm) {
        if self.last_move.elapsed().as_millis() < self.snake_move_interval {
            return;
        }

        // Clear current position
        set_pixel!(ctx, self.head_pos.x, self.head_pos.y, GREEN, ' ');

        if self.tail.len() > 0 {
            self.tail.push(Point::new(self.head_pos.x, self.head_pos.y));
        }

        match self.move_direction {
            Move::Left => self.head_pos.x -= 1,
            Move::Right => self.head_pos.x += 1,
            Move::Up => self.head_pos.y -= 1,
            Move::Down => self.head_pos.y += 1,
        }

        set_pixel!(ctx, self.head_pos.x, self.head_pos.y, GREEN, '@');
        self.last_move = Instant::now();
    }

    fn check_food_expiry(&mut self, ctx: &mut BTerm) {
        if self.food_spawn_time.elapsed().as_secs() < self.food_lifetime {
            return;
        }

        set_pixel!(ctx, self.food.x, self.food.y, BLACK, ' ');

        self.score -= 2;
        self.food = Self::gen_new_food();
        self.food_spawn_time = Instant::now();
    }

    pub fn setup_screen(ctx: &mut BTerm) {
        ctx.cls();
        ctx.draw_box(
            0,
            0,
            WIDTH - 1,
            HEIGHT - 1,
            RGB::named(WHITE),
            RGB::named(BLACK),
        );
        ctx.print(WIDTH - SCORE_INDICATOR_OFFSET, 0, "[Score:     ]");
    }

    pub fn game_tick(&mut self, ctx: &mut BTerm) -> GameEvent {
        let input = INPUT.lock();

        if self.head_pos == self.food {
            self.score += 1;
            self.food = Self::gen_new_food();
            self.food_spawn_time = Instant::now();
            self.tail.inc_length();
        }

        if self.tail.contains(self.head_pos) {
            return GameEvent::End;
        }

        if [0, WIDTH - 1].contains(&self.head_pos.x) || [0, HEIGHT - 1].contains(&self.head_pos.y) {
            return GameEvent::End;
        }

        self.check_food_expiry(ctx);

        ctx.print(WIDTH - SCORE_INDICATOR_OFFSET + 8, 0, "    ");
        ctx.print(
            WIDTH - SCORE_INDICATOR_OFFSET + 8,
            0,
            format!("{}", self.score),
        );

        // Display food position
        set_pixel!(ctx, self.food.x, self.food.y, RED, 'o');

        if let Some(key) = input.key_pressed_set().iter().next() {
            match *key {
                VirtualKeyCode::W | VirtualKeyCode::Up if self.move_direction != Move::Down => {
                    self.move_direction = Move::Up;
                }
                VirtualKeyCode::A | VirtualKeyCode::Left if self.move_direction != Move::Right => {
                    self.move_direction = Move::Left;
                }
                VirtualKeyCode::S | VirtualKeyCode::Down if self.move_direction != Move::Up => {
                    self.move_direction = Move::Down;
                }
                VirtualKeyCode::D | VirtualKeyCode::Right if self.move_direction != Move::Left => {
                    self.move_direction = Move::Right;
                }
                VirtualKeyCode::Escape => return GameEvent::Pause,
                _ => (),
            }
        }

        if self.tail.len() > 0 {
            for point in self.tail.iter() {
                set_pixel!(ctx, point.x, point.y, BLACK, ' ');
            }
        }

        self.move_player(ctx);

        set_pixel!(ctx, self.head_pos.x, self.head_pos.y, GREEN, '@');

        if self.tail.len() > 0 {
            for point in self.tail.iter() {
                set_pixel!(ctx, point.x, point.y, GREEN, 'x');
            }
        }

        GameEvent::Continue
    }

    pub fn setup_paused_screen(ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_color_centered((HEIGHT / 2) - 1, ORANGE, BLACK, "Paused");
        ctx.print_centered(HEIGHT / 2, "Press [Enter] to resume game.");
    }

    pub fn pased_tick(&self) -> GameEvent {
        let input = INPUT.lock();

        if let Some(VirtualKeyCode::Return) = input.key_pressed_set().iter().next() {
            return GameEvent::Continue;
        }

        GameEvent::Pause
    }
}

impl MainScreenInfo {
    pub fn screen_tick(&mut self, ctx: &mut BTerm) -> bool {
        let input = INPUT.lock();
        ctx.cls();

        ctx.print_centered(
            (HEIGHT / 2) - 5,
            if self.choice == 0 { "> Play <" } else { "Play" },
        );

        ctx.print_centered(
            (HEIGHT / 2) + 5,
            format!("Difficulty: {}", {
                let mut display_name = self.difficulty.to_string();

                if self.choice == 1 {
                    display_name.insert_str(0, "< ");
                    display_name.push_str(" >");
                }

                display_name
            }),
        );
        ctx.print_centered(
            HEIGHT - 1,
            "Use [UP], [DOWN] and [ENTER] to change and set selection.",
        );

        let version_string = format!("v{}", env!("CARGO_PKG_VERSION"));
        ctx.print(WIDTH - version_string.len() as i32, 0, version_string);

        #[cfg(debug_assertions)]
        ctx.print(WIDTH - 5, 1, "DEBUG");

        for (i, line) in self.difficulty.description().split('\n').enumerate() {
            let offset = i as i32 * 2;
            ctx.print_centered((HEIGHT / 2) + 7 + offset, line);
        }

        if let Some(key) = input.key_pressed_set().iter().next() {
            if self.last_key.map_or(true, |last_key| last_key != *key) {
                match *key {
                    VirtualKeyCode::Left if self.choice == 1 => self.difficulty.decrease(),
                    VirtualKeyCode::Right if self.choice == 1 => self.difficulty.increase(),
                    VirtualKeyCode::Up if self.choice == 1 => self.choice = 0,
                    VirtualKeyCode::Down if self.choice == 0 => self.choice = 1,
                    VirtualKeyCode::Return if self.choice == 0 => {
                        self.last_key = None;
                        return true;
                    }
                    VirtualKeyCode::Escape => {
                        exit(0);
                    }
                    _ => (),
                }
                self.last_key = Some(*key);
            }
        } else if self.last_key.is_some() {
            self.last_key = None;
        }
        false
    }
}

impl EndInfo {
    pub fn screen_tick(&mut self, ctx: &mut BTerm) -> bool {
        let input = INPUT.lock();
        ctx.cls();

        ctx.print_color_centered(HEIGHT / 2, BLUE, BLACK, "GAME OVER");
        ctx.print_centered(
            (HEIGHT / 2) + 1,
            format!("Final score: {}", self.final_score),
        );
        ctx.print_centered(
            (HEIGHT / 2) + 2,
            format!("Playtime: {}", {
                let full_seconds = self.playtime.as_secs();
                let hours = full_seconds / 3600;
                let minutes = (full_seconds % 3600) / 60;
                let seconds = ((full_seconds % 3600) % 60) % 60;

                format!("{hours:02}:{minutes:02}:{seconds:02}")
            }),
        );

        ctx.print_centered(HEIGHT - 1, "Press [ENTER] to restart the game.");

        input.key_pressed_set().iter().next() == Some(&VirtualKeyCode::Return)
    }

    pub fn restart_game(state: &mut State) {
        let info = MainScreenInfo {
            last_key: Some(VirtualKeyCode::Return),
            ..Default::default()
        };

        *state = State::MainScreen(info);
    }
}

impl Default for State {
    fn default() -> Self {
        Self::MainScreen(MainScreenInfo::default())
    }
}

impl From<&mut StateInfo> for EndInfo {
    fn from(value: &mut StateInfo) -> Self {
        Self {
            final_score: value.score,
            playtime: value.start.elapsed(),
        }
    }
}
