use crate::state::GameState;
use leptos::*;

const DAY_COLOR: &str = "yellow";
const NIGHT_COLOR: &str = "black";

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
    fn random() -> (Self, &'static str) {
        if fastrand::bool() {
            (Self::Day, DAY_COLOR)
        } else {
            (Self::Night, NIGHT_COLOR)
        }
    }

    fn next_state<'a, I>(my_state: &'a Self, neighs: I, w_color: WriteSignal<&'static str>) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        let neigh_num = neighs.map(|s| usize::from(matches!(s, Self::Day))).sum();

        match my_state {
            Self::Day => match neigh_num {
                0 | 1 | 2 | 5 => {
                    w_color.set(NIGHT_COLOR);
                    Self::Night
                }
                _ => Self::Day,
            },
            Self::Night => match neigh_num {
                3 | 6 | 7 | 8 => {
                    w_color.set(DAY_COLOR);
                    Self::Day
                }
                _ => Self::Night,
            },
        }
    }

    fn automaton_name() -> &'static str {
        "Bell's Day and Night automaton"
    }
}
