use crate::state::GameState;
use leptos::*;

const LIGHTNING_PPB: f64 = 0.0001;
const REGROW_PPB: f64 = 0.001;

const FIRE_COLOR1: &str = "rgb(195,0,0)";
const FIRE_COLOR2: &str = "rgb(215,0,0)";
const FIRE_COLOR3: &str = "rgb(235,0,0)";
const FIRE_COLOR4: &str = "rgb(255,0,0)";

const FIRE_STEP1: usize = 6;
const FIRE_STEP2: usize = 2 * FIRE_STEP1;
const FIRE_STEP3: usize = 3 * FIRE_STEP1;
const FIRE_STEP4: usize = 4 * FIRE_STEP1;

const TREE_COLOR1: &str = "rgb(0,195,0)";
const TREE_COLOR2: &str = "rgb(20,215,20)";
const TREE_COLOR3: &str = "rgb(40,235,40)";
const TREE_COLOR4: &str = "rgb(60,255,60)";

const TREE_STEP1: usize = 10;
const TREE_STEP2: usize = 2 * TREE_STEP1;
const TREE_STEP3: usize = 3 * TREE_STEP1;
const TREE_STEP4: usize = 4 * TREE_STEP1;

const DIRT_COLOR: &str = "black";
const LIGHT_COLOR: &str = "white";

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
            Self::Tree(TREE_STEP1)
        } else {
            Self::Dirt
        }
    }

    fn next_state<'a, I>(
        my_state: &'a Self,
        mut neighs: I,
        w_color: WriteSignal<&'static str>,
    ) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        match my_state {
            Self::Dirt => {
                if fastrand::f64() < REGROW_PPB {
                    w_color.set(TREE_COLOR1);
                    Self::Tree(TREE_STEP1)
                } else {
                    Self::Dirt
                }
            }
            Self::Tree(n) => {
                if fastrand::f64() < LIGHTNING_PPB {
                    w_color.set(LIGHT_COLOR);
                    Self::Lightning
                } else if neighs.any(|s| matches!(s, Self::Fire(_))) {
                    w_color.set(FIRE_COLOR4);
                    Self::Fire(FIRE_STEP4)
                } else {
                    match *n {
                        TREE_STEP2 => w_color.set(TREE_COLOR2),
                        TREE_STEP3 => w_color.set(TREE_COLOR3),
                        TREE_STEP4 => w_color.set(TREE_COLOR4),
                        _ => (),
                    }
                    Self::Tree(n + 1)
                }
            }
            Self::Lightning => {
                w_color.set(FIRE_COLOR4);
                Self::Fire(FIRE_STEP4)
            }
            Self::Fire(n) => {
                if *n == 0 {
                    w_color.set(DIRT_COLOR);
                    Self::Dirt
                } else {
                    match *n {
                        FIRE_STEP3 => w_color.set(FIRE_COLOR3),
                        FIRE_STEP2 => w_color.set(FIRE_COLOR2),
                        FIRE_STEP1 => w_color.set(FIRE_COLOR1),
                        _ => (),
                    }
                    Self::Fire(n - 1)
                }
            }
        }
    }

    fn to_color(&self) -> &'static str {
        match self {
            Self::Dirt => DIRT_COLOR,
            Self::Tree(_) => TREE_COLOR1,
            Self::Lightning => LIGHT_COLOR,
            Self::Fire(_) => FIRE_COLOR1,
        }
    }

    fn automaton_name() -> &'static str {
        "Wildfire simulation"
    }
}
