#![allow(
    clippy::wildcard_imports,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::module_name_repetitions
)]
mod consts;
mod difficulties;
mod macros;
mod moveobj;
mod state;
mod tail;

use bracket_lib::prelude::*;
use consts::*;
use state::*;

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self {
            Self::MainScreen(inner_state) => {
                if inner_state.screen_tick(ctx) {
                    *self = Self::InGame(StateInfo::new(inner_state.difficulty));
                    StateInfo::setup_screen(ctx);
                }
            }
            Self::InGame(inner_state) => match inner_state.game_tick(ctx) {
                GameEvent::End => {
                    let end_stats: EndInfo = inner_state.into();
                    *self = Self::End(end_stats);
                }
                GameEvent::Pause => {
                    StateInfo::setup_paused_screen(ctx);
                    *self = Self::Paused(inner_state.clone());
                }
                GameEvent::Continue => (),
            },
            Self::Paused(inner_state) => {
                if let GameEvent::Continue = inner_state.pased_tick() {
                    StateInfo::setup_screen(ctx);
                    *self = Self::InGame(inner_state.clone());
                }
            }
            Self::End(inner_state) => inner_state.screen_tick(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple(WIDTH, HEIGHT)?
        .with_title("Snake")
        .with_fps_cap(FPS_LIMIT)
        .build()?;

    let state = State::default();
    main_loop(context, state)
}
