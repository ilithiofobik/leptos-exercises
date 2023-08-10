use crate::state::GameState;

#[derive(Clone, Copy)]
pub enum DayNNightState {
    Day,
    Night,
}

impl Default for DayNNightState {
    fn default() -> Self {
        Self::Night
    }
}

impl GameState for DayNNightState {
    fn random() -> Self {
        if fastrand::bool() {
            Self::Day
        } else {
            Self::Night
        }
    }

    fn next_state<'a, I>(my_state: &'a Self, neighs: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        let neigh_num = neighs.map(|s| usize::from(matches!(s, Self::Day))).sum();

        match (neigh_num, my_state) {
            | (3, _)
            | (4, Self::Day) 
            | (6, _)
            | (7, _)
            | (8, _) => Self::Day,
            _ => Self::Night,
        }
    }

    fn to_color(&self) -> &'static str {
        match self {
            Self::Day => "yellow",
            Self::Night => "black",
        }
    }

    fn automaton_name() -> &'static str {
        "Bell's Day and Night automaton"
    }
}
