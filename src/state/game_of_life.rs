use crate::state::GameState;
use leptos::*;

const ALIVE_COLOR: &str = "red";
const DEAD_COLOR: &str = "white";

#[derive(Clone, Copy)]
pub enum GameOfLifeState {
    Alive,
    Dead,
}

impl Default for GameOfLifeState {
    fn default() -> Self {
        Self::Dead
    }
}

impl GameState for GameOfLifeState {
    fn random() -> Self {
        if fastrand::bool() {
            Self::Alive
        } else {
            Self::Dead
        }
    }

    fn next_state<'a, I>(my_state: &'a Self, neighs: I, w_color: WriteSignal<&'static str>) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        let neigh_num = neighs.map(|s| usize::from(matches!(s, Self::Alive))).sum();

        match my_state {
            Self::Alive => match neigh_num {
                2 | 3 => Self::Alive,
                _ => {
                    w_color.set(DEAD_COLOR);
                    Self::Dead
                }
            },
            Self::Dead => match neigh_num {
                3 => {
                    w_color.set(ALIVE_COLOR);
                    Self::Alive
                }
                _ => Self::Dead,
            },
        }
    }

    fn to_color(&self) -> &'static str {
        match self {
            Self::Alive => ALIVE_COLOR,
            Self::Dead => DEAD_COLOR,
        }
    }

    fn automaton_name() -> &'static str {
        "Conways's Game of Life"
    }
}
