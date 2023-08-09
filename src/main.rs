use std::time::Duration;
use leptos::*;
use chrono::{Local, Datelike};
use fastrand;

const ROWS : usize = 40;
const COLS : usize = 120;
const NEIGHS : [(usize, usize); 8] = 
    [
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
enum InnerGrid { A, B }

#[derive(Clone)]
struct Grid {
    inner_a : [[bool; COLS]; ROWS],
    inner_b : [[bool; COLS]; ROWS],
    current : InnerGrid
}

impl Grid {
    pub fn random_grid() -> Grid {
        let mut inner_a = [[false; COLS]; ROWS];
        let mut inner_b = [[false; COLS]; ROWS];
        let current = InnerGrid::A;

        for row in 0..ROWS {
            for col in 0..COLS {
                let rand_bool = fastrand::bool();
                inner_a[row][col] = rand_bool;
                inner_b[row][col] = rand_bool;
            }
        }

        Grid {
            inner_a,
            inner_b,
            current
        }
    }

    pub fn get(&self, row: usize, col: usize) -> bool {
        match self.current {
            InnerGrid::A => self.inner_a[row][col],
            InnerGrid::B => self.inner_b[row][col]
        }
    }

    pub fn set_prev(&mut self, row: usize, col: usize, value: bool) {
        match self.current {
            InnerGrid::A => self.inner_b[row][col] = value,
            InnerGrid::B => self.inner_a[row][col] = value
        }
    }

    pub fn change_curr(&mut self) {
        self.current = 
            match self.current {
                InnerGrid::A => InnerGrid::B,
                InnerGrid::B => InnerGrid::A
            }
    }

    pub fn count_neighs(&self, row: usize, col: usize) -> usize {
        NEIGHS
        .iter()
        .map(|(mrow, mcol)| usize::from(self.get((row + mrow) % ROWS, (col + mcol) % COLS)))
        .sum()
    }

    pub fn game_move(&mut self) {
        for row in 0..ROWS {
            for col in 0..COLS {
                let neigh_num = self.count_neighs(row, col);
                let new_state =
                    (neigh_num == 3) || (neigh_num == 2 && self.get(row, col));
                self.set_prev(row, col, new_state);
            }
        }
        self.change_curr();
    }
}

#[component]
fn Footer(cx: Scope) -> impl IntoView{ 
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

    let (r_grid, w_grid) = create_signal(cx, Grid::random_grid());

    let duration = Duration::from_millis(100);
    set_interval(
        move || {
            w_grid.update(|grid| grid.game_move());
        }, 
        duration);

    let cell_divs =
        (0..ROWS).into_iter().map(|row| 
            (0..COLS).map(
                |col| {
                    view! { cx,     
                    <div
                        style:width = move || 20
                        style:height = move || 20
                        style:background-color=move || if (r_grid()).get(row, col) { "red" } else { "white" }
                        style:border = "1px solid #595959"
                        //background-color = move || "red"
                       // class:red=move || cell()
                    >
                    <br/>
                    </div>                    
                    }
                }
            )
            .collect_view(cx)
        )
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