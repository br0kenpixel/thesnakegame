#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Difficulty {
    Easy,
    #[default]
    Normal,
    Hard,
    Extreme,
    Extreme2,
}

impl Difficulty {
    pub const fn snake_move_interval(self) -> u128 {
        match self {
            Self::Easy => 55,
            Self::Normal => 40,
            Self::Hard => 35,
            Self::Extreme => 20,
            Self::Extreme2 => 16,
        }
    }

    pub const fn food_lifetime(self) -> u64 {
        match self {
            Self::Easy => 20,
            Self::Normal => 10,
            Self::Hard | Self::Extreme2 => 8,
            Self::Extreme => 7,
        }
    }

    pub fn increase(&mut self) {
        *self = match self {
            Self::Easy => Self::Normal,
            Self::Normal => Self::Hard,
            Self::Hard => Self::Extreme,
            Self::Extreme | Self::Extreme2 => Self::Extreme2,
        }
    }

    pub fn decrease(&mut self) {
        *self = match self {
            Self::Extreme2 => Self::Extreme,
            Self::Extreme => Self::Hard,
            Self::Hard => Self::Normal,
            Self::Normal | Self::Easy => Self::Easy,
        }
    }

    pub fn description(self) -> String {
        let mut result = format!(
            "Snake moves every {}ms and food lasts for {} seconds",
            self.snake_move_interval(),
            self.food_lifetime()
        );

        if self == Self::Extreme2 {
            result.insert_str(0, "Why the fuck would you even want this?\n");
        }

        result
    }
}

impl ToString for Difficulty {
    fn to_string(&self) -> String {
        String::from(match self {
            Self::Easy => "Easy",
            Self::Normal => "Normal",
            Self::Hard => "Hard",
            Self::Extreme => "Extreme",
            Self::Extreme2 => "HONDA CIVIC", // just for the memes :D
        })
    }
}
