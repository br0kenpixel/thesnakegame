#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Difficulty {
    Easy,
    #[default]
    Normal,
    Hard,
    Extreme,
    Extreme2,
}

pub struct DifficultyProps {
    pub snake_move_interval: u128, //ms
    pub food_lifetime: u64,        //sec
}

impl From<Difficulty> for DifficultyProps {
    fn from(value: Difficulty) -> Self {
        match value {
            Difficulty::Easy => Self {
                snake_move_interval: 35,
                food_lifetime: 15,
            },
            Difficulty::Normal => Self {
                snake_move_interval: 30,
                food_lifetime: 10,
            },
            Difficulty::Hard => Self {
                snake_move_interval: 25,
                food_lifetime: 8,
            },
            Difficulty::Extreme => Self {
                snake_move_interval: 18,
                food_lifetime: 7,
            },
            Difficulty::Extreme2 => Self {
                snake_move_interval: 12,
                food_lifetime: 8,
            },
        }
    }
}

impl Difficulty {
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
        let props: DifficultyProps = self.into();
        let mut result = format!(
            "Snake moves every {}ms and food lasts for {} seconds",
            props.snake_move_interval, props.food_lifetime
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
