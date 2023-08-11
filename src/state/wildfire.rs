use crate::state::GameState;
use crate::utils::rand::geo;
use leptos::*;

const LIGHTNING_PPB: f64 = 0.0001;
const REGROW_PPB: f64 = 0.001;

const FIRE_COLOR1: &str = "rgb(165,0,0)";
const FIRE_COLOR2: &str = "rgb(195,0,0)";
const FIRE_COLOR3: &str = "rgb(225,0,0)";
const FIRE_COLOR4: &str = "rgb(255,0,0)";

const FIRE_STEP1: usize = 6;
const FIRE_STEP2: usize = 2 * FIRE_STEP1;
const FIRE_STEP3: usize = 3 * FIRE_STEP1;
const FIRE_STEP4: usize = 4 * FIRE_STEP1;

const TREE_COLOR1: &str = "rgb(0,165,0)";
const TREE_COLOR2: &str = "rgb(20,295,20)";
const TREE_COLOR3: &str = "rgb(40,225,40)";
const TREE_COLOR4: &str = "rgb(60,255,60)";

const TREE_STEP1: usize = 10;
const TREE_STEP2: usize = 2 * TREE_STEP1;
const TREE_STEP3: usize = 3 * TREE_STEP1;
const TREE_STEP4: usize = 4 * TREE_STEP1;

const DIRT_COLOR: &str = "black";
const LIGHT_COLOR: &str = "white";

#[derive(Clone, Copy)]
pub enum WildFireState {
    Tree(usize, usize),
    Dirt(usize),
    Fire(usize),
    Lightning,
}

impl Default for WildFireState {
    fn default() -> Self {
        Self::Dirt(10000)
    }
}

impl GameState for WildFireState {
    fn random() -> (Self, &'static str) {
        (Self::Dirt(geo(REGROW_PPB)), DIRT_COLOR)
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
            Self::Dirt(0) => {
                w_color.set(TREE_COLOR1);
                Self::Tree(TREE_STEP1, geo(LIGHTNING_PPB))
            }
            Self::Dirt(n) => Self::Dirt(n - 1),

            Self::Tree(_, 0) => {
                w_color.set(LIGHT_COLOR);
                Self::Lightning
            }

            Self::Tree(n, k) => {
                if neighs.any(|s| matches!(s, Self::Fire(_))) {
                    w_color.set(FIRE_COLOR4);
                    Self::Fire(FIRE_STEP4)
                } else {
                    match *n {
                        TREE_STEP2 => w_color.set(TREE_COLOR2),
                        TREE_STEP3 => w_color.set(TREE_COLOR3),
                        TREE_STEP4 => w_color.set(TREE_COLOR4),
                        _ => (),
                    }
                    Self::Tree(n + 1, k - 1)
                }
            }

            Self::Lightning => {
                w_color.set(FIRE_COLOR4);
                Self::Fire(FIRE_STEP4)
            }

            Self::Fire(0) => {
                w_color.set(DIRT_COLOR);
                Self::Dirt(geo(REGROW_PPB))
            }
            Self::Fire(n) => {
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

    fn automaton_name() -> &'static str {
        "Wildfire simulation"
    }
}
