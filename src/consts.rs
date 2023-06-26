use std::ops::Range;

pub const WIDTH: i32 = 100;
pub const HEIGHT: i32 = 56;
pub const FPS_LIMIT: f32 = 60.0;
pub const SCORE_INDICATOR_OFFSET: i32 = 15;

pub const FOOD_COL_RANGE: Range<i32> = 2..(WIDTH - 1);
pub const FOOD_ROW_RANGE: Range<i32> = 2..(HEIGHT - 1);
