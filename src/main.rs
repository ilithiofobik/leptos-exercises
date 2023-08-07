use std::fmt::Write;
use std::time::Duration;
use leptos::*;
use chrono::{Local, Datelike};
use fastrand;

const ROWS : usize = 20;
const COLS : usize = 30;

#[derive(Clone)]
enum InnerGrid {
    A,
    B
}

// impl InnerGrid {
//     pub fn to_color(&self) -> &str {
//         match self  {
//             InnerGrid::A => "red",
//             InnerGrid::B => "white"
//         }
//     }
//}

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

    pub fn set(&mut self, row: usize, col: usize, value: bool) {
        match self.current {
            InnerGrid::A => self.inner_a[row][col] = value,
            InnerGrid::B => self.inner_b[row][col] = value
        }
    }

    pub fn set_prev(&mut self, row: usize, col: usize, value: bool) {
        match self.current {
            InnerGrid::A => self.inner_b[row][col] = value,
            InnerGrid::B => self.inner_a[row][col] = value
        }
    }
    pub fn set_rev(&mut self, row: usize, col: usize) {
        let curr_array = 
            match self.current {
                InnerGrid::A => &mut self.inner_a,
                InnerGrid::B => &mut self.inner_b
            };
        curr_array[row][col] = !curr_array[row][col];
    }

    pub fn change_curr(&mut self) {
        self.current = 
            match self.current {
                InnerGrid::A => InnerGrid::B,
                InnerGrid::B => InnerGrid::A
            }
    }




    pub fn count_neighs(&self, row: usize, col: usize) -> usize {
        [
            (ROWS - 1, COLS - 1),
            (ROWS - 1, 0),
            (ROWS - 1, 1),
            (0, COLS - 1),
            (0, 1),
            (1, COLS - 1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .map(|(mrow, mcol)| usize::from(self.get((row + mrow) % ROWS, (col + mcol) % COLS)))
        .sum()
    }

    pub fn game_move(&mut self) {
        for row in 0..ROWS {
            for col in 0..COLS {
                let neigh_num = self.count_neighs(row, col);
                if self.get(row, col) {
                    match neigh_num {
                        | 2 | 3 => self.set_prev(row, col, true),
                        | _ => self.set_prev(row, col, false)
                    }
                } else {
                    match neigh_num {
                        | 3 => self.set_prev(row, col, true),
                        | _ => self.set_prev(row, col, false)
                    }
                }
            }
        }
        self.change_curr();
    }
}

#[derive(Debug, Clone)]
enum Color {
    White,
    Red
}

impl Color {
    pub fn to_reversed(color: &Color) -> Color {
        match color {
            Color::Red => Color::White,
            Color::White => Color::Red
        }
    }

    pub fn reverse(&mut self) {
        *self = Color::to_reversed(self) 
    }
}

impl IntoView for Color {
    fn into_view(self, cx: Scope) -> View {
        let color_str = format!("{:?}", self);
        color_str.into_view(cx)
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
    let mut r_cells = Vec::with_capacity(ROWS);
    let mut w_cells = Vec::with_capacity(ROWS);
    
    for _ in 0..ROWS {
        let mut r_row = Vec::with_capacity(COLS);
        let mut w_row = Vec::with_capacity(COLS);
        for _ in 0..COLS {
            let (r_cell, w_cell) = create_signal(cx, fastrand::bool());
            r_row.push(r_cell);
            w_row.push(w_cell);
        }
        r_cells.push(r_row);
        w_cells.push(w_row);
    }

    let (r_grid, w_grid) = create_signal(cx, Grid::random_grid());

    // let cells : Vec<Vec<_>> = 
    //     (0..ROWS)
    //     .map(|_| 
    //         (0..COLS)
    //         .map(|_| create_signal(cx, fastrand::bool()))
    //         .collect()
    //   //      .collect::Vec<(ReadSignal<bool>, WriteSignal<bool>)()
    //     ).collect(); //.collect::Vec<Vec<(ReadSignal<bool>, WriteSignal<bool>)>>();

    // for row in 0..ROWS {
    //     for col in 0..COLS {
    //         if ((cells[row][col]).0)() {
    //             log!("1");
    //         } else {
    //             log!("0");
    //         }
    //     }
    // }

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
        
    
    // create a list of N signals
    let counters = 

        (1..=10).map(|idx| create_signal(cx, idx));

    // each item manages a reactive view
    // but the list itself will never change
    let counter_buttons = 
        counters
        .map(|(count, set_count)| {
            view! { cx,
                <li>
                    <button
                        on:click=move |_| set_count.update(|n| *n += 1)
                    >
                        {count}
                    </button>
                </li>
            }
        })
        .collect_view(cx);

    let color = create_rw_signal(cx, Color::White);
    let next_color = move || Color::to_reversed(&color());

    let s = "";

    view! { cx,
        <button
            on:click=move |_| {
                color.update(|c| c.reverse() );
            }
            class:red=move || matches!(color(), Color::Red)
        >
            "Click me: " { color }
        </button>
        
        <p>
            "Next color: " { next_color }
        </p>

        { s }

        <div
            class:grid=move || true
        >
        { cell_divs }
        </div>

        <ul>{counter_buttons}</ul>

        <Footer/>
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}