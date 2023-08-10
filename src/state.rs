pub mod game_of_life;
pub mod wildfire;

pub trait GameState: Default + Copy {
    fn random() -> Self;
    fn next_state<I>(my_state: Self, neighs: I) -> Self
    where
        I: Iterator<Item = Self>;
    fn to_color(&self) -> &'static str;
    fn automaton_name() -> &'static str;
}
