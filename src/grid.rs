use crate::state::GameState;

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

#[derive(Clone)]
pub struct Grid<T> {
    inner_a: [[T; COLS]; ROWS],
    inner_b: [[T; COLS]; ROWS],
    current: InnerGrid,
}

impl<T: GameState> Grid<T> {
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
