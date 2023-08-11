pub mod day_n_night;
pub mod game_of_life;
pub mod wildfire;
use leptos::*;

pub trait GameState: Default + Copy {
    fn random() -> Self;
    fn next_state<'a, I>(my_state: &'a Self, neighs: I, w_color: WriteSignal<&'static str>) -> Self
    where
        Self: 'a,
        I: Iterator<Item = &'a Self>;
    fn to_color(&self) -> &'static str;
    fn automaton_name() -> &'static str;
}
