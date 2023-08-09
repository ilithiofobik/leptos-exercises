use chrono::{Datelike, Local};
use leptos::*;
use std::time::Duration;

const ROWS: usize = 40;
const COLS: usize = 120;
const NEIGHS: [(usize, usize); 8] = [
    (ROWS - 1, COLS - 1),
    (ROWS - 1, 0),
    (ROWS - 1, 1),
    (0, COLS - 1),
    (0, 1),
    (1, COLS - 1),
    (1, 0),
    (1, 1),
];

#[derive(Clone)]
enum InnerGrid {
    A,
    B,
}

#[derive(Clone)]
struct Grid<T> {
    inner_a: [[T; COLS]; ROWS],
    inner_b: [[T; COLS]; ROWS],
    current: InnerGrid,
}

trait IState {
    fn random() -> Self;
    fn next_state<I>(my_state: Self, neighs: I) -> Self
    where
        I: Iterator<Item = Self>;
    fn to_color(&self) -> &'static str;
}

#[derive(Clone, Copy)]
enum GOLState {
    Alive,
    Dead,
}

impl Default for GOLState {
    fn default() -> Self {
        Self::Dead
    }
}

impl IState for GOLState {
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
        let neigh_num = neighs
            .map(|s| matches!(s, Self::Alive))
            .map(usize::from)
            .sum();

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
}

impl<T: Default + Copy + IState> Grid<T> {
    pub fn random_grid() -> Grid<T> {
        let mut inner_a = [[T::default(); COLS]; ROWS];
        let mut inner_b = [[T::default(); COLS]; ROWS];
        let current = InnerGrid::A;

        for row in 0..ROWS {
            for col in 0..COLS {
                let rand_val = T::random();
                inner_a[row][col] = rand_val;
                inner_b[row][col] = rand_val;
            }
        }

        Grid {
            inner_a,
            inner_b,
            current,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> T {
        match self.current {
            InnerGrid::A => self.inner_a[row][col],
            InnerGrid::B => self.inner_b[row][col],
        }
    }

    pub fn set_prev(&mut self, row: usize, col: usize, value: T) {
        match self.current {
            InnerGrid::A => self.inner_b[row][col] = value,
            InnerGrid::B => self.inner_a[row][col] = value,
        }
    }

    pub fn change_curr(&mut self) {
        self.current = match self.current {
            InnerGrid::A => InnerGrid::B,
            InnerGrid::B => InnerGrid::A,
        }
    }

    pub fn game_move(&mut self) {
        for row in 0..ROWS {
            for col in 0..COLS {
                let my_state = self.get(row, col);
                let neighs = NEIGHS
                    .iter()
                    .map(|(mrow, mcol)| self.get((row + mrow) % ROWS, (col + mcol) % COLS));

                let new_state = T::next_state(my_state, neighs);

                self.set_prev(row, col, new_state);
            }
        }
        self.change_curr();
    }
}

#[component]
fn Footer(cx: Scope) -> impl IntoView {
    let year = Local::now().year();

    view! { cx,
        <footer>
            <p class="copyright">
                "©" { year } " Wojciech Sęk"
            </p>
        </footer>
    }
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (r_grid, w_grid) = create_signal(cx, Grid::<GOLState>::random_grid());

    let duration = Duration::from_millis(100);
    set_interval(
        move || {
            w_grid.update(|grid| grid.game_move());
        },
        duration,
    );

    let cell_divs = (0..ROWS)
        .map(|row| {
            (0..COLS)
                .map(|col| {
                    view! { cx,
                    <div
                        style:width = move || 20
                        style:height = move || 20
                        style:background-color=move || r_grid().get(row, col).to_color()
                        style:border = "1px solid #595959"
                        //background-color = move || "red"
                       // class:red=move || cell()
                    >
                    <br/>
                    </div>
                    }
                })
                .collect_view(cx)
        })
        .collect_view(cx);

    view! { cx,
        <h1>
            "CONWAY'S GAME OF LIFE"
        </h1>

        <div
            class:grid=true
        >
        { cell_divs }
        </div>

        <Footer/>
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
