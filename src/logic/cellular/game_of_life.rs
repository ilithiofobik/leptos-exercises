use crate::logic::cellular::state::GameState;
use leptos::*;

const ALIVE_COLOR: &str = "#b62203";
const DEAD_COLOR: &str = "#F5F5DC";

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
    fn random() -> (Self, &'static str) {
        if fastrand::bool() {
            (Self::Alive, ALIVE_COLOR)
        } else {
            (Self::Dead, DEAD_COLOR)
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

    fn automaton_name() -> &'static str {
        "Conways's Game of Life"
    }
}
