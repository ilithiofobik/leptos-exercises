use crate::state::GameState;

const LIGHTNING_PPB: f64 = 0.0001;
const REGROW_PPB: f64 = 0.001;

#[derive(Clone, Copy)]
pub enum WildFireState {
    Tree(usize),
    Dirt,
    Fire(usize),
    Lightning,
}

impl Default for WildFireState {
    fn default() -> Self {
        Self::Dirt
    }
}

impl GameState for WildFireState {
    fn random() -> Self {
        if fastrand::bool() {
            Self::Tree(3)
        } else {
            Self::Dirt
        }
    }

    fn next_state<I>(my_state: Self, mut neighs: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        match my_state {
            Self::Dirt => {
                if fastrand::f64() < REGROW_PPB {
                    Self::Tree(0)
                } else {
                    Self::Dirt
                }
            }
            Self::Tree(n) => {
                if fastrand::f64() < LIGHTNING_PPB {
                    Self::Lightning
                } else if neighs.any(|s| matches!(s, Self::Fire(_))) {
                    Self::Fire(10)
                } else {
                    Self::Tree(n + 1)
                } 
            }
            Self::Lightning => Self::Fire(10),
            Self::Fire(n) => {
                if n == 0 {
                    Self::Dirt
                } else {
                    Self::Fire(n - 1)
                }
            }
        }
    }

    fn to_color(&self) -> &'static str {
        match self {
            Self::Dirt => "black",
            Self::Tree(n) => 
                if *n < 5 {
                    "rgb(0,195,0)"
                } else if *n < 10 {
                    "rgb(20,215,20)"
                } else if *n < 15 {
                    "rgb(40,235,40)"
                } else {
                    "rgb(60,255,60)"
                }
            Self::Lightning => "white",
            Self::Fire(n) => 
                if *n > 10 {
                    "rgb(255,0,0)"
                } else if *n > 6 {
                    "rgb(235,0,0)"
                } else if *n > 3 {
                    "rgb(215,0,0)"
                } else {
                    "rgb(195,0,0)"
                }
        }
    }

    fn automaton_name() -> &'static str {
        "Wildfire simulation"
    }
}
