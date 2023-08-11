use crate::state::GameState;
use leptos::*;

pub const ROWS: usize = 40;
pub const COLS: usize = 120;
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

impl InnerGrid {
    fn flip(&mut self) {
        *self = match self {
            InnerGrid::A => InnerGrid::B,
            InnerGrid::B => InnerGrid::A,
        };
    }
}

#[derive(Clone)]
pub struct Grid<T> {
    inner_a: [[T; COLS]; ROWS],
    inner_b: [[T; COLS]; ROWS],
    current: InnerGrid,
    pub r_colors: Vec<Vec<ReadSignal<&'static str>>>,
    w_colors: Vec<Vec<WriteSignal<&'static str>>>,
}

impl<T: GameState> Grid<T> {
    pub fn random_grid(cx: Scope) -> Grid<T> {
        let mut inner_a = [[T::default(); COLS]; ROWS];
        let mut inner_b = [[T::default(); COLS]; ROWS];
        let current = InnerGrid::A;

        let mut r_colors = Vec::with_capacity(ROWS);
        let mut w_colors = Vec::with_capacity(ROWS);

        for row in 0..ROWS {
            let mut r_row = Vec::with_capacity(COLS);
            let mut w_row = Vec::with_capacity(COLS);

            for col in 0..COLS {
                let rand_val = T::random();
                inner_a[row][col] = rand_val;
                inner_b[row][col] = rand_val;

                let (r_color, w_color) = create_signal(cx, rand_val.to_color());
                r_row.push(r_color);
                w_row.push(w_color);
            }

            r_colors.push(r_row);
            w_colors.push(w_row);
        }

        Grid {
            inner_a,
            inner_b,
            current,
            r_colors,
            w_colors,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        match self.current {
            InnerGrid::A => &self.inner_a[row][col],
            InnerGrid::B => &self.inner_b[row][col],
        }
    }

    pub fn set_prev(&mut self, row: usize, col: usize, value: T) {
        match self.current {
            InnerGrid::A => self.inner_b[row][col] = value,
            InnerGrid::B => self.inner_a[row][col] = value,
        }
    }

    pub fn game_move(&mut self) {
        for row in 0..ROWS {
            for col in 0..COLS {
                let my_state = self.get(row, col);
                let neighs = NEIGHS
                    .iter()
                    .map(|(mrow, mcol)| self.get((row + mrow) % ROWS, (col + mcol) % COLS));

                let new_state = T::next_state(my_state, neighs, self.w_colors[row][col]);

                self.set_prev(row, col, new_state);
            }
        }
        self.current.flip();
    }
}
