use crate::state::IState;

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

impl IState for GameOfLifeState {
    fn random() -> Self {
        if fastrand::bool() {
            Self::Alive
        } else {
            Self::Dead
        }
    }

    fn next_state<I>(my_state: Self, neighs: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        let neigh_num = neighs.map(|s| usize::from(matches!(s, Self::Alive))).sum();

        match (neigh_num, my_state) {
            (3, _) | (2, Self::Alive) => Self::Alive,
            _ => Self::Dead,
        }
    }

    fn to_color(&self) -> &'static str {
        match self {
            Self::Alive => "red",
            Self::Dead => "white",
        }
    }

    fn automaton_name() -> &'static str {
        "Conways's Game of Life"
    }
}
